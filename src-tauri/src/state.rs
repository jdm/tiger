use squeak::{Delegate, Observable};
use std::path::{Path, PathBuf};
use thiserror::Error;

use crate::document::{ClipboardManifest, Document, DocumentError};
use crate::utils::handle;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("The requested document (`{0}`) is not currently opened.")]
    DocumentNotFound(PathBuf),
    #[error(transparent)]
    DocumentError(#[from] DocumentError),
}

pub type Handle = handle::Handle<State>;

#[derive(Debug, Default)]
pub struct State {
    documents: Vec<Document>,
    current_document: Option<PathBuf>,
    recent_documents: Observable<'static, Vec<PathBuf>>,
    clipboard_manifest: Option<ClipboardManifest>,
    errors: Vec<UserFacingError>,
    exit_requested: bool,
}

#[derive(Debug)]
pub struct UserFacingError {
    pub key: uuid::Uuid,
    pub title: String,
    pub summary: String,
    pub details: String,
}

impl State {
    pub fn documents_iter(&self) -> impl Iterator<Item = &Document> {
        self.documents.iter()
    }

    pub fn documents_iter_mut(&mut self) -> impl Iterator<Item = &mut Document> {
        self.documents.iter_mut()
    }

    pub fn new_document<T: AsRef<Path>>(&mut self, path: T) {
        match self.document_mut(&path) {
            Some(d) => *d = Document::new(path.as_ref()),
            None => {
                let document = Document::new(path.as_ref());
                self.documents.push(document);
            }
        }
        self.focus_document(&path).unwrap();
        self.add_recent_document(path);
    }

    pub fn open_document(&mut self, document: Document) {
        let path = document.path().to_owned();
        if self.document(document.path()).is_none() {
            self.documents.push(document);
        }
        self.focus_document(&path).unwrap();
        self.add_recent_document(path);
    }

    pub fn focus_document<T: AsRef<Path>>(&mut self, path: T) -> Result<(), AppError> {
        self.document(&path)
            .ok_or_else(|| AppError::DocumentNotFound(path.as_ref().to_owned()))?;
        self.current_document = Some(path.as_ref().to_owned());
        Ok(())
    }

    pub fn current_document(&self) -> Option<&Document> {
        match &self.current_document {
            None => None,
            Some(p) => self.documents.iter().find(|d| d.path() == p),
        }
    }

    pub fn current_document_mut(&mut self) -> Option<&mut Document> {
        self.current_document
            .clone()
            .and_then(|path| self.documents.iter_mut().find(|d| d.path() == path))
    }

    pub fn document<T: AsRef<Path>>(&mut self, path: T) -> Option<&Document> {
        self.documents.iter().find(|d| d.path() == path.as_ref())
    }

    pub fn document_mut<T: AsRef<Path>>(&mut self, path: T) -> Option<&mut Document> {
        self.documents
            .iter_mut()
            .find(|d| d.path() == path.as_ref())
    }

    pub fn relocate_document<T: AsRef<Path>, U: AsRef<Path>>(&mut self, from: T, to: U) {
        if from.as_ref() == to.as_ref() {
            return;
        }
        self.documents.retain(|d| d.path() != to.as_ref());
        if let Some(moved_document) = self.document_mut(&from) {
            moved_document.set_path(to.as_ref().to_owned());
        }
        if Some(from.as_ref()) == self.current_document.as_deref() {
            self.focus_document(&to).unwrap();
        }
        self.add_recent_document(to);
    }

    pub fn close_document<T: AsRef<Path>>(&mut self, path: T) {
        if let Some(index) = self
            .documents
            .iter()
            .position(|d| d.path() == path.as_ref())
        {
            self.documents.remove(index);
            self.current_document = if self.documents.is_empty() {
                None
            } else {
                Some(
                    self.documents[std::cmp::min(index, self.documents.len() - 1)]
                        .path()
                        .to_owned(),
                )
            };
        }
    }

    pub fn show_error_message(&mut self, title: String, summary: String, details: String) {
        self.errors.push(UserFacingError {
            key: uuid::Uuid::new_v4(),
            title,
            summary,
            details,
        });
    }

    pub fn error(&self) -> Option<&UserFacingError> {
        self.errors.first()
    }

    pub fn request_exit(&mut self) {
        self.exit_requested = true;
        for document in &mut self.documents {
            document.request_close();
        }
        self.advance_exit();
    }

    pub fn cancel_exit(&mut self) {
        self.exit_requested = false;
        for document in &mut self.documents {
            document.cancel_close();
        }
    }

    pub fn advance_exit(&mut self) {
        let closable_documents: Vec<PathBuf> = self
            .documents
            .iter()
            .filter(|d| d.should_close())
            .map(|d| d.path().to_owned())
            .collect();
        for path in closable_documents {
            self.close_document(path);
        }
    }

    pub fn should_exit(&self) -> bool {
        self.exit_requested && self.documents.is_empty()
    }

