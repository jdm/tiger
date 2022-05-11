use std::{
    path::{Path, PathBuf},
    sync::Mutex,
};
use thiserror::Error;

use crate::state::{Document, DocumentError};

#[derive(Error, Debug)]
pub enum AppError {
    #[error("The requested document (`{0}`) is not currently opened.")]
    DocumentNotFound(PathBuf),
    #[error("Invalid document operation: {0}")]
    DocumentError(#[from] DocumentError),
}

impl From<AppError> for String {
    fn from(e: AppError) -> Self {
        e.to_string()
    }
}

pub struct AppState(pub Mutex<App>);
#[derive(Debug, Default)]
pub struct App {
    documents: Vec<Document>,
    current_document: Option<PathBuf>,
}

impl App {
    pub fn documents_iter(&self) -> impl Iterator<Item = &Document> {
        self.documents.iter()
    }

    pub fn open_document<T: AsRef<Path>>(&mut self, path: T) -> Result<(), AppError> {
        if self.get_document(&path).is_none() {
            let document = Document::open(&path)?;
            self.documents.push(document);
        }
        self.focus_document(path)
    }

    pub fn focus_document<T: AsRef<Path>>(&mut self, path: T) -> Result<(), AppError> {
        let document = self
            .get_document_mut(&path)
            .ok_or_else(|| AppError::DocumentNotFound(path.as_ref().to_path_buf()))?;
        document.clear_transient();
        self.current_document = Some(path.as_ref().to_owned());
        Ok(())
    }

    pub fn get_current_document(&self) -> Option<&Document> {
        match &self.current_document {
            None => None,
            Some(p) => self.documents.iter().find(|d| d.source() == p),
        }
    }

    fn get_document<T: AsRef<Path>>(&mut self, path: T) -> Option<&Document> {
        self.documents.iter().find(|d| d.source() == path.as_ref())
    }

    fn get_document_mut<T: AsRef<Path>>(&mut self, path: T) -> Option<&mut Document> {
        self.documents
            .iter_mut()
            .find(|d| d.source() == path.as_ref())
    }
}
