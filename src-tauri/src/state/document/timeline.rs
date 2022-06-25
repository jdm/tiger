use std::time::Duration;

use crate::sheet::{Direction, DirectionPreset};
use crate::state::*;

impl Document {
    pub(super) fn tick(&mut self, delta: Duration) {
        self.advance_timeline(delta);
    }

    fn advance_timeline(&mut self, delta: Duration) {
        if self.is_timeline_playing() {
            self.view.timeline_clock += delta;
            if let Ok((animation_name, animation)) = self.get_workbench_animation() {
                let animation_name = animation_name.clone();
                if let Ok((direction, sequence)) = self.get_workbench_sequence() {
                    let num_keyframes = sequence.num_keyframes();
                    match sequence.duration_millis() {
                        Some(d) if d > 0 => {
                            let clock_ms = self.timeline_clock().as_millis() as u64;
                            // Loop animation
                            if animation.looping() {
                                self.view.timeline_clock = Duration::from_millis(clock_ms % d);

                            // Stop playhead at the end of animation
                            } else if clock_ms >= d {
                                self.persistent.timeline_is_playing = false;
                                self.view.timeline_clock = Duration::from_millis(d);
                                if self.view.selection.keyframes().count() <= 1 {
                                    self.select_keyframe_only(
                                        animation_name,
                                        direction,
                                        num_keyframes - 1,
                                    );
                                }
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
        self.persistent.timeline_is_playing = true;
        self.view.selection.hitboxes.clear();
        if self
            .get_workbench_sequence()?
            .1
            .duration_millis()
            .map(Duration::from_millis)
            .map(|d| d <= self.view.timeline_clock)
            .unwrap_or_default()
        {
            self.view.skip_to_timeline_start();
        }
        Ok(())
    }

    pub(super) fn pause(&mut self) -> Result<(), DocumentError> {
        self.persistent.timeline_is_playing = false;
        let (animation_name, _) = self.get_workbench_animation()?;
        let animation_name = animation_name.clone();
        let (direction, sequence) = self.get_workbench_sequence()?;
        let keyframe_index = sequence
            .keyframe_index_at(self.view.timeline_clock)
            .unwrap_or_default();
        if self.view.selection.keyframes().count() <= 1 {
            self.select_keyframe_only(animation_name, direction, keyframe_index);
        }
        Ok(())
    }

    pub(super) fn scrub_timeline(&mut self, time: Duration) -> Result<(), DocumentError> {
        let (animation_name, _) = self.get_workbench_animation()?;
        let animation_name = animation_name.clone();
        let (direction, sequence) = self.get_workbench_sequence()?;
        let new_time = match sequence.duration() {
            Some(d) if d < time => d,
            Some(_) => time,
            None => Duration::ZERO,
        };
        let keyframe_index = sequence.keyframe_index_at(new_time).unwrap_or_default();
        self.view.timeline_clock = new_time;
        self.select_keyframe_only(animation_name, direction, keyframe_index);
        Ok(())
    }

    pub fn jump_to_animation_start(&mut self) -> Result<(), DocumentError> {
        self.scrub_timeline(Duration::ZERO)
    }

    pub fn jump_to_animation_end(&mut self) -> Result<(), DocumentError> {
        let (_, sequence) = self.get_workbench_sequence()?;
        let duration = sequence.duration().unwrap_or_default();
        self.scrub_timeline(duration)
    }

    pub fn jump_to_previous_frame(&mut self) -> Result<(), DocumentError> {
        let (_, sequence) = self.get_workbench_sequence()?;
        let now = self.view.timeline_clock.as_millis() as u64;
        let new_time = sequence
            .keyframe_time_ranges()
            .into_iter()
            .rev()
            .find(|range| range.end <= now)
            .map(|range| range.start)
            .unwrap_or_default();
        self.scrub_timeline(Duration::from_millis(new_time))
    }

    pub fn jump_to_next_frame(&mut self) -> Result<(), DocumentError> {
        let (_, sequence) = self.get_workbench_sequence()?;
        let now = self.view.timeline_clock.as_millis() as u64;
        let new_time = sequence
            .keyframe_times()
            .into_iter()
            .find(|time| *time > now)
            .or_else(|| sequence.keyframe_times().last().copied())
            .unwrap_or_default();
        self.scrub_timeline(Duration::from_millis(new_time))
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

    pub(super) fn select_direction(&mut self, direction: Direction) -> Result<(), DocumentError> {
        self.view.current_sequence = Some(direction);
        let (animation_name, _) = self.get_workbench_animation()?;
        let animation_name = animation_name.clone();
        let (direction, sequence) = self.get_workbench_sequence()?;
        if let Some(keyframe) = sequence.keyframe_index_at(self.view.timeline_clock) {
            self.select_keyframe_only(animation_name, direction, keyframe);
        } else {
            self.view.selection.keyframes.clear();
        }
        Ok(())
    }

    pub(super) fn delete_selected_keyframes(&mut self) -> Result<(), DocumentError> {
        let mut selected_keyframes = self
            .view
            .selection
            .keyframes()
            .map(|(_, d, i)| (*d, *i))
            .collect::<Vec<_>>();
        selected_keyframes.sort();
        selected_keyframes.reverse();
        let (_, animation) = self.get_workbench_animation_mut()?;
        for (direction, index) in selected_keyframes {
            animation
                .sequence_mut(direction)
                .ok_or(DocumentError::SequenceNotInAnimation(direction))?
                .delete_keyframe(index)?;
        }
        Ok(())
    }
}
