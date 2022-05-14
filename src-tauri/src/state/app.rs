use std::{
    path::{Path, PathBuf},
    sync::Mutex,
};
use thiserror::Error;

use crate::state::{Document, DocumentError};

#[derive(Error, Debug)]
pub enum AppError {
    #[error("No document is open")]
    NoDocumentOpen,
    #[error("The requested document (`{0}`) is not currently opened.")]
    DocumentNotFound(PathBuf),
    #[error(transparent)]
    DocumentError(#[from] DocumentError),
}

pub struct AppState(pub Mutex<App>);
#[derive(Debug, Default)]
pub struct App {
    documents: Vec<Document>,
    current_document: Option<PathBuf>,
    errors: Vec<String>,
}

impl App {
    pub fn documents_iter(&self) -> impl Iterator<Item = &Document> {
        self.documents.iter()
    }

    pub fn open_document(&mut self, document: Document) {
        let path = document.path().to_owned();
        if self.document(document.path()).is_none() {
            self.documents.push(document);
        }
        self.focus_document(&path).unwrap();
    }

    pub fn focus_document<T: AsRef<Path>>(&mut self, path: T) -> Result<(), AppError> {
        let document = self
            .document_mut(&path)
            .ok_or_else(|| AppError::DocumentNotFound(path.as_ref().to_path_buf()))?;
        document.clear_transient();
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

    pub fn show_error_message<T: AsRef<str>>(&mut self, message: T) {
        self.errors.push(message.as_ref().to_owned());
    }

    pub fn acknowledge_error(&mut self) {
        if !self.errors.is_empty() {
            self.errors.remove(0);
        }
    }
}

#[test]
fn can_open_and_close_documents() {
    let mut app = App::default();

    app.open_document("test-data/sample_sheet_1.tiger").unwrap();
    assert_eq!(app.documents_iter().count(), 1);
    assert!(app.document("test-data/sample_sheet_1.tiger").is_some());
    assert!(app.document_mut("test-data/sample_sheet_1.tiger").is_some());

    app.open_document("test-data/sample_sheet_2.tiger").unwrap();
    assert_eq!(app.documents_iter().count(), 2);
    assert!(app.document("test-data/sample_sheet_2.tiger").is_some());
    assert!(app.document_mut("test-data/sample_sheet_2.tiger").is_some());

    app.close_document("test-data/sample_sheet_2.tiger");
    assert_eq!(app.documents_iter().count(), 1);
    assert!(app.document("test-data/sample_sheet_2.tiger").is_none());
    assert!(app.document_mut("test-data/sample_sheet_2.tiger").is_none());
}

#[test]
fn open_and_close_update_focused_document() {
    let mut app = App::default();

    app.open_document("test-data/sample_sheet_1.tiger").unwrap();
    assert_eq!(
        app.current_document().unwrap().path(),
        Path::new("test-data/sample_sheet_1.tiger")
    );

    app.open_document("test-data/sample_sheet_2.tiger").unwrap();
    assert_eq!(
        app.current_document().unwrap().path(),
        Path::new("test-data/sample_sheet_2.tiger")
    );

    app.close_document("test-data/sample_sheet_2.tiger");
    assert_eq!(
        app.current_document().unwrap().path(),
        Path::new("test-data/sample_sheet_1.tiger")
    );
}

#[test]
fn can_manually_focus_a_document() {
    let mut app = App::default();
    app.open_document("test-data/sample_sheet_1.tiger").unwrap();
    app.open_document("test-data/sample_sheet_2.tiger").unwrap();
    app.focus_document("test-data/sample_sheet_1.tiger")
        .unwrap();
    assert_eq!(
        app.current_document().unwrap().path(),
        Path::new("test-data/sample_sheet_1.tiger")
    );
}
