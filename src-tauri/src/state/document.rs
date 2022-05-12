use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::sheet::{Sheet, SheetError};
use crate::state::*;

#[derive(Debug)]
pub struct Document {
    source: PathBuf,
    sheet: Sheet,                 // Sheet being edited, fully recorded in history
    view: View, // View state, recorded in history but consecutive changes while the sheet stays unchanged are merged
    transient: Option<Transient>, // State preventing undo actions when not default, not recorded in history
    persistent: Persistent,       // Other state not recorded in history
    next_version: i32,
    history: Vec<HistoryEntry>,
    history_index: usize,
}

#[derive(Debug)]
pub struct Transient {}

#[derive(Debug, Default)]
struct HistoryEntry {
    last_command: Option<DocumentCommand>,
    sheet: Sheet,
    view: View,
    version: i32,
}

#[derive(Clone, Debug, Default)]
pub struct Persistent {
    pub close_state: Option<CloseState>,
    timeline_is_playing: bool,
    disk_version: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CloseState {
    Requested,
    Saving,
    Allowed,
}

#[derive(Error, Debug)]
pub enum DocumentError {
    #[error("Invalid sheet operation: {0}")]
    SheetError(#[from] SheetError),
}

impl Document {
    pub fn new<T: AsRef<Path>>(path: T) -> Document {
        let history_entry: HistoryEntry = Default::default();
        let sheet = history_entry.sheet.clone();
        let view = history_entry.view.clone();
        let next_version = history_entry.version;
        Document {
            source: path.as_ref().to_owned(),
            history: vec![history_entry],
            sheet: sheet,
            view: view,
            transient: None,
            persistent: Default::default(),
            next_version: next_version,
            history_index: 0,
        }
    }

    pub fn sheet(&self) -> &Sheet {
        &self.sheet
    }

    pub fn open<T: AsRef<Path>>(path: T) -> Result<Document, DocumentError> {
        let mut document = Document::new(&path);
        document.sheet = Sheet::read(path.as_ref())?;
        document.history[0].sheet = document.sheet.clone();
        document.persistent.disk_version = document.next_version;
        Ok(document)
    }

    pub fn mark_as_saved(&mut self, saved_version: i32) {
        self.persistent.disk_version = saved_version;
    }

    pub fn version(&self) -> i32 {
        self.history[self.history_index].version
    }

    pub fn source(&self) -> &Path {
        &self.source
    }

    pub fn clear_transient(&mut self) {
        self.transient = None;
    }
}
