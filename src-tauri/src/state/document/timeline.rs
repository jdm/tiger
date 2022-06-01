use std::collections::HashMap;
use std::time::Duration;

use crate::sheet::{Direction, DirectionPreset};
use crate::state::{Document, DocumentError};

#[derive(Clone, Debug, PartialEq)]
pub(super) struct KeyframeDurationDrag {
    frame_being_dragged: (Direction, usize),
    original_durations: HashMap<(Direction, usize), u64>,
}

impl Document {
    pub(super) fn tick(&mut self, delta: Duration) {
        self.advance_timeline(delta);
    }

    fn advance_timeline(&mut self, delta: Duration) {
        if self.persistent.is_timeline_playing() {
            self.view.timeline_clock += delta;
            if let Ok((_, animation)) = self.get_workbench_animation() {
                if let Ok(sequence) = self.get_workbench_sequence() {
                    match sequence.duration_millis() {
                        Some(d) if d > 0 => {
                            let clock_ms = self.view.timeline_clock().as_millis() as u64;
                            // Loop animation
                            if animation.looping() {
                                self.view.timeline_clock = Duration::from_millis(clock_ms % d);

                            // Stop playhead at the end of animation
                            } else if clock_ms >= d {
                                self.persistent.timeline_is_playing = false;
                                self.view.timeline_clock = Duration::from_millis(u64::from(d));
                            }
                        }

                        // Reset playhead
                        _ => {
                            self.persistent.timeline_is_playing = false;
                            self.view.skip_to_timeline_start();
                        }
                    };
                }
            }
        }
    }

    pub(super) fn play(&mut self) -> Result<(), DocumentError> {
        if self.persistent.is_timeline_playing() {
            return Ok(());
        }

        let sequence = self.get_workbench_sequence()?;
        if let Some(d) = sequence.duration_millis() {
            if d > 0 && self.view.timeline_clock().as_millis() >= u128::from(d) {
                self.view.skip_to_timeline_start();
            }
        }
        self.persistent.timeline_is_playing = true;
        Ok(())
    }

    pub(super) fn pause(&mut self) {
        self.persistent.timeline_is_playing = false;
    }

    pub(super) fn scrub_timeline(&mut self, time: Duration) -> Result<(), DocumentError> {
        let sequence = self.get_workbench_sequence()?;
        let new_time = match sequence.duration() {
            Some(d) if d < time => d,
            Some(_) => time,
            None => Duration::ZERO,
        };
        self.view.timeline_clock = new_time;
        Ok(())
    }

    pub(super) fn set_animation_looping(&mut self, is_looping: bool) -> Result<(), DocumentError> {
        let (_, animation) = self.get_workbench_animation_mut()?;
        animation.set_looping(is_looping);
        Ok(())
    }

    pub(super) fn apply_direction_preset(
        &mut self,
        preset: DirectionPreset,
    ) -> Result<(), DocumentError> {
        let (_, animation) = self.get_workbench_animation_mut()?;
        animation.apply_direction_preset(preset);
        Ok(())
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
}
