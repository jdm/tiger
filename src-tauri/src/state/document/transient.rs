use std::{collections::HashMap, path::PathBuf};

use crate::sheet::Direction;
use crate::state::*;

#[derive(Clone, Debug, PartialEq)]
pub(super) struct KeyframeDurationDrag {
    pub(super) frame_being_dragged: (Direction, usize),
    pub(super) original_durations: HashMap<(Direction, usize), u64>,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct FramesDragAndDrop {
    pub(super) frames: Vec<PathBuf>,
}

#[derive(Debug, Default)]
pub struct Transient {
    pub(super) frames_drag_and_drop: Option<FramesDragAndDrop>,
    pub(super) keyframe_duration_drag: Option<KeyframeDurationDrag>,
}

impl Transient {
    pub fn is_dragging_keyframe_duration(&self) -> bool {
        self.keyframe_duration_drag.is_some()
    }

    pub fn frames_being_dragged(&self) -> Option<Vec<PathBuf>> {
        self.frames_drag_and_drop.as_ref().map(|f| f.frames.clone())
    }
}

impl Document {
    pub(super) fn begin_drag_keyframe_duration(
        &mut self,
        direction: Direction,
        index: usize,
    ) -> Result<(), DocumentError> {
        let (animation_name, _) = self.get_workbench_animation()?;
        let animation_name = animation_name.clone();
        if !self
            .view
            .selection
            .is_keyframe_selected(&animation_name, direction, index)
        {
            self.view
                .selection
                .select_keyframe(animation_name, direction, index);
        }
        self.transient.keyframe_duration_drag = Some(KeyframeDurationDrag {
            frame_being_dragged: (direction, index),
            original_durations: self
                .get_selected_keyframes()?
                .into_iter()
                .map(|(d, i, k)| ((d, i), k.duration_millis()))
                .collect(),
        });
        Ok(())
    }

    pub(super) fn update_drag_keyframe_duration(
        &mut self,
        delta_millis: i64,
    ) -> Result<(), DocumentError> {
        let drag_state = self
            .transient
            .keyframe_duration_drag
            .clone()
            .ok_or_else(|| DocumentError::NotDraggingKeyframeDuration)?;

        let minimum_duration = 20.0 as u64;
        let duration_delta_per_frame = delta_millis
            / self
                .get_selected_keyframes()?
                .iter()
                .filter(|(d, i, _k)| {
                    drag_state.frame_being_dragged.0 == *d && drag_state.frame_being_dragged.1 >= *i
                })
                .count()
                .max(1) as i64;

        for (d, i, keyframe) in self.get_selected_keyframes_mut()? {
            let old_duration = drag_state
                .original_durations
                .get(&(d, i))
                .ok_or_else(|| DocumentError::MissingKeyframeDragData)?;
            let new_duration = if duration_delta_per_frame > 0 {
                old_duration.saturating_add(duration_delta_per_frame as u64)
            } else {
                old_duration.saturating_sub(duration_delta_per_frame.unsigned_abs())
            }
            .max(minimum_duration);
            keyframe.set_duration_millis(new_duration);
        }

        Ok(())
    }

    pub(super) fn begin_drag_and_drop_frames(&mut self, frames: Vec<PathBuf>) {
        self.transient.frames_drag_and_drop = Some(FramesDragAndDrop { frames });
    }

    pub(super) fn drop_frames_on_timeline(&mut self, direction: Direction, index: usize) {
        // TODO
    }
}
