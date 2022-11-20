use euclid::default::{Rect, Vector2D};
use euclid::{point2, vec2};
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

#[derive(Clone, Debug, PartialEq)]
pub(super) struct KeyframeNudge {
    pub(super) keyframe_being_dragged: (Direction, usize),
    pub(super) original_positions: HashMap<(Direction, usize), Vector2D<i32>>,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct HitboxNudge {
    pub(super) hitbox_being_dragged: String,
    pub(super) original_positions: HashMap<String, Vector2D<i32>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResizeAxis {
    N,
    S,
    W,
    E,
    NW,
    NE,
    SE,
    SW,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct HitboxResize {
    pub(super) axis: ResizeAxis,
    pub(super) hitbox_being_dragged: String,
    pub(super) original_positions: HashMap<String, Rect<i32>>,
}

#[derive(Debug, Default)]
pub struct Transient {
    pub(super) frame_drag_and_drop: Option<PathBuf>,
    pub(super) keyframe_duration_drag: Option<KeyframeDurationDrag>,
    pub(super) keyframe_drag_and_drop: Option<(Direction, usize)>,
    pub(super) keyframe_nudge: Option<KeyframeNudge>,
    pub(super) hitbox_nudge: Option<HitboxNudge>,
    pub(super) hitbox_resize: Option<HitboxResize>,
}

impl Document {
    pub(super) fn begin_drag_and_drop_frame(&mut self, frame: PathBuf) {
        if !self.view.selection.is_frame_selected(&frame) {
            self.select_frame_only(frame.clone());
        }
        self.transient.frame_drag_and_drop = Some(frame);
    }

    pub(super) fn drop_frame_on_timeline(
        &mut self,
        direction: Direction,
        index: usize,
    ) -> Result<(), DocumentError> {
        let selected_frames = {
            let mut frames: Vec<PathBuf> = self.view.selection.frames().cloned().collect();
            frames.sort();
            frames.reverse();
            frames
        };

        let timeline_is_playing = self.persistent.timeline_is_playing;
        let (animation_name, animation) = self.get_workbench_animation_mut()?;
        let sequence = animation
            .sequence_mut(direction)
            .ok_or(DocumentError::SequenceNotInAnimation(direction))?;
        for frame in &selected_frames {
            let keyframe = Keyframe::new(frame);
            sequence.insert_keyframe(keyframe, index)?;
        }
        if !timeline_is_playing {
            self.view.timeline_clock = Duration::from_millis(sequence.keyframe_times()[index]);
        }
        self.select_keyframes_only(
            (index..(index + selected_frames.len()))
                .map(|i| (animation_name.clone(), direction, i)),
        );
        self.view.current_sequence = Some(direction);

        self.clear_transient();
        Ok(())
    }

    pub(super) fn end_drag_and_drop_frame(&mut self) {
        self.clear_transient();
    }

    pub fn frames_being_dragged(&self) -> HashSet<PathBuf> {
        match self.transient.frame_drag_and_drop.is_some() {
            true => self.view.selection.frames().cloned().collect(),
            false => HashSet::new(),
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
            self.select_keyframe_only(animation_name, direction, index);
        }
        self.transient.keyframe_drag_and_drop = Some((direction, index));
        Ok(())
    }

    pub(super) fn drop_keyframe_on_timeline(
        &mut self,
        direction: Direction,
        index: usize,
    ) -> Result<(), DocumentError> {
        let timeline_is_playing = self.persistent.timeline_is_playing;

        // Sort affected keyframes
        let selection = {
            let (_, animation) = self.get_workbench_animation()?;
            let mut selection: Vec<(Direction, usize)> = self
                .view
                .selection
                .keyframes()
                .map(|(_, d, i)| (*d, *i))
                .collect();
            let keyframe_times: HashMap<(Direction, usize), u64> = selection
                .iter()
                .filter_map(|(d, i)| {
                    animation
                        .sequence(*d)
                        .and_then(|sequence| sequence.keyframe_times().get(*i).copied())
                        .map(|t| ((*d, *i), t))
                })
                .collect();
            selection.sort_by_key(|(d, i)| {
                (
                    keyframe_times.get(&(*d, *i)).copied().unwrap_or_default(),
                    *d,
                )
            });
            selection
        };

        let (animation_name, animation) = self.get_workbench_animation_mut()?;

        // Remove keyframes from their current sequence
        let mut keyframes = Vec::with_capacity(selection.len());
        for (d, i) in selection.iter().rev() {
            let sequence = animation
                .sequence_mut(*d)
                .ok_or(DocumentError::SequenceNotInAnimation(*d))?;
            keyframes.push(sequence.delete_keyframe(*i)?);
        }

        // Insert keyframes at target location
        let num_affected_frames_before_insert_point = selection
            .iter()
            .filter(|(d, i)| *d == direction && *i < index)
            .count();
        let insert_index = index - num_affected_frames_before_insert_point;

        let sequence = animation
            .sequence_mut(direction)
            .ok_or(DocumentError::SequenceNotInAnimation(direction))?;
        for keyframe in keyframes {
            sequence.insert_keyframe(keyframe, insert_index)?;
        }

        // Update timeline position
        if !timeline_is_playing {
            let keyframe_times = sequence.keyframe_times();
            let timeline_pos = *keyframe_times
                .get(insert_index)
                .ok_or(DocumentError::NoKeyframeAtIndex(insert_index))?;
            self.view.timeline_clock = Duration::from_millis(timeline_pos);
        }

        // Update selection
        let new_selection = (insert_index..(insert_index + selection.len()))
            .map(|i| (animation_name.clone(), direction, i));
        self.select_keyframes_only(new_selection);
        self.view.current_sequence = Some(direction);

        self.clear_transient();
        Ok(())
    }

    pub(super) fn end_drag_and_drop_keyframe(&mut self) {
        self.clear_transient();
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
            self.select_keyframe_only(animation_name, direction, index);
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
            .ok_or(DocumentError::NotDraggingKeyframeDuration)?;

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
                .ok_or(DocumentError::MissingKeyframeDragData)?;
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

    pub(super) fn end_drag_keyframe_duration(&mut self) {
        self.clear_transient();
    }

    pub fn is_dragging_keyframe_duration(&self) -> bool {
        self.transient.keyframe_duration_drag.is_some()
    }

    pub(super) fn begin_nudge_keyframe(
        &mut self,
        direction: Direction,
        index: usize,
    ) -> Result<(), DocumentError> {
        let (_, animation) = self.get_workbench_animation()?;
        self.transient.keyframe_nudge = Some(KeyframeNudge {
            keyframe_being_dragged: (direction, index),
            original_positions: animation
                .sequences_iter()
                .flat_map(|(direction, sequence)| {
                    sequence
                        .keyframes_iter()
                        .enumerate()
                        .map(|(index, keyframe)| ((*direction, index), keyframe.offset()))
                })
                .collect(),
        });

        Ok(())
    }

    pub(super) fn update_nudge_keyframe(
        &mut self,
        mut displacement: Vector2D<i32>,
        both_axis: bool,
    ) -> Result<(), DocumentError> {
        let zoom = self.workbench_zoom();
        let nudge = self
            .transient
            .keyframe_nudge
            .clone()
            .ok_or(DocumentError::NotNudgingKeyframe)?;

        let (animation_name, _) = self.get_workbench_animation()?;
        let animation_name = animation_name.clone();

        // Select dragged frame
        if !self.view.selection.is_keyframe_selected(
            &animation_name,
            nudge.keyframe_being_dragged.0,
            nudge.keyframe_being_dragged.1,
        ) {
            self.select_keyframe_only(
                animation_name,
                nudge.keyframe_being_dragged.0,
                nudge.keyframe_being_dragged.1,
            );
        }

        if !both_axis {
            if displacement.x.abs() > displacement.y.abs() {
                displacement.y = 0;
            } else {
                displacement.x = 0;
            }
        }

        let affected_keyframes = self
            .view
            .selection
            .keyframes()
            .map(|(_, direction, index)| (*direction, *index))
            .collect::<Vec<_>>();

        for (direction, index) in affected_keyframes {
            let old_keyframe_offset = nudge
                .original_positions
                .get(&(direction, index))
                .ok_or(DocumentError::MissingKeyframePositionData)?;

            let (_, animation) = self.get_workbench_animation_mut()?;
            let keyframe = animation
                .sequence_mut(direction)
                .ok_or(DocumentError::SequenceNotInAnimation(direction))?
                .keyframe_mut(index)
                .ok_or(DocumentError::NoKeyframeAtIndex(index))?;

            let new_key_frame_offset = (old_keyframe_offset.to_f32()
                + displacement.to_f32() / zoom)
                .floor()
                .to_i32();
            Document::nudge_keyframe(keyframe, new_key_frame_offset);
        }

        Ok(())
    }

    pub(super) fn nudge_keyframe(keyframe: &mut Keyframe, new_offset: Vector2D<i32>) {
        let old_offset = keyframe.offset();
        keyframe.set_offset(new_offset);
        let displacement = new_offset - old_offset;
        for (_, hitbox) in keyframe.hitboxes_iter_mut() {
            hitbox.set_position(hitbox.position() + displacement);
        }
    }

    pub(super) fn begin_nudge_hitbox<T: AsRef<str>>(
        &mut self,
        hitbox_name: T,
    ) -> Result<(), DocumentError> {
        let (_, keyframe) = self.get_workbench_keyframe()?;
        self.transient.hitbox_nudge = Some(HitboxNudge {
            hitbox_being_dragged: hitbox_name.as_ref().to_owned(),
            original_positions: keyframe
                .hitboxes_iter()
                .map(|(n, hitbox)| (n.clone(), hitbox.position()))
                .collect(),
        });
        Ok(())
    }

    pub(super) fn update_nudge_hitbox(
        &mut self,
        mut displacement: Vector2D<i32>,
        both_axis: bool,
    ) -> Result<(), DocumentError> {
        let zoom = self.workbench_zoom();
        let nudge = self
            .transient
            .hitbox_nudge
            .clone()
            .ok_or(DocumentError::NotNudgingHitbox)?;

        let (animation_name, _) = self.get_workbench_animation()?;
        let animation_name = animation_name.clone();
        let ((direction, index), _) = self.get_workbench_keyframe()?;

        // Select dragged hitbox
        if !self.view.selection.is_hitbox_selected(
            &animation_name,
            direction,
            index,
            &nudge.hitbox_being_dragged,
        ) {
            self.select_hitbox_only(animation_name, direction, index, nudge.hitbox_being_dragged);
        }

        if !both_axis {
            if displacement.x.abs() > displacement.y.abs() {
                displacement.y = 0;
            } else {
                displacement.x = 0;
            }
        }

        let selected_hitboxes = self
            .view
            .selection
            .hitboxes()
            .map(|(_, _, _, hitbox_name)| hitbox_name.clone())
            .collect::<HashSet<_>>();

        let (_, keyframe) = self.get_workbench_keyframe_mut()?;
        for (hitbox_name, hitbox) in keyframe
            .hitboxes_iter_mut()
            .filter(|(hitbox_name, _)| selected_hitboxes.contains(*hitbox_name))
        {
            let old_position = nudge
                .original_positions
                .get(hitbox_name)
                .ok_or(DocumentError::MissingHitboxPositionData)?;
            let new_position = (old_position.to_f32() + displacement.to_f32() / zoom)
                .floor()
                .to_i32();
            hitbox.set_position(new_position);
        }

        Ok(())
    }

    pub fn hitboxes_being_nudged(&self) -> HashSet<&String> {
        match self.transient.hitbox_nudge.is_some() {
            true => self
                .view
                .selection
                .hitboxes()
                .map(|(_, _, _, hitbox_name)| hitbox_name)
                .collect(),
            false => HashSet::new(),
        }
    }

    pub(super) fn begin_resize_hitbox<T: AsRef<str>>(
        &mut self,
        hitbox_name: T,
        axis: ResizeAxis,
    ) -> Result<(), DocumentError> {
        let (_, keyframe) = self.get_workbench_keyframe()?;
        self.transient.hitbox_resize = Some(HitboxResize {
            axis,
            hitbox_being_dragged: hitbox_name.as_ref().to_owned(),
            original_positions: keyframe
                .hitboxes_iter()
                .map(|(n, hitbox)| (n.clone(), hitbox.rectangle()))
                .collect(),
        });
        Ok(())
    }

    pub(super) fn update_resize_hitbox(
        &mut self,
        mouse_delta: Vector2D<i32>,
        preserve_aspect_ratio: bool,
    ) -> Result<(), DocumentError> {
        use ResizeAxis::*;

        let resize = self
            .transient
            .hitbox_resize
            .clone()
            .ok_or(DocumentError::NotResizingHitbox)?;

        let selected_hitboxes = self
            .view
            .selection
            .hitboxes()
            .map(|(_, _, _, hitbox_name)| hitbox_name.clone())
            .collect::<HashSet<_>>();

        let zoom = self.workbench_zoom();
        let (_, keyframe) = self.get_workbench_keyframe_mut()?;

        for (hitbox_name, hitbox) in keyframe
            .hitboxes_iter_mut()
            .filter(|(hitbox_name, _)| selected_hitboxes.contains(*hitbox_name))
        {
            let old_rect = resize
                .original_positions
                .get(hitbox_name)
                .ok_or(DocumentError::MissingHitboxPositionData)?;

            let delta = if preserve_aspect_ratio && resize.axis.is_diagonal() {
                let aspect_ratio =
                    old_rect.size.width.max(1) as f32 / old_rect.size.height.max(1) as f32;
                let odd_axis_factor = if resize.axis == NE || resize.axis == SW {
                    -1.0
                } else {
                    1.0
                };
                if mouse_delta.x.abs() > mouse_delta.y.abs() {
                    vec2(
                        mouse_delta.x as f32,
                        odd_axis_factor * (mouse_delta.x as f32 / aspect_ratio).round(),
                    )
                } else {
                    vec2(
                        odd_axis_factor * (mouse_delta.y as f32 * aspect_ratio).round(),
                        mouse_delta.y as f32,
                    )
                }
            } else {
                mouse_delta.to_f32()
            };

            let delta = (delta / zoom).round().to_i32();

            let bottom_left = point2(old_rect.min_x(), old_rect.max_y());
            let top_right = point2(old_rect.max_x(), old_rect.min_y());

            let new_rect = Rect::from_points(match resize.axis {
                NW => vec![old_rect.max(), old_rect.origin + delta],
                NE => vec![bottom_left, top_right + delta],
                SW => vec![top_right, bottom_left + delta],
                SE => vec![old_rect.origin, old_rect.max() + delta],
                N => vec![
                    bottom_left,
                    point2(old_rect.max_x(), old_rect.min_y() + delta.y),
                ],
                W => vec![
                    top_right,
                    point2(old_rect.min_x() + delta.x, old_rect.max_y()),
                ],
                S => vec![
                    old_rect.origin,
                    point2(old_rect.max_x(), old_rect.max_y() + delta.y),
                ],
                E => vec![
                    old_rect.origin,
                    point2(old_rect.max_x() + delta.x, old_rect.max_y()),
                ],
            });

            hitbox.set_position(new_rect.origin.to_vector());
            hitbox.set_size(new_rect.size.to_u32().to_vector());
        }

        Ok(())
    }

    pub fn hitboxes_being_resized(&self) -> HashSet<&String> {
        match self.transient.hitbox_resize.is_some() {
            true => self
                .view
                .selection
                .hitboxes()
                .map(|(_, _, _, hitbox_name)| hitbox_name)
                .collect(),
            false => HashSet::new(),
        }
    }
}

impl ResizeAxis {
    pub fn is_diagonal(self) -> bool {
        use ResizeAxis::*;
        self == NW || self == NE || self == SW || self == SE
    }
}

#[test]
fn can_drag_and_drop_frame_to_timeline() {
    use std::path::Path;

    let mut d = Document::new("tmp");
    d.sheet.add_frames(&vec!["walk_0", "walk_1", "walk_2"]);
    d.sheet
        .add_test_animation::<_, &Path>("walk_cycle", HashMap::new());
    d.edit_animation("walk_cycle").unwrap();

    d.begin_drag_and_drop_frame(PathBuf::from("walk_1"));
    d.drop_frame_on_timeline(Direction::North, 0).unwrap();
    d.begin_drag_and_drop_frame(PathBuf::from("walk_0"));
    d.drop_frame_on_timeline(Direction::North, 0).unwrap();
    d.begin_drag_and_drop_frame(PathBuf::from("walk_2"));
    d.drop_frame_on_timeline(Direction::North, 2).unwrap();

    let animation = d.sheet.animation("walk_cycle").unwrap();
    let sequence = animation.sequence(Direction::North).unwrap();
    let keyframes = sequence
        .keyframes_iter()
        .map(|k| k.frame())
        .collect::<Vec<_>>();
    assert_eq!(
        keyframes,
        vec![
            Path::new("walk_0"),
            Path::new("walk_1"),
            Path::new("walk_2")
        ]
    );
}

#[test]
fn can_drag_and_drop_multiple_frames_to_timeline() {
    use std::path::Path;

    let mut d = Document::new("tmp");
    d.sheet.add_frames(&vec!["walk_0", "walk_2"]);
    d.sheet
        .add_test_animation::<_, &Path>("walk_cycle", HashMap::new());
    d.edit_animation("walk_cycle").unwrap();

    d.select_frame("walk_0", false, true);
    d.select_frame("walk_2", false, true);
    d.begin_drag_and_drop_frame(PathBuf::from("walk_0"));
    d.drop_frame_on_timeline(Direction::North, 0).unwrap();

    let animation = d.sheet.animation("walk_cycle").unwrap();
    let sequence = animation.sequence(Direction::North).unwrap();
    let keyframes = sequence
        .keyframes_iter()
        .map(|k| k.frame())
        .collect::<Vec<_>>();
    assert_eq!(keyframes, vec![Path::new("walk_0"), Path::new("walk_2")]);
}

#[test]
fn keeps_track_of_frames_being_dragged() {
    use std::path::Path;

    let mut d = Document::new("tmp");
    d.sheet.add_frames(&vec!["walk_0", "walk_1", "walk_2"]);
    d.sheet
        .add_test_animation::<_, &Path>("walk_cycle", HashMap::new());
    d.edit_animation("walk_cycle").unwrap();

    d.select_frame("walk_0", false, true);
    d.select_frame("walk_2", false, true);
    assert!(d.frames_being_dragged().is_empty());

    d.begin_drag_and_drop_frame(PathBuf::from("walk_0"));
    assert_eq!(
        d.frames_being_dragged(),
        HashSet::from([PathBuf::from("walk_0"), PathBuf::from("walk_2")]),
    );

    d.drop_frame_on_timeline(Direction::North, 0).unwrap();
    assert!(d.frames_being_dragged().is_empty());
}

#[test]
fn can_drag_and_drop_keyframe_to_reorder() {
    use std::path::Path;

    let mut d = Document::new("tmp");
    d.sheet.add_frames(&vec!["walk_0", "walk_1", "walk_2"]);
    d.sheet.add_test_animation(
        "walk_cycle",
        HashMap::from([(Direction::North, vec!["walk_0", "walk_1", "walk_2"])]),
    );
    d.edit_animation("walk_cycle").unwrap();

    d.begin_drag_and_drop_keyframe(Direction::North, 1).unwrap();
    d.drop_keyframe_on_timeline(Direction::North, 0).unwrap();

    let animation = d.sheet.animation("walk_cycle").unwrap();
    let sequence = animation.sequence(Direction::North).unwrap();
    let keyframes = sequence
        .keyframes_iter()
        .map(|k| k.frame())
        .collect::<Vec<_>>();
    assert_eq!(
        keyframes,
        vec![
            Path::new("walk_1"),
            Path::new("walk_0"),
            Path::new("walk_2")
        ]
    );
}

#[test]
fn can_drag_and_drop_multiple_keyframes_to_reorder() {
    use std::path::Path;

    let mut d = Document::new("tmp");
    d.sheet.add_frames(&vec!["walk_0", "walk_1", "walk_2"]);
    d.sheet.add_test_animation(
        "walk_cycle",
        HashMap::from([(Direction::North, vec!["walk_0", "walk_1", "walk_2"])]),
    );
    d.edit_animation("walk_cycle").unwrap();

    d.select_keyframes_only([
        ("walk_cycle".to_owned(), Direction::North, 0),
        ("walk_cycle".to_owned(), Direction::North, 1),
    ]);
    d.begin_drag_and_drop_keyframe(Direction::North, 1).unwrap();
    d.drop_keyframe_on_timeline(Direction::North, 3).unwrap();

    let animation = d.sheet.animation("walk_cycle").unwrap();
    let sequence = animation.sequence(Direction::North).unwrap();
    let keyframes = sequence
        .keyframes_iter()
        .map(|k| k.frame())
        .collect::<Vec<_>>();
    assert_eq!(
        keyframes,
        vec![
            Path::new("walk_2"),
            Path::new("walk_0"),
            Path::new("walk_1")
        ]
    );
}

#[test]
fn can_drag_and_drop_keyframes_to_different_direction() {
    use std::path::Path;

    let mut d = Document::new("tmp");
    d.sheet.add_frames(&vec!["walk_0", "walk_1", "walk_2"]);
    d.sheet.add_test_animation(
        "walk_cycle",
        HashMap::from([
            (Direction::North, vec!["walk_0", "walk_1", "walk_2"]),
            (Direction::South, vec![]),
        ]),
    );
    d.edit_animation("walk_cycle").unwrap();

    d.select_keyframes_only([
        ("walk_cycle".to_owned(), Direction::North, 1),
        ("walk_cycle".to_owned(), Direction::North, 2),
    ]);
    d.begin_drag_and_drop_keyframe(Direction::North, 1).unwrap();
    d.drop_keyframe_on_timeline(Direction::South, 0).unwrap();

    let animation = d.sheet.animation("walk_cycle").unwrap();
    {
        let sequence = animation.sequence(Direction::North).unwrap();
        let keyframes = sequence
            .keyframes_iter()
            .map(|k| k.frame())
            .collect::<Vec<_>>();
        assert_eq!(keyframes, vec![Path::new("walk_0"),]);
    }
    {
        let sequence = animation.sequence(Direction::South).unwrap();
        let keyframes = sequence
            .keyframes_iter()
            .map(|k| k.frame())
            .collect::<Vec<_>>();
        assert_eq!(keyframes, vec![Path::new("walk_1"), Path::new("walk_2")]);
    }
}

#[test]
fn keeps_track_of_keyframes_being_dragged() {
    let mut d = Document::new("tmp");
    d.sheet.add_frames(&vec!["walk_0", "walk_1", "walk_2"]);
    d.sheet.add_test_animation(
        "walk_cycle",
        HashMap::from([(Direction::North, vec!["walk_0", "walk_1", "walk_2"])]),
    );
    d.edit_animation("walk_cycle").unwrap();

    d.select_keyframes_only([
        ("walk_cycle".to_owned(), Direction::North, 0),
        ("walk_cycle".to_owned(), Direction::North, 1),
    ]);
    assert!(d.keyframes_being_dragged().is_empty());
    d.begin_drag_and_drop_keyframe(Direction::North, 1).unwrap();
    assert_eq!(
        d.keyframes_being_dragged(),
        HashSet::from([(Direction::North, 0), (Direction::North, 1)])
    );
    d.drop_keyframe_on_timeline(Direction::North, 3).unwrap();
    assert!(d.keyframes_being_dragged().is_empty());
}

#[test]
fn can_drag_keyframe_duration() {
    let mut d = Document::new("tmp");
    d.sheet.add_frames(&vec!["walk_0", "walk_1", "walk_2"]);
    d.sheet.add_test_animation(
        "walk_cycle",
        HashMap::from([(Direction::North, vec!["walk_0", "walk_1", "walk_2"])]),
    );

    let old_duration = d
        .sheet
        .keyframe("walk_cycle", Direction::North, 0)
        .duration_millis();

    d.edit_animation("walk_cycle").unwrap();
    d.begin_drag_keyframe_duration(Direction::North, 0).unwrap();
    d.update_drag_keyframe_duration(50).unwrap();
    d.end_drag_keyframe_duration();
    let new_duration = d
        .sheet
        .keyframe("walk_cycle", Direction::North, 0)
        .duration_millis();

    assert_eq!(new_duration, old_duration + 50);
}

#[test]
fn can_drag_multiple_keyframe_durations() {
    let mut d = Document::new("tmp");
    d.sheet.add_frames(&vec!["walk_0", "walk_1", "walk_2"]);
    d.sheet.add_test_animation(
        "walk_cycle",
        HashMap::from([(Direction::North, vec!["walk_0", "walk_1", "walk_2"])]),
    );

    let old_duration = d
        .sheet
        .keyframe("walk_cycle", Direction::North, 0)
        .duration_millis();

    d.edit_animation("walk_cycle").unwrap();
    d.select_keyframes_only([
        ("walk_cycle".to_owned(), Direction::North, 0),
        ("walk_cycle".to_owned(), Direction::North, 1),
    ]);
    d.begin_drag_keyframe_duration(Direction::North, 1).unwrap();
    d.update_drag_keyframe_duration(50).unwrap();
    d.end_drag_keyframe_duration();

    assert_eq!(
        d.sheet
            .keyframe("walk_cycle", Direction::North, 0)
            .duration_millis(),
        old_duration + 25
    );

    assert_eq!(
        d.sheet
            .keyframe("walk_cycle", Direction::North, 1)
            .duration_millis(),
        old_duration + 25
    );
}

#[test]
fn keeps_track_of_keyframe_durations_being_dragged() {
    let mut d = Document::new("tmp");
    d.sheet.add_frames(&vec!["walk_0", "walk_1", "walk_2"]);
    d.sheet.add_test_animation(
        "walk_cycle",
        HashMap::from([(Direction::North, vec!["walk_0", "walk_1", "walk_2"])]),
    );
    d.edit_animation("walk_cycle").unwrap();

    d.select_keyframes_only([
        ("walk_cycle".to_owned(), Direction::North, 0),
        ("walk_cycle".to_owned(), Direction::North, 1),
    ]);

    assert!(!d.is_dragging_keyframe_duration());
    d.begin_drag_keyframe_duration(Direction::North, 1).unwrap();
    assert!(d.is_dragging_keyframe_duration());
    d.update_drag_keyframe_duration(50).unwrap();
    assert!(d.is_dragging_keyframe_duration());
    d.end_drag_keyframe_duration();
    assert!(!d.is_dragging_keyframe_duration());
}
