use std::time::Duration;

use crate::sheet::{Direction, DirectionPreset};
use crate::state::*;

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

    pub(super) fn select_direction(&mut self, direction: Direction) -> Result<(), DocumentError> {
        self.view.current_sequence = Some(direction);
        let (animation_name, _) = self.get_workbench_animation()?;
        let animation_name = animation_name.clone();
        let sequence = self.get_workbench_sequence()?;
        if let Some(keyframe) = sequence.keyframe_index_at(self.view.timeline_clock) {
            self.view
                .selection
                .select_keyframe(animation_name, direction, keyframe);
        }
        Ok(())
    }
}
