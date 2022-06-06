use std::path::{Path, PathBuf};
use std::time::Duration;
use thiserror::Error;

use crate::sheet::{Animation, Direction, Keyframe, Sequence, Sheet, SheetError};

mod command;
mod content;
mod persistent;
mod selection;
mod timeline;
mod transient;
mod view;

pub use command::*;
pub use content::*;
use persistent::*;
pub use selection::*;
pub use timeline::*;
pub use transient::*;
pub use view::*;

#[derive(Debug)]
pub struct Document {
    path: PathBuf,
    sheet: Sheet,           // Sheet being edited, fully recorded in history
    view: View, // View state, recorded in history but consecutive changes while the sheet stays unchanged are merged
    transient: Transient, // State preventing undo actions when not default, not recorded in history
    persistent: Persistent, // Other state not recorded in history
    next_version: i32,
    history: Vec<HistoryEntry>,
    history_index: usize,
}

#[derive(Error, Debug)]
pub enum DocumentError {
    #[error(transparent)]
    SheetError(#[from] SheetError),
    #[error("Animation `{0}` does not exist")]
    AnimationNotInDocument(String),
    #[error("Current animation does not have a `{0:?}` sequence")]
    SequenceNotInAnimation(Direction),
    #[error("Sequence does not have a keyframe at time `{0:?}`")]
    NoKeyframeAtTime(Duration),
    #[error("Sequence does not have a keyframe at index `{0}`")]
    NoKeyframeAtIndex(usize),
    #[error("Not currently editing an animation")]
    NotEditingAnyAnimation,
    #[error("Not currently editing a sequence")]
    NotEditingAnySequence,
    #[error("Not currently dragging a keyframe duration")]
    NotDraggingKeyframeDuration,
    #[error("Could not find duration of keyframe when drag started")]
    MissingKeyframeDragData,
    #[error("Not currently nudging a keyframe")]
    NotNudgingKeyframe,
    #[error("Could not find position of keyframe when drag started")]
    MissingKeyframePositionData,
    #[error("Could not find position of hitbox when drag started")]
    MissingHitboxPositionData,
}

impl Document {
    pub fn new<T: AsRef<Path>>(path: T) -> Document {
        let history_entry: HistoryEntry = Default::default();
        let next_version = history_entry.version;
        Document {
            path: path.as_ref().to_owned(),
            history: vec![history_entry],
            sheet: Default::default(),
            view: Default::default(),
            transient: Default::default(),
            persistent: Default::default(),
            next_version: next_version,
            history_index: 0,
        }
    }

    pub fn open<T: AsRef<Path>>(path: T) -> Result<Document, DocumentError> {
        let mut document = Document::new(&path);
        document.sheet = Sheet::read(path.as_ref())?;
        document.history[0].sheet = document.sheet.clone();
        document.persistent.disk_version = document.next_version;
        Ok(document)
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

    pub fn transient(&self) -> &Transient {
        &self.transient
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn clear_transient(&mut self) {
        self.transient = Default::default();
    }

    pub fn request_close(&mut self) {
        self.persistent.close_requested = true;
    }

    pub fn cancel_close(&mut self) {
        self.persistent.close_requested = false;
    }

    pub fn should_close(&self) -> bool {
        self.persistent.close_requested() && self.is_saved()
    }

    fn sanitize_view(&mut self) {
        match self.get_workbench_animation() {
            Ok((_, animation)) => {
                if self.get_workbench_sequence().is_err() {
                    let any_direction = animation.sequences_iter().next().map(|(d, _s)| *d);
                    self.view.current_sequence = any_direction;
                }
            }
            Err(_) => {
                self.view.current_animation = None;
                self.view.current_sequence = None;
            }
        };

        let timeline_cap = self
            .get_workbench_sequence()
            .ok()
            .and_then(|(_, s)| s.duration())
            .unwrap_or_default();
        self.view.timeline_clock = self.view.timeline_clock.min(timeline_cap);

        self.view
            .selection
            .frames
            .retain(|path| self.sheet.has_frame(path));

        self.view
            .selection
            .animations
            .retain(|name| self.sheet.has_animation(name));

        self.view
            .selection
            .keyframes
            .retain(|(name, direction, index)| {
                Some(name) == self.view.current_animation.as_ref()
                    && self
                        .sheet
                        .animation(name)
                        .and_then(|a| a.sequence(*direction))
                        .map(|s| *index < s.num_keyframes())
                        .unwrap_or_default()
            });

        // TODO hitbox selection cleanup
    }

    pub fn get_workbench_animation(&self) -> Result<(&String, &Animation), DocumentError> {
        let animation_name = self
            .view
            .current_animation()
            .as_ref()
            .ok_or_else(|| DocumentError::NotEditingAnyAnimation)?;
        let animation =
            self.sheet
                .animation(&animation_name)
                .ok_or(DocumentError::AnimationNotInDocument(
                    animation_name.to_owned(),
                ))?;
        Ok((animation_name, animation))
    }

    pub fn get_workbench_animation_mut(
        &mut self,
    ) -> Result<(String, &mut Animation), DocumentError> {
        let animation_name = self
            .view
            .current_animation()
            .clone()
            .ok_or_else(|| DocumentError::NotEditingAnyAnimation)?;
        let animation = self.sheet.animation_mut(&animation_name).ok_or(
            DocumentError::AnimationNotInDocument(animation_name.to_owned()),
        )?;
        Ok((animation_name, animation))
    }

    pub fn get_selected_keyframes(
        &self,
    ) -> Result<Vec<(Direction, usize, &Keyframe)>, DocumentError> {
        let (animation_name, animation) = self.get_workbench_animation()?;
        Ok(animation
            .sequences_iter()
            .flat_map(|(direction, sequence)| {
                sequence
                    .keyframes_iter()
                    .enumerate()
                    .filter_map(|(index, keyframe)| {
                        if self.view.selection().is_keyframe_selected(
                            animation_name,
                            *direction,
                            index,
                        ) {
                            Some((*direction, index, keyframe))
                        } else {
                            None
                        }
                    })
            })
            .collect())
    }

    pub fn get_workbench_sequence(&self) -> Result<(Direction, &Sequence), DocumentError> {
        let (_, animation) = self.get_workbench_animation()?;
        let direction = self
            .view
            .current_sequence()
            .ok_or_else(|| DocumentError::NotEditingAnySequence)?;
        Ok((
            direction,
            animation
                .sequence(direction)
                .ok_or(DocumentError::SequenceNotInAnimation(direction))?,
        ))
    }

    pub fn get_workbench_keyframe(&self) -> Result<((Direction, usize), &Keyframe), DocumentError> {
        let (direction, sequence) = self.get_workbench_sequence()?;
        let (index, keyframe) = sequence
            .keyframe_at(self.view.timeline_clock)
            .ok_or(DocumentError::NoKeyframeAtTime(self.view.timeline_clock))?;
        Ok(((direction, index), keyframe))
    }

    pub fn get_selected_keyframes_mut(
        &mut self,
    ) -> Result<Vec<(Direction, usize, &mut Keyframe)>, DocumentError> {
        let selection = self.view().selection().clone();
        let (animation_name, animation) = self.get_workbench_animation_mut()?;
        Ok(animation
            .sequences_iter_mut()
            .flat_map(|(direction, sequence)| {
                sequence
                    .keyframes_iter_mut()
                    .enumerate()
                    .filter_map(|(index, keyframe)| {
                        if selection.is_keyframe_selected(&animation_name, *direction, index) {
                            Some((*direction, index, keyframe))
                        } else {
                            None
                        }
                    })
            })
            .collect())
    }
}
