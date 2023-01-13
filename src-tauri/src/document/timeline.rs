use enum_iterator::{all, last, reverse_all};
use std::time::Duration;

use crate::document::*;
use crate::sheet::{Direction, DirectionPreset};

impl Document {
    pub fn advance_timeline(&mut self, delta: Duration) {
        if self.is_timeline_playing() {
            self.view.timeline_clock += delta;
            if let Ok((_, animation)) = self.workbench_animation() {
                if let Ok((_, sequence)) = self.workbench_sequence() {
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
        if self
            .workbench_sequence()?
            .1
            .duration_millis()
            .map(Duration::from_millis)
            .map(|d| d <= self.view.timeline_clock)
            .unwrap_or_default()
        {
            self.view.skip_to_timeline_start();
        }
        self.persistent.timeline_is_playing = true;
        self.view.selection.hitboxes.clear();
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
        let (_, sequence) = self.workbench_sequence()?;
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
        self.scrub_timeline(Duration::ZERO)?;
        self.view.reset_timeline_offset();
        Ok(())
    }

    pub(super) fn jump_to_animation_end(&mut self) -> DocumentResult<()> {
        let (_, sequence) = self.workbench_sequence()?;
        let duration = sequence.duration().unwrap_or_default();
        self.scrub_timeline(duration)
    }

    pub(super) fn jump_to_previous_frame(&mut self) -> DocumentResult<()> {
        let (_, sequence) = self.workbench_sequence()?;
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
        let (_, sequence) = self.workbench_sequence()?;
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

        let (_, animation) = self.workbench_animation()?;
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

        let (_, animation) = self.workbench_animation()?;
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
        let (_, animation) = self.workbench_animation_mut()?;
        animation.set_looping(is_looping);
        Ok(())
    }

    pub(super) fn apply_direction_preset(&mut self, preset: DirectionPreset) -> DocumentResult<()> {
        let (_, animation) = self.workbench_animation_mut()?;
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

        if selected_keyframes.is_empty() {
            return Ok(());
        }

        selected_keyframes.sort();
        selected_keyframes.reverse();
        let (direction, sequence) = self.workbench_sequence()?;
        let mut new_clock = self.view.timeline_clock.as_millis() as u64;

        let keyframes_ranges = sequence.keyframe_time_ranges();
        if let Ok((_, animation)) = self.workbench_animation_mut() {
            for (d, i) in selected_keyframes {
                animation
                    .sequence_mut(d)
                    .ok_or(DocumentError::SequenceNotInAnimation(d))?
                    .delete_keyframe(i)?;
                if d == direction {
                    let Some(range) = keyframes_ranges.get(i) else { continue };
                    if range.start <= new_clock {
                        let delta = range.end.min(new_clock) - range.start;
                        new_clock = new_clock.saturating_sub(delta);
                    }
                }
            }
        }

        self.view.timeline_clock = Duration::from_millis(new_clock);
        self.select_current_keyframe().ok();
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use std::collections::HashMap;

    use crate::mock::TigerAppMock;

    use super::*;

    #[tokio::test]
    async fn can_toggle_playback() {
        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        app.edit_animation("walk");

        app.tick(50.0);
        assert_eq!(app.client_state().documents[0].timeline_clock_millis, 0);

        app.play();
        assert!(app.client_state().documents[0].timeline_is_playing);

        app.tick(100.0);
        assert_eq!(app.client_state().documents[0].timeline_clock_millis, 100);

        app.pause();
        assert!(!app.client_state().documents[0].timeline_is_playing);

        app.tick(100.0);
        assert_eq!(app.client_state().documents[0].timeline_clock_millis, 100);
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
    fn play_from_end_of_sequence_starts_over() {
        let mut d = Document::new("tmp");
        d.sheet.add_frames(&vec!["walk_0", "walk_1", "walk_2"]);
        d.sheet.add_test_animation(
            "walk_cycle",
            HashMap::from([(Direction::North, vec!["walk_0", "walk_1", "walk_2"])]),
        );

        d.edit_animation("walk_cycle").unwrap();
        d.jump_to_animation_end().unwrap();
        assert_eq!(d.timeline_clock().as_millis(), 300);
        d.play().unwrap();
        assert_eq!(d.timeline_clock().as_millis(), 0);
    }

    #[test]
    fn scrubbing_blank_sequence_jumps_to_start() {
        let mut d = Document::new("tmp");
        d.sheet.add_test_animation::<_, &str>(
            "walk_cycle",
            HashMap::from([(Direction::North, vec![])]),
        );

        d.edit_animation("walk_cycle").unwrap();
        d.scrub_timeline(Duration::from_millis(500)).unwrap();
        assert_eq!(d.timeline_clock().as_millis(), 0);
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
        d.scrub_timeline(Duration::from_millis(50)).unwrap();
        assert_eq!(d.timeline_clock().as_millis(), 50);

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
        d.scrub_timeline(Duration::from_millis(50)).unwrap();
        assert_eq!(d.timeline_clock().as_millis(), 50);

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
    fn can_cycle_directions() {
        let mut d = Document::new("tmp");
        d.sheet.add_frames(&vec!["walk_0", "walk_1", "walk_2"]);
        d.sheet.add_test_animation(
            "walk_cycle",
            HashMap::from([
                (Direction::East, vec!["walk_0", "walk_1", "walk_2"]),
                (Direction::North, vec!["walk_0", "walk_1", "walk_2"]),
                (Direction::West, vec!["walk_0", "walk_1", "walk_2"]),
                (Direction::South, vec!["walk_0", "walk_1", "walk_2"]),
            ]),
        );

        d.edit_animation("walk_cycle").unwrap();
        d.select_direction(Direction::North).unwrap();

        assert_eq!(d.current_sequence().to_owned(), Some(Direction::North));
        d.cycle_directions_forward().unwrap();
        assert_eq!(d.current_sequence().to_owned(), Some(Direction::West));
        d.cycle_directions_forward().unwrap();
        assert_eq!(d.current_sequence().to_owned(), Some(Direction::South));
        d.cycle_directions_forward().unwrap();
        assert_eq!(d.current_sequence().to_owned(), Some(Direction::East));
        d.cycle_directions_forward().unwrap();
        assert_eq!(d.current_sequence().to_owned(), Some(Direction::North));
        d.cycle_directions_backward().unwrap();
        assert_eq!(d.current_sequence().to_owned(), Some(Direction::East));
        d.cycle_directions_backward().unwrap();
        assert_eq!(d.current_sequence().to_owned(), Some(Direction::South));
        d.cycle_directions_backward().unwrap();
        assert_eq!(d.current_sequence().to_owned(), Some(Direction::West));
        d.cycle_directions_backward().unwrap();
        assert_eq!(d.current_sequence().to_owned(), Some(Direction::North));
    }

    #[test]
    fn can_skip_gaps_when_cycling_directions() {
        let mut d = Document::new("tmp");
        d.sheet.add_frames(&vec!["walk_0", "walk_1", "walk_2"]);
        d.sheet.add_test_animation(
            "walk_cycle",
            HashMap::from([
                (Direction::East, vec!["walk_0", "walk_1", "walk_2"]),
                (Direction::North, vec!["walk_0", "walk_1"]),
                (Direction::West, vec!["walk_0", "walk_1", "walk_2"]),
                (Direction::South, vec!["walk_0", "walk_1", "walk_2"]),
            ]),
        );

        d.edit_animation("walk_cycle").unwrap();
        d.select_direction(Direction::East).unwrap();
        d.scrub_timeline(Duration::from_millis(250)).unwrap();
        assert_eq!(d.timeline_clock().as_millis(), 250);

        assert_eq!(d.current_sequence().to_owned(), Some(Direction::East));
        d.cycle_directions_forward().unwrap();
        assert_eq!(d.current_sequence().to_owned(), Some(Direction::West));
        d.cycle_directions_backward().unwrap();
        assert_eq!(d.current_sequence().to_owned(), Some(Direction::East));
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

        assert_eq!(
            d.selected_keyframes()
                .unwrap()
                .iter()
                .map(|(d, i, _)| (*d, *i))
                .collect::<Vec<_>>(),
            vec![(Direction::North, 0)]
        );
    }
}
