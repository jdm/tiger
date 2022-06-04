use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::time::Duration;

use crate::sheet::{Direction, Keyframe};
use crate::state::*;

#[derive(Clone, Debug, PartialEq)]
pub(super) struct KeyframeDurationDrag {
    pub(super) frame_being_dragged: (Direction, usize),
    pub(super) original_durations: HashMap<(Direction, usize), u64>,
}

#[derive(Debug, Default)]
pub struct Transient {
    pub(super) frame_drag_and_drop: Option<PathBuf>,
    pub(super) keyframe_duration_drag: Option<KeyframeDurationDrag>,
    pub(super) keyframe_drag_and_drop: Option<(Direction, usize)>,
}

impl Document {
    pub(super) fn begin_drag_and_drop_frame(&mut self, frame: PathBuf) {
        if !self.view.selection.is_frame_selected(&frame) {
            self.view.selection.select_frame(frame.clone());
        }
        self.transient.frame_drag_and_drop = Some(frame);
    }

    pub(super) fn drop_frame_on_timeline(
        &mut self,
        direction: Direction,
        index: usize,
    ) -> Result<(), DocumentError> {
        let selected_frames: Vec<PathBuf> = self.view.selection.frames().cloned().collect(); // TODO sort
        let timeline_is_playing = self.persistent.timeline_is_playing;
        let (animation_name, animation) = self.get_workbench_animation_mut()?;
        let sequence = animation
            .sequence_mut(direction)
            .ok_or_else(|| DocumentError::SequenceNotInAnimation(direction))?;
        for frame in &selected_frames {
            let keyframe = Keyframe::new(frame);
            sequence.insert_keyframe(keyframe, index)?;
        }
        if !timeline_is_playing {
            self.view.timeline_clock = Duration::from_millis(sequence.keyframe_times()[index]);
            self.view.current_sequence = Some(direction);
        }
        self.view.selection.select_keyframes(
            (index..(index + selected_frames.len()))
                .map(|i| (animation_name.clone(), direction, i)),
        );
        Ok(())
    }

    pub fn frames_being_dragged(&self) -> Vec<PathBuf> {
        match self.transient.frame_drag_and_drop.is_some() {
            true => self.view.selection.frames().cloned().collect(),
            false => Vec::new(),
        }
    }

    pub(super) fn begin_drag_and_drop_keyframe(
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
        self.transient.keyframe_drag_and_drop = Some((direction, index));
        Ok(())
    }

    pub(super) fn drop_keyframe_on_timeline(
        &mut self,
        direction: Direction,
        index: usize,
    ) -> Result<(), DocumentError> {
        // TODO
        Ok(())
    }

    pub fn keyframes_being_dragged(&self) -> HashSet<(Direction, usize)> {
        match self.transient.keyframe_drag_and_drop.is_some() {
            true => self
                .view
                .selection
                .keyframes()
                .map(|(_, d, i)| (*d, *i))
                .collect(),
            false => HashSet::new(),
        }
    }

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
            self.view.current_sequence = Some(direction);
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

    pub fn is_dragging_keyframe_duration(&self) -> bool {
        self.transient.keyframe_duration_drag.is_some()
    }
}