    pub fn acknowledge_error(&mut self) {
        if !self.errors.is_empty() {
            self.errors.remove(0);
        }
    }

    fn add_recent_document<T: AsRef<Path>>(&mut self, path: T) {
        self.recent_documents.mutate(|d| {
            d.retain(|p| p.as_path() != path.as_ref());
            d.insert(0, path.as_ref().to_owned());
            d.truncate(10);
        });
    }

    pub fn recent_documents(&self) -> impl Iterator<Item = &Path> {
        self.recent_documents.iter().map(|d| d.as_path())
    }

    pub fn set_recent_documents(&mut self, documents: Vec<PathBuf>) {
        self.recent_documents.mutate(|d| {
            *d = documents;
        });
    }

    pub fn recent_documents_delegate(&self) -> &Delegate<'static, Vec<PathBuf>> {
        self.recent_documents.delegate()
    }

    pub fn set_clipboard_manifest(&mut self, new_manifest: Option<ClipboardManifest>) {
        self.clipboard_manifest = new_manifest;
    }

    pub fn clipboard_manifest(&self) -> &Option<ClipboardManifest> {
        &self.clipboard_manifest
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use std::path::PathBuf;

    #[test]
    fn can_open_and_close_documents() {
        let mut state = State::default();

        state.open_document(Document::open("test-data/samurai.tiger").unwrap());
        assert_eq!(state.documents_iter().count(), 1);
        assert!(state.document("test-data/samurai.tiger").is_some());
        assert!(state.document_mut("test-data/samurai.tiger").is_some());

        state.open_document(Document::open("test-data/flame.tiger").unwrap());
        assert_eq!(state.documents_iter().count(), 2);
        assert!(state.document("test-data/flame.tiger").is_some());
        assert!(state.document_mut("test-data/flame.tiger").is_some());

        state.close_document("test-data/flame.tiger");
        assert_eq!(state.documents_iter().count(), 1);
        assert!(state.document("test-data/flame.tiger").is_none());
        assert!(state.document_mut("test-data/flame.tiger").is_none());
    }

    #[test]
    fn open_and_close_updates_focused_document() {
        let mut state = State::default();

        state.open_document(Document::open("test-data/samurai.tiger").unwrap());
        assert_eq!(
            state.current_document().unwrap().path(),
            Path::new("test-data/samurai.tiger")
        );

        state.open_document(Document::open("test-data/flame.tiger").unwrap());
        assert_eq!(
            state.current_document().unwrap().path(),
            Path::new("test-data/flame.tiger")
        );

        state.close_document("test-data/flame.tiger");
        assert_eq!(
            state.current_document().unwrap().path(),
            Path::new("test-data/samurai.tiger")
        );
    }

    #[test]
    fn can_manually_focus_a_document() {
        let mut state = State::default();

        state.open_document(Document::open("test-data/samurai.tiger").unwrap());
        state.open_document(Document::open("test-data/flame.tiger").unwrap());
        state.focus_document("test-data/samurai.tiger").unwrap();
        assert_eq!(
            state.current_document().unwrap().path(),
            Path::new("test-data/samurai.tiger")
        );
    }

    #[test]
    fn keeps_track_of_recently_opened_documents() {
        let mut state = State::default();

        state.open_document(Document::open("test-data/samurai.tiger").unwrap());
        assert_eq!(
            *state.recent_documents,
            vec![PathBuf::from("test-data/samurai.tiger")]
        );

        state.open_document(Document::open("test-data/flame.tiger").unwrap());
        assert_eq!(
            *state.recent_documents,
            vec![
                PathBuf::from("test-data/flame.tiger"),
                PathBuf::from("test-data/samurai.tiger")
            ]
        );

        state.open_document(Document::open("test-data/samurai.tiger").unwrap());
        assert_eq!(
            *state.recent_documents,
            vec![
                PathBuf::from("test-data/samurai.tiger"),
                PathBuf::from("test-data/flame.tiger"),
            ]
        );

        state.relocate_document("test-data/samurai.tiger", "relocated");
        assert_eq!(
            *state.recent_documents,
            vec![
                PathBuf::from("relocated"),
                PathBuf::from("test-data/samurai.tiger"),
                PathBuf::from("test-data/flame.tiger"),
            ]
        );

        state.new_document("new");
        assert_eq!(
            *state.recent_documents,
            vec![
                PathBuf::from("new"),
                PathBuf::from("relocated"),
                PathBuf::from("test-data/samurai.tiger"),
                PathBuf::from("test-data/flame.tiger"),
            ]
        );
    }

    #[test]
    fn limits_list_of_recent_documents() {
        let mut state = State::default();

        for i in 0..100 {
            state.add_recent_document(PathBuf::from(format!("doc_{i}")));
        }

        assert_eq!(
            *state.recent_documents,
            (90..=99)
                .rev()
                .map(|i| PathBuf::from(format!("doc_{i}")))
                .collect::<Vec<_>>()
        );
    }
}
