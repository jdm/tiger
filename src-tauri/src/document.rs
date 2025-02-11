use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::time::Duration;
use thiserror::Error;

use crate::sheet::*;

mod clipboard;
mod command;
mod content;
mod export;
mod keyframe;
mod relocate;
mod selection;
mod timeline;
mod transient;
mod view;

pub use clipboard::*;
pub use command::*;
pub use content::*;
pub use export::*;
pub use keyframe::*;
pub use relocate::*;
pub use selection::*;
pub use timeline::*;
pub use transient::*;
pub use view::*;

#[derive(Debug)]
pub struct Document {
    path: PathBuf,
    sheet: Sheet<Absolute>, // Sheet being edited, fully recorded in history
    view: View, // View state, recorded in history but consecutive changes while the sheet stays unchanged are merged
    detached_view: Option<View>, // Copy of view state while browsing at an older point in history (may differ from view state in history)
    transient: Transient, // State preventing undo actions when not default, not recorded in history
    persistent: Persistent, // Other state not recorded in history
    latest_version: i32,
    history: Vec<HistoryEntry>,
    history_index: usize,
}

#[derive(Clone, Debug, Default)]
pub struct Persistent {
    pub(super) disk_version: Option<i32>,
    pub(super) close_requested: bool,
    pub(super) timeline_is_playing: bool,
    pub(super) export_settings_edit: Option<ExportSettings<Any>>,
    pub(super) relocate_frames_edit: Option<HashMap<PathBuf, PathBuf>>,
    pub(super) preserve_aspect_ratio: bool,
    pub(super) missing_textures: HashSet<PathBuf>,
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
    #[error("Not currently renaming an animation")]
    NotRenamingAnyAnimation,
    #[error("Not currently renaming a hitbox")]
    NotRenamingAnyHitbox,
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
    #[error("Not currently nudging a hitbox")]
    NotNudgingHitbox,
    #[error("Not currently resizing a hitbox")]
    NotResizingHitbox,
    #[error("Not currently adjusting export settings")]
    NotEditingExportSettings,
    #[error("Not currently relocating frames")]
    NotRelocatingFrames,
    #[error("Sequence in animation has no keyframes")]
    SequenceHasNoKeyframes,
}

pub type DocumentResult<T> = Result<T, DocumentError>;

impl Document {
    pub fn new<T: AsRef<Path>>(path: T) -> Document {
        let history_entry: HistoryEntry = Default::default();
        let latest_version = history_entry.version;
        Document {
            path: path.as_ref().to_owned(),
            history: vec![history_entry],
            sheet: Default::default(),
            view: Default::default(),
            detached_view: Default::default(),
            transient: Default::default(),
            persistent: Default::default(),
            latest_version,
            history_index: 0,
        }
    }

    pub fn open<T: AsRef<Path>>(path: T) -> DocumentResult<Document> {
        let mut directory = path.as_ref().to_owned();
        directory.pop();

        let mut document = Document::new(&path);
        document.sheet = Sheet::<Any>::read(path.as_ref())?
            .with_relative_paths(directory)?
            .with_absolute_paths();
        document.mark_as_saved(document.version());

        if let Some(name) = document
            .sheet
            .sorted_animations()
            .into_iter()
            .map(|(name, _)| name)
            .min()
            .cloned()
        {
            document.edit_animation(name)?;
        }

        document.history[0].sheet = document.sheet.clone();
        document.history[0].view = document.view.clone();

        Ok(document)
    }

