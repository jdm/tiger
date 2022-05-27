use euclid::default::Vector2D;
use std::path::{Path, PathBuf};
use std::time::Duration;
use thiserror::Error;

use crate::sheet::{Animation, Direction, Sequence, Sheet, SheetError};
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

#[derive(Error, Debug)]
pub enum DocumentError {
    #[error(transparent)]
    SheetError(#[from] SheetError),
    #[error("Cannot manipulate undo history")]
    UndoOperationNotAllowed,
    #[error("Animation `{0}` does not exist")]
    AnimationNotInDocument(String),
    #[error("Current animation does not have a `{0:?}` sequence")]
    SequenceNotInAnimation(Direction),
    #[error("Sequence does not have a keyframe at time `{0:?}`")]
    NoKeyframeAtTime(Duration),
    #[error("Not currently editing an animation")]
    NotEditingAnyAnimation,
    #[error("Not currently editing a sequence")]
    NotEditingAnySequence,
}

#[derive(Clone, Debug)]
pub enum Command {
    Undo,
    Redo,
    FocusContentTab(ContentTab),
    ClearSelection,
    AlterSelection(SingleSelection, bool, bool),
    Pan(Vector2D<f32>),
    CenterWorkbench,
    ZoomInWorkbench,
    ZoomOutWorkbench,
    ResetWorkbenchZoom,
    EditAnimation(String),
    RenameAnimation(String, String),
    DeleteAnimation(String),
    Tick(Duration),
    Play,
    Pause,
    ScrubTimeline(Duration),
    ZoomInTimeline,
    ZoomOutTimeline,
    ResetTimelineZoom,
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

    pub fn persistent(&self) -> &Persistent {
        &self.persistent
    }

    pub fn open<T: AsRef<Path>>(path: T) -> Result<Document, DocumentError> {
        let mut document = Document::new(&path);
        document.sheet = Sheet::read(path.as_ref())?;
        document.history[0].sheet = document.sheet.clone();
        document.persistent.set_disk_version(document.next_version);
        Ok(document)
    }

    pub fn is_saved(&self) -> bool {
        self.persistent.disk_version() == self.version()
    }

