use enum_iterator::{all, last, reverse_all};
use std::time::Duration;

use crate::document::*;
use crate::sheet::{Direction, DirectionPreset};

impl Document {
    pub(super) fn tick(&mut self, delta: Duration) {
        self.advance_timeline(delta);
    }

    fn advance_timeline(&mut self, delta: Duration) {
        if self.is_timeline_playing() {
            self.view.timeline_clock += delta;
            if let Ok((_, animation)) = self.get_workbench_animation() {
                if let Ok((_, sequence)) = self.get_workbench_sequence() {
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
                                    self.select_current_keyframe().ok();
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

    pub(super) fn play(&mut self) -> DocumentResult<()> {
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

    pub(super) fn pause(&mut self) -> DocumentResult<()> {
        self.persistent.timeline_is_playing = false;
        if self.view.selection.keyframes().count() <= 1 {
            self.select_current_keyframe().ok();
        }
        Ok(())
    }

    pub(super) fn scrub_timeline(&mut self, time: Duration) -> DocumentResult<()> {
        let (_, sequence) = self.get_workbench_sequence()?;
        let new_time = match sequence.duration() {
            Some(d) if d < time => d,
            Some(_) => time,
            None => Duration::ZERO,
        };
        self.view.timeline_clock = new_time;
        self.select_current_keyframe().ok();
        Ok(())
    }

    pub(super) fn jump_to_animation_start(&mut self) -> DocumentResult<()> {
        self.scrub_timeline(Duration::ZERO)
    }

    pub(super) fn jump_to_animation_end(&mut self) -> DocumentResult<()> {
        let (_, sequence) = self.get_workbench_sequence()?;
        let duration = sequence.duration().unwrap_or_default();
        self.scrub_timeline(duration)
    }

    pub(super) fn jump_to_previous_frame(&mut self) -> DocumentResult<()> {
        let (_, sequence) = self.get_workbench_sequence()?;
        let now = self.view.timeline_clock;
        let new_time = sequence
            .keyframe_time_ranges()
            .into_iter()
            .rev()
            .find(|range| range.end <= now.as_millis() as u64)
            .map(|range| Duration::from_millis(range.start))
            .unwrap_or(Duration::ZERO);
        self.view.timeline_clock = new_time;
        self.select_current_keyframe()?;
        Ok(())
    }

    pub(super) fn jump_to_next_frame(&mut self) -> DocumentResult<()> {
        let (_, sequence) = self.get_workbench_sequence()?;
        let now = self.view.timeline_clock;
        let new_time = sequence
            .keyframe_time_ranges()
            .into_iter()
            .find(|range| range.start > now.as_millis() as u64)
            .map(|range| Duration::from_millis(range.start))
            .or_else(|| sequence.duration())
            .unwrap_or(Duration::ZERO);
        self.view.timeline_clock = new_time;
        self.select_current_keyframe()?;
        Ok(())
    }

    pub(super) fn cycle_directions_backward(&mut self) -> DocumentResult<()> {
        let now = self.view.timeline_clock;
        let old_direction = self
            .view
            .current_sequence
            .unwrap_or_else(|| last::<Direction>().unwrap());

        let (_, animation) = self.get_workbench_animation()?;
        let new_direction = reverse_all::<Direction>()
            .cycle()
            .skip_while(|d| *d != old_direction)
            .skip(1)
            .take_while(|d| *d != old_direction)
            .find(|d| {
                if let Some(sequence) = animation.sequence(*d) {
                    sequence.duration().map(|d| d >= now).unwrap_or(false)
                } else {
                    false
                }
            });

        self.view.current_sequence = new_direction;
        self.select_current_keyframe()?;
        Ok(())
    }

    pub(super) fn cycle_directions_forward(&mut self) -> DocumentResult<()> {
        let now = self.view.timeline_clock;
        let old_direction = self
            .view
            .current_sequence
            .unwrap_or_else(|| last::<Direction>().unwrap());

        let (_, animation) = self.get_workbench_animation()?;
        let new_direction = all::<Direction>()
            .cycle()
            .skip_while(|d| *d != old_direction)
            .skip(1)
            .take_while(|d| *d != old_direction)
            .find(|d| {
                if let Some(sequence) = animation.sequence(*d) {
                    sequence.duration().map(|d| d >= now).unwrap_or(false)
                } else {
                    false
                }
            });

        self.view.current_sequence = new_direction;
        self.select_current_keyframe()?;
        Ok(())
    }

    pub(super) fn set_animation_looping(&mut self, is_looping: bool) -> DocumentResult<()> {
        let (_, animation) = self.get_workbench_animation_mut()?;
        animation.set_looping(is_looping);
        Ok(())
    }

    pub(super) fn apply_direction_preset(&mut self, preset: DirectionPreset) -> DocumentResult<()> {
        let (_, animation) = self.get_workbench_animation_mut()?;
        animation.apply_direction_preset(preset);
        Ok(())
    }

    pub(super) fn select_direction(&mut self, direction: Direction) -> DocumentResult<()> {
        self.view.current_sequence = Some(direction);
        self.select_current_keyframe().ok();
        Ok(())
    }

    pub(super) fn delete_selected_keyframes(&mut self) -> DocumentResult<()> {
        let mut selected_keyframes = self
            .view
            .selection
            .keyframes()
            .map(|(_, d, i)| (*d, *i))
            .collect::<Vec<_>>();
        selected_keyframes.sort();
        selected_keyframes.reverse();
        if let Ok((_, animation)) = self.get_workbench_animation_mut() {
            for (direction, index) in selected_keyframes {
                animation
                    .sequence_mut(direction)
                    .ok_or(DocumentError::SequenceNotInAnimation(direction))?
                    .delete_keyframe(index)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {

    use std::collections::HashMap;

    use super::*;

    #[test]
    fn can_toggle_playback() {
        let mut d = Document::new("tmp");
        d.sheet.add_frames(&vec!["walk_0", "walk_1", "walk_2"]);
        d.sheet.add_test_animation(
            "walk_cycle",
            HashMap::from([(Direction::North, vec!["walk_0", "walk_1", "walk_2"])]),
        );

        d.edit_animation("walk_cycle").unwrap();
        d.advance_timeline(Duration::from_millis(50));
        assert_eq!(d.timeline_clock().as_millis(), 0);
        d.play().unwrap();
        assert!(d.is_timeline_playing());
        d.advance_timeline(Duration::from_millis(100));
        assert_eq!(d.timeline_clock().as_millis(), 100);
        d.pause().unwrap();
        assert!(!d.is_timeline_playing());
        d.advance_timeline(Duration::from_millis(100));
        assert_eq!(d.timeline_clock().as_millis(), 100);
    }

    #[test]
    fn playback_stops_at_the_end_sequence() {
        let mut d = Document::new("tmp");
        d.sheet.add_frames(&vec!["walk_0", "walk_1", "walk_2"]);
        d.sheet.add_test_animation(
            "walk_cycle",
            HashMap::from([(Direction::North, vec!["walk_0", "walk_1", "walk_2"])]),
        );

        d.edit_animation("walk_cycle").unwrap();
        d.play().unwrap();
        d.advance_timeline(Duration::from_millis(500));
        assert_eq!(d.timeline_clock().as_millis(), 300);
        assert!(!d.is_timeline_playing());
    }

    #[test]
    fn can_loop_animation() {
        let mut d = Document::new("tmp");
        d.sheet.add_frames(&vec!["walk_0", "walk_1", "walk_2"]);
        d.sheet.add_test_animation(
            "walk_cycle",
            HashMap::from([(Direction::North, vec!["walk_0", "walk_1", "walk_2"])]),
        );

        d.edit_animation("walk_cycle").unwrap();
        d.set_animation_looping(true).unwrap();
        d.play().unwrap();
        assert_eq!(d.timeline_clock().as_millis(), 0);
        d.advance_timeline(Duration::from_millis(50));
        assert_eq!(d.timeline_clock().as_millis(), 50);
        d.advance_timeline(Duration::from_millis(100));
        assert_eq!(d.timeline_clock().as_millis(), 150);
        d.advance_timeline(Duration::from_millis(400));
        assert_eq!(d.timeline_clock().as_millis(), 250);
    }

    #[test]
    fn can_jump_to_animation_boundaries() {
        let mut d = Document::new("tmp");
        d.sheet.add_frames(&vec!["walk_0", "walk_1", "walk_2"]);
        d.sheet.add_test_animation(
            "walk_cycle",
            HashMap::from([(Direction::North, vec!["walk_0", "walk_1", "walk_2"])]),
        );

        d.edit_animation("walk_cycle").unwrap();
        d.advance_timeline(Duration::from_millis(50));

        d.jump_to_animation_start().unwrap();
        assert_eq!(d.timeline_clock().as_millis(), 0);

        d.jump_to_animation_end().unwrap();
        assert_eq!(d.timeline_clock().as_millis(), 300);
    }

    #[test]
    fn can_jump_to_next_or_previous_frame() {
        let mut d = Document::new("tmp");
        d.sheet.add_frames(&vec!["walk_0", "walk_1", "walk_2"]);
        d.sheet.add_test_animation(
            "walk_cycle",
            HashMap::from([(Direction::North, vec!["walk_0", "walk_1", "walk_2"])]),
        );

        d.edit_animation("walk_cycle").unwrap();
        d.advance_timeline(Duration::from_millis(50));

        d.jump_to_next_frame().unwrap();
        assert_eq!(d.timeline_clock().as_millis(), 100);
        d.jump_to_next_frame().unwrap();
        assert_eq!(d.timeline_clock().as_millis(), 200);
        d.jump_to_next_frame().unwrap();
        assert_eq!(d.timeline_clock().as_millis(), 300);
        d.jump_to_next_frame().unwrap();
        assert_eq!(d.timeline_clock().as_millis(), 300);
        d.jump_to_previous_frame().unwrap();
        assert_eq!(d.timeline_clock().as_millis(), 200);
        d.jump_to_previous_frame().unwrap();
        assert_eq!(d.timeline_clock().as_millis(), 100);
        d.jump_to_previous_frame().unwrap();
        assert_eq!(d.timeline_clock().as_millis(), 0);
        d.jump_to_previous_frame().unwrap();
        assert_eq!(d.timeline_clock().as_millis(), 0);
    }

    #[test]
    fn can_change_direction_preset() {
        let mut d = Document::new("tmp");
        d.sheet.add_frames(&vec!["walk_0", "walk_1", "walk_2"]);
        d.sheet.add_test_animation(
            "walk_cycle",
            HashMap::from([(Direction::North, vec!["walk_0", "walk_1", "walk_2"])]),
        );

        d.edit_animation("walk_cycle").unwrap();
        d.apply_direction_preset(DirectionPreset::EightDirections)
            .unwrap();

        assert_eq!(
            8,
            d.sheet
                .animation("walk_cycle")
                .unwrap()
                .sequences_iter()
                .count()
        );
    }

    #[test]
    fn can_delete_keyframes() {
        let mut d = Document::new("tmp");
        d.sheet.add_frames(&vec!["walk_0", "walk_1", "walk_2"]);
        d.sheet.add_test_animation(
            "walk_cycle",
            HashMap::from([(Direction::North, vec!["walk_0", "walk_1", "walk_2"])]),
        );

        d.edit_animation("walk_cycle").unwrap();
        d.select_keyframes_only(vec![
            ("walk_cycle".to_owned(), Direction::North, 1),
            ("walk_cycle".to_owned(), Direction::North, 2),
        ]);

        d.delete_selected_keyframes().unwrap();
        assert_eq!(
            1,
            d.sheet
                .sequence("walk_cycle", Direction::North)
                .num_keyframes()
        );
    }
}