    pub fn sheet(&self) -> &Sheet<Absolute> {
        &self.sheet
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn set_path(&mut self, new_path: PathBuf) {
        self.path = new_path
    }

    pub fn request_close(&mut self) {
        self.persistent.close_requested = true;
    }

    pub fn cancel_close(&mut self) {
        self.persistent.close_requested = false;
    }

    pub fn close_requested(&self) -> bool {
        self.persistent.close_requested
    }

    pub fn should_close(&self) -> bool {
        self.close_requested() && self.is_saved()
    }

    pub fn is_timeline_playing(&self) -> bool {
        self.persistent.timeline_is_playing
    }

    pub fn preserves_aspect_ratio(&self) -> bool {
        self.persistent.preserve_aspect_ratio
    }

    fn sanitize_view(&mut self) {
        match self.workbench_animation() {
            Ok((_, animation)) => {
                if self.workbench_sequence().is_err() {
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
            .workbench_sequence()
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

        if self.view.lock_hitboxes || self.persistent.timeline_is_playing {
            self.view.selection.hitboxes.clear();
        }

        let current_keyframe_index = self
            .workbench_sequence()
            .ok()
            .and_then(|(_, s)| s.keyframe_at(self.view.timeline_clock))
            .map(|(i, _)| i);
        self.view
            .selection
            .hitboxes
            .retain(|(animation_name, direction, index, hitbox_name)| {
                Some(animation_name) == self.view.current_animation.as_ref()
                    && Some(direction) == self.view.current_sequence.as_ref()
                    && Some(*index) == current_keyframe_index
                    && self
                        .sheet
                        .animation(animation_name)
                        .and_then(|a| a.sequence(*direction))
                        .and_then(|s| s.keyframe(*index))
                        .map(|k| k.has_hitbox(hitbox_name))
                        .unwrap_or_default()
            });
    }

    pub fn workbench_animation(&self) -> DocumentResult<(&String, &Animation<Absolute>)> {
        let animation_name = self
            .current_animation()
            .as_ref()
            .ok_or(DocumentError::NotEditingAnyAnimation)?;
        let animation = self
            .sheet
            .animation(animation_name)
            .ok_or_else(|| DocumentError::AnimationNotInDocument(animation_name.to_owned()))?;
        Ok((animation_name, animation))
    }

    pub fn workbench_animation_mut(
        &mut self,
    ) -> DocumentResult<(String, &mut Animation<Absolute>)> {
        let animation_name = self
            .current_animation()
            .clone()
            .ok_or(DocumentError::NotEditingAnyAnimation)?;
        let animation = self
            .sheet
            .animation_mut(&animation_name)
            .ok_or_else(|| DocumentError::AnimationNotInDocument(animation_name.to_owned()))?;
        Ok((animation_name, animation))
    }

    pub fn workbench_sequence(&self) -> DocumentResult<(Direction, &Sequence<Absolute>)> {
        let (_, animation) = self.workbench_animation()?;
        let direction = self
            .current_sequence()
            .ok_or(DocumentError::NotEditingAnySequence)?;
        Ok((
            direction,
            animation
                .sequence(direction)
                .ok_or(DocumentError::SequenceNotInAnimation(direction))?,
        ))
    }

    pub fn workbench_sequence_mut(
        &mut self,
    ) -> DocumentResult<(Direction, &mut Sequence<Absolute>)> {
        let direction = self
            .current_sequence()
            .ok_or(DocumentError::NotEditingAnySequence)?;
        let (_, animation) = self.workbench_animation_mut()?;
        Ok((
            direction,
            animation
                .sequence_mut(direction)
                .ok_or(DocumentError::SequenceNotInAnimation(direction))?,
        ))
    }

    pub fn workbench_keyframe(&self) -> DocumentResult<((Direction, usize), &Keyframe<Absolute>)> {
        let (direction, sequence) = self.workbench_sequence()?;
        let (index, keyframe) = sequence
            .keyframe_at(self.view.timeline_clock)
            .ok_or(DocumentError::NoKeyframeAtTime(self.view.timeline_clock))?;
        Ok(((direction, index), keyframe))
    }

    pub fn workbench_keyframe_mut(
        &mut self,
    ) -> DocumentResult<((Direction, usize), &mut Keyframe<Absolute>)> {
        let timeline_clock = self.view.timeline_clock;
        let (direction, sequence) = self.workbench_sequence_mut()?;
        let (index, keyframe) = sequence
            .keyframe_at_mut(timeline_clock)
            .ok_or(DocumentError::NoKeyframeAtTime(timeline_clock))?;
        Ok(((direction, index), keyframe))
    }

    pub fn selected_animations(&self) -> Vec<(&String, &Animation<Absolute>)> {
        self.sheet
            .animations_iter()
            .filter(|(name, _)| self.view.selection.is_animation_selected(name))
            .collect()
    }

    pub fn selected_keyframes(
        &self,
    ) -> DocumentResult<Vec<(Direction, usize, &Keyframe<Absolute>)>> {
        let (animation_name, animation) = self.workbench_animation()?;
        Ok(animation
            .sequences_iter()
            .flat_map(|(direction, sequence)| {
                sequence
                    .keyframes_iter()
                    .enumerate()
                    .filter_map(|(index, keyframe)| {
                        if self
                            .selection()
                            .is_keyframe_selected(animation_name, *direction, index)
                        {
                            Some((*direction, index, keyframe))
                        } else {
                            None
                        }
                    })
            })
            .collect())
    }

    pub fn selected_keyframes_mut(
        &mut self,
    ) -> DocumentResult<Vec<(Direction, usize, &mut Keyframe<Absolute>)>> {
        let selection = self.view.selection.clone();
        let (animation_name, animation) = self.workbench_animation_mut()?;
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

    pub fn selected_hitboxes(&self) -> DocumentResult<Vec<(&String, &Hitbox)>> {
        let (animation_name, _) = self.workbench_animation()?;
        let selection = self.view.selection.clone();
        let ((direction, index), keyframe) = self.workbench_keyframe()?;
        Ok(keyframe
            .hitboxes_iter()
            .filter_map(|(hitbox_name, hitbox)| {
                if selection.is_hitbox_selected(
                    animation_name.clone(),
                    direction,
                    index,
                    hitbox_name,
                ) {
                    Some((hitbox_name, hitbox))
                } else {
                    None
                }
            })
            .collect())
    }

    pub fn selected_hitboxes_mut(&mut self) -> DocumentResult<Vec<(String, &mut Hitbox)>> {
        let (animation_name, _) = self.workbench_animation_mut()?;
        let selection = self.view.selection.clone();
        let ((direction, index), keyframe) = self.workbench_keyframe_mut()?;
        Ok(keyframe
            .hitboxes_iter_mut()
            .filter_map(|(hitbox_name, hitbox)| {
                if selection.is_hitbox_selected(
                    animation_name.clone(),
                    direction,
                    index,
                    hitbox_name,
                ) {
                    Some((hitbox_name.clone(), hitbox))
                } else {
                    None
                }
            })
            .collect())
    }
}
