use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::sheet::{Sheet, SheetError};
use crate::state::*;

#[derive(Debug)]
pub struct Document {
    path: PathBuf,
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

impl Transient {
    fn is_transient_command(command: &Command) -> bool {
        // TODO list transient commands
        false
    }
}

#[derive(Debug, Default)]
struct HistoryEntry {
    last_command: Option<Command>,
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
    #[error(transparent)]
    SheetError(#[from] SheetError),
    #[error("Cannot manipulate undo history")]
    UndoOperationNowAllowed,
}

#[derive(Clone, Debug)]
pub enum Command {
    FocusContentTab(ContentTab),
}

impl Document {
    pub fn new<T: AsRef<Path>>(path: T) -> Document {
        let history_entry: HistoryEntry = Default::default();
        let sheet = history_entry.sheet.clone();
        let view = history_entry.view.clone();
        let next_version = history_entry.version;
        Document {
            path: path.as_ref().to_owned(),
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

    pub fn view(&self) -> &View {
        &self.view
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

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn clear_transient(&mut self) {
        self.transient = None;
    }

    fn focus_content_tab(&mut self, content_tab: ContentTab) {
        self.view.set_content_tab(content_tab);
    }

    fn push_undo_state(&mut self, entry: HistoryEntry) {
        self.history.truncate(self.history_index + 1);
        self.history.push(entry);
        self.history_index = self.history.len() - 1;

        while self.history.len() > 100 {
            self.history.remove(0);
            self.history_index -= 1;
        }
    }

    fn can_use_undo_system(&self) -> bool {
        self.transient.is_none()
    }

    fn record_command(&mut self, command: Command) {
        if !self.can_use_undo_system() {
            return;
        }

        let has_sheet_changes = &self.history[self.history_index].sheet != &self.sheet;

        if has_sheet_changes {
            self.next_version += 1;
        }

        let new_undo_state = HistoryEntry {
            sheet: self.sheet.clone(),
            view: self.view.clone(),
            last_command: Some(command),
            version: self.next_version,
        };

        if has_sheet_changes {
            self.push_undo_state(new_undo_state);
        } else if &self.history[self.history_index].view != &new_undo_state.view {
            let merge = self.history_index > 0
                && self.history[self.history_index - 1].sheet
                    == self.history[self.history_index].sheet;
            if merge {
                self.history[self.history_index].view = new_undo_state.view;
            } else {
                self.push_undo_state(new_undo_state);
            }
        }
    }

    pub fn undo(&mut self) -> Result<(), DocumentError> {
        if !self.can_use_undo_system() {
            return Err(DocumentError::UndoOperationNowAllowed);
        }
        if self.history_index > 0 {
            self.history_index -= 1;
            self.sheet = self.history[self.history_index].sheet.clone();
            self.view = self.history[self.history_index].view.clone();
            self.persistent.timeline_is_playing = false;
        }
        Ok(())
    }

    pub fn redo(&mut self) -> Result<(), DocumentError> {
        if !self.can_use_undo_system() {
            return Err(DocumentError::UndoOperationNowAllowed);
        }
        if self.history_index < self.history.len() - 1 {
            self.history_index += 1;
            self.sheet = self.history[self.history_index].sheet.clone();
            self.view = self.history[self.history_index].view.clone();
            self.persistent.timeline_is_playing = false;
        }
        Ok(())
    }

    pub fn get_undo_command(&self) -> Option<&Command> {
        self.history[self.history_index].last_command.as_ref()
    }

    pub fn get_redo_command(&self) -> Option<&Command> {
        if self.history_index < self.history.len() - 1 {
            self.history[self.history_index + 1].last_command.as_ref()
        } else {
            None
        }
    }

    pub fn process_command(&mut self, command: Command) {
        match command {
            Command::FocusContentTab(t) => self.focus_content_tab(t),
        }
        if !Transient::is_transient_command(&command) {
            self.transient = None;
        }
        self.record_command(command);
    }
}