    pub fn mark_as_saved(&mut self, saved_version: i32) {
        self.persistent.set_disk_version(saved_version);
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

    pub fn request_close(&mut self) {
        self.persistent.set_close_requested(true);
    }

    pub fn should_close(&self) -> bool {
        self.persistent.close_requested() && self.is_saved()
    }

    fn focus_content_tab(&mut self, content_tab: ContentTab) {
        self.view.set_content_tab(content_tab);
    }

    fn alter_selection(&mut self, selection: &SingleSelection, shift: bool, ctrl: bool) {
        let edit = match selection {
            SingleSelection::Frame(f) => MultiSelectionEdit::Frames(
                f.clone(),
                self.sheet
                    .frames_iter()
                    .map(|f| f.source().to_owned())
                    .collect(),
            ),
            SingleSelection::Animation(a) => MultiSelectionEdit::Animations(
                a.clone(),
                self.sheet
                    .animations_iter()
                    .map(|(n, _)| n.clone())
                    .collect(),
            ),
            SingleSelection::Hitbox(_) => todo!(),
            SingleSelection::Keyframe(_) => todo!(),
        };
        self.view.selection_mut().alter(edit, shift, ctrl);
    }

    fn edit_animation<T: AsRef<str>>(&mut self, name: T) -> Result<(), DocumentError> {
        let animation =
            self.sheet
                .animation(&name)
                .ok_or(DocumentError::AnimationNotInDocument(
                    name.as_ref().to_owned(),
                ))?;
        self.view.set_current_animation(&name);
        self.view.center_workbench();
        self.view.skip_to_timeline_start();
        self.persistent.set_timeline_is_playing(false);
        // TODO preserve current direction if possible?
        match animation.sequences_iter().next().map(|(d, _s)| d) {
            Some(d) => self.view.set_current_sequence(*d),
            None => self.view.clear_current_sequence(),
        }
        Ok(())
    }

    fn rename_animation<T: AsRef<str>, U: AsRef<str>>(
        &mut self,
        old_name: T,
        new_name: U,
    ) -> Result<(), DocumentError> {
        self.sheet.rename_animation(&old_name, &new_name)?;
        self.view
            .selection_mut()
            .select(SingleSelection::Animation(new_name.as_ref().to_owned()));
        if Some(old_name.as_ref()) == self.view.current_animation().as_deref() {
            self.view.set_current_animation(new_name);
        }
        Ok(())
    }

    fn delete_animation<T: AsRef<str>>(&mut self, name: T) {
        self.sheet.delete_animation(&name);
        self.view
            .selection_mut()
            .remove(SingleSelection::Animation(name.as_ref().to_owned()));
        if Some(name.as_ref()) == self.view.current_animation().as_deref() {
            self.view.clear_current_animation();
        }
    }

    fn tick(&mut self, delta: Duration) {
        self.advance_timeline(delta);
        // self.try_close(); // TODO
    }

    fn advance_timeline(&mut self, delta: Duration) {
        if self.persistent.is_timeline_playing() {
            self.view
                .set_timeline_clock(self.view.timeline_clock() + delta);
            if let Ok(animation) = self.get_workbench_animation() {
                if let Ok(sequence) = self.get_workbench_sequence() {
                    match sequence.duration_millis() {
                        Some(d) if d > 0 => {
                            let clock_ms = self.view.timeline_clock().as_millis() as u64;
                            // Loop animation
                            if animation.looping() {
                                self.view
                                    .set_timeline_clock(Duration::from_millis(clock_ms % d));

                            // Stop playhead at the end of animation
                            } else if clock_ms >= d {
                                self.persistent.set_timeline_is_playing(false);
                                self.view
                                    .set_timeline_clock(Duration::from_millis(u64::from(d)));
                            }
                        }

                        // Reset playhead
                        _ => {
                            self.persistent.set_timeline_is_playing(false);
                            self.view.skip_to_timeline_start();
                        }
                    };
                }
            }
        }
    }

    fn play(&mut self) -> Result<(), DocumentError> {
        if self.persistent.is_timeline_playing() {
            return Ok(());
        }

        let sequence = self.get_workbench_sequence()?;
        if let Some(d) = sequence.duration_millis() {
            if d > 0 && self.view.timeline_clock().as_millis() >= u128::from(d) {
                self.view.skip_to_timeline_start();
            }
        }

        self.persistent.set_timeline_is_playing(true);

        if self.view.selection().has_hitboxes() || self.view.selection().has_keyframes() {
            self.view.selection_mut().clear();
        }

        Ok(())
    }

    fn pause(&mut self) {
        self.persistent.set_timeline_is_playing(false);
    }

    fn scrub_timeline(&mut self, time: Duration) -> Result<(), DocumentError> {
        let sequence = self.get_workbench_sequence()?;
        let new_time = match sequence.duration() {
            Some(d) if d < time => d,
            Some(_) => time,
            None => Duration::ZERO,
        };
        let (index, _) = sequence
            .keyframe_at(new_time)
            .ok_or(DocumentError::NoKeyframeAtTime(new_time))?;

        self.view.set_timeline_clock(new_time);
        self.view
            .selection_mut()
            .select(SingleSelection::Keyframe(index));

        Ok(())
    }

    pub fn get_workbench_sequence(&self) -> Result<&Sequence, DocumentError> {
        let animation = self.get_workbench_animation()?;
        let direction = self
            .view
            .current_sequence()
            .ok_or_else(|| DocumentError::NotEditingAnySequence)?;
        animation
            .sequence(direction)
            .ok_or(DocumentError::SequenceNotInAnimation(direction))
    }

    pub fn get_workbench_animation(&self) -> Result<&Animation, DocumentError> {
        let animation_name = self
            .view
            .current_animation()
            .clone()
            .ok_or_else(|| DocumentError::NotEditingAnyAnimation)?;
        self.sheet
            .animation(&animation_name)
            .ok_or(DocumentError::AnimationNotInDocument(
                animation_name.to_owned(),
            ))
    }

    pub fn get_workbench_animation_mut(&mut self) -> Result<&mut Animation, DocumentError> {
        let animation_name = self
            .view
            .current_animation()
            .clone()
            .ok_or_else(|| DocumentError::NotEditingAnyAnimation)?;
        self.sheet
            .animation_mut(&animation_name)
            .ok_or(DocumentError::AnimationNotInDocument(
                animation_name.to_owned(),
            ))
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
            return Err(DocumentError::UndoOperationNotAllowed);
        }
        if self.history_index > 0 {
            self.history_index -= 1;
            self.sheet = self.history[self.history_index].sheet.clone();
            self.view = self.history[self.history_index].view.clone();
            self.persistent.set_timeline_is_playing(false);
        }
        Ok(())
    }

    pub fn redo(&mut self) -> Result<(), DocumentError> {
        if !self.can_use_undo_system() {
            return Err(DocumentError::UndoOperationNotAllowed);
        }
        if self.history_index < self.history.len() - 1 {
            self.history_index += 1;
            self.sheet = self.history[self.history_index].sheet.clone();
            self.view = self.history[self.history_index].view.clone();
            self.persistent.set_timeline_is_playing(false);
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

    pub fn process_command(&mut self, command: Command) -> Result<(), DocumentError> {
        match command {
            Command::Undo => self.undo()?,
            Command::Redo => self.redo()?,
            Command::FocusContentTab(t) => self.focus_content_tab(t),
            Command::ClearSelection => self.view.selection_mut().clear(),
            Command::AlterSelection(ref selection, shift, ctrl) => {
                self.alter_selection(selection, shift, ctrl)
            }
            Command::Pan(delta) => self.view.pan(delta),
            Command::CenterWorkbench => self.view.center_workbench(),
            Command::ZoomInWorkbench => self.view.zoom_in_workbench(),
            Command::ZoomOutWorkbench => self.view.zoom_out_workbench(),
            Command::ResetWorkbenchZoom => self.view.reset_workbench_zoom(),
            Command::EditAnimation(ref name) => self.edit_animation(name)?,
            Command::RenameAnimation(ref old_name, ref new_name) => {
                self.rename_animation(old_name, new_name)?
            }
            Command::DeleteAnimation(ref name) => self.delete_animation(name),
            Command::Tick(dt) => self.tick(dt),
            Command::Play => self.play()?,
            Command::Pause => self.pause(),
            Command::ScrubTimeline(t) => self.scrub_timeline(t)?,
            Command::ZoomInTimeline => self.view.zoom_in_timeline(),
            Command::ZoomOutTimeline => self.view.zoom_out_timeline(),
            Command::ResetTimelineZoom => self.view.reset_timeline_zoom(),
        }
        if !Transient::is_transient_command(&command) {
            self.transient = None;
        }
        self.record_command(command);
        Ok(())
    }
}
