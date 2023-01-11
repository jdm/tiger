use euclid::default::{Rect, Vector2D};
use euclid::{point2, vec2};
use std::collections::{HashMap, HashSet};
use std::ops::Range;
use std::path::PathBuf;
use std::time::Duration;

use crate::document::*;
use crate::sheet::{Direction, Keyframe};

#[derive(Clone, Debug, PartialEq)]
pub(super) struct KeyframeDurationDrag {
    pub(super) frame_being_dragged: (Direction, usize),
    pub(super) original_ranges: HashMap<(Direction, usize), Range<u64>>,
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

#[derive(Clone, Debug)]
pub(super) enum Rename {
    Animation(String),
    Hitbox(String),
}

#[derive(Debug, Default)]
pub struct Transient {
    pub(super) rename: Option<Rename>,
    pub(super) frame_drag_and_drop: Option<PathBuf>,
    pub(super) keyframe_duration_drag: Option<KeyframeDurationDrag>,
    pub(super) keyframe_drag_and_drop: Option<(Direction, usize)>,
    pub(super) keyframe_nudge: Option<KeyframeNudge>,
    pub(super) hitbox_nudge: Option<HitboxNudge>,
    pub(super) hitbox_resize: Option<HitboxResize>,
}

impl Document {
    pub(super) fn begin_rename_animation(&mut self, animation_name: String) {
        self.transient.rename = Some(Rename::Animation(animation_name));
    }

    pub(super) fn end_rename_animation(&mut self, new_name: String) -> DocumentResult<()> {
        if let Some(Rename::Animation(old_name)) = self.transient.rename.clone() {
            self.transient.rename = None;
            self.rename_animation(old_name, new_name)
        } else {
            Err(DocumentError::NotRenamingAnyAnimation)
        }
    }

    pub(super) fn begin_rename_hitbox(&mut self, hitbox_name: String) {
        self.transient.rename = Some(Rename::Hitbox(hitbox_name));
    }

    pub(super) fn end_rename_hitbox(&mut self, new_name: String) -> DocumentResult<()> {
        if let Some(Rename::Hitbox(old_name)) = self.transient.rename.clone() {
            self.transient.rename = None;
            self.rename_hitbox(old_name, new_name)
        } else {
            Err(DocumentError::NotRenamingAnyHitbox)
        }
    }

    pub(super) fn cancel_rename(&mut self) {
        self.transient.rename = None;
    }

    pub fn animation_being_renamed(&self) -> Option<&String> {
        match self.transient.rename {
            Some(Rename::Animation(ref n)) => Some(n),
            _ => None,
        }
    }

    pub fn hitbox_being_renamed(&self) -> Option<&String> {
        match self.transient.rename {
            Some(Rename::Hitbox(ref n)) => Some(n),
            _ => None,
        }
    }

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
    ) -> DocumentResult<()> {
        let selected_frames = {
            let mut frames: Vec<PathBuf> = self.view.selection.frames().cloned().collect();
            frames.sort();
            frames.reverse();
            frames
        };

        let timeline_is_playing = self.persistent.timeline_is_playing;
        let (animation_name, animation) = self.workbench_animation_mut()?;
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

        self.transient.frame_drag_and_drop = None;
        Ok(())
    }

    pub(super) fn end_drag_and_drop_frame(&mut self) {
        self.transient.frame_drag_and_drop = None;
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
    ) -> DocumentResult<()> {
        let (animation_name, _) = self.workbench_animation()?;
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
    ) -> DocumentResult<()> {
        let timeline_is_playing = self.persistent.timeline_is_playing;

        // Sort affected keyframes
        let selection = {
            let (_, animation) = self.workbench_animation()?;
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

        let (animation_name, animation) = self.workbench_animation_mut()?;

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

        self.transient.keyframe_drag_and_drop = None;
        Ok(())
    }

    pub(super) fn end_drag_and_drop_keyframe(&mut self) {
        self.transient.keyframe_drag_and_drop = None;
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
    ) -> DocumentResult<()> {
        let (animation_name, _) = self.workbench_animation()?;
        let animation_name = animation_name.clone();
        if !self
            .view
            .selection
            .is_keyframe_selected(&animation_name, direction, index)
        {
            self.select_keyframe_only(animation_name.clone(), direction, index);
            self.view.current_sequence = Some(direction);
        }

        let (_, animation) = self.workbench_animation()?;

        self.transient.keyframe_duration_drag = Some(KeyframeDurationDrag {
            frame_being_dragged: (direction, index),
            original_ranges: animation
                .sequences_iter()
                .map(|(d, s)| (*d, s.keyframe_time_ranges()))
                .flat_map(|(d, r)| r.into_iter().enumerate().map(move |(i, r)| ((d, i), r)))
                .filter(|((d, i), _)| {
                    self.view
                        .selection
                        .is_keyframe_selected(&animation_name, *d, *i)
                })
                .collect(),
        });

        Ok(())
    }

    pub(super) fn update_drag_keyframe_duration(
        &mut self,
        delta_millis: i64,
    ) -> DocumentResult<()> {
        let drag_state = self
            .transient
            .keyframe_duration_drag
            .clone()
            .ok_or(DocumentError::NotDraggingKeyframeDuration)?;
        let (_, animation) = self.workbench_animation()?;
        let direction = drag_state.frame_being_dragged.0;
        let index = drag_state.frame_being_dragged.1;
        let zoom = self.timeline_zoom_factor();
        let apply_delta = |from: u64, delta: i64| {
            if delta > 0 {
                from.saturating_add(delta as u64)
            } else {
                from.saturating_sub(delta.unsigned_abs())
            }
        };

        let num_frames_affected = self
            .selected_keyframes()?
            .iter()
            .filter(|(d, i, _k)| direction == *d && index >= *i)
            .count()
            .max(1);

        let mut duration_delta_per_frame = delta_millis / num_frames_affected as i64;

        let original_range = drag_state
            .original_ranges
            .get(&(direction, index))
            .ok_or(DocumentError::MissingKeyframeDragData)?;

        if self.view.snap_keyframe_durations {
            let proposed_range = apply_delta(
                original_range.start,
                duration_delta_per_frame * (num_frames_affected as i64 - 1),
            )..apply_delta(original_range.end, delta_millis);
            let (snap_to, snapping_distance) = Self::snap_keyframe(
                animation,
                &drag_state,
                num_frames_affected,
                original_range,
                &proposed_range,
                self.view.snap_keyframes_to_other_keyframes,
                self.view
                    .snap_keyframes_to_multiples_of_duration
                    .then_some(self.view.keyframe_snapping_base_duration),
            );
            if (snapping_distance as f32) < (20.0 / zoom) {
                let snapped_delta_millis = snap_to as i64 - original_range.end as i64;
                duration_delta_per_frame = snapped_delta_millis / num_frames_affected as i64;
            }
        }

        let minimum_duration = 20.0 as u64;
        for (d, i, keyframe) in self.selected_keyframes_mut()? {
            let old_duration = drag_state
                .original_ranges
                .get(&(d, i))
                .map(|r| r.to_owned().count() as u64)
                .ok_or(DocumentError::MissingKeyframeDragData)?;
            let new_duration =
                apply_delta(old_duration, duration_delta_per_frame).max(minimum_duration);
            keyframe.set_duration_millis(new_duration);
        }

        Ok(())
    }

    fn snap_keyframe(
        animation: &Animation<Absolute>,
        drag_state: &KeyframeDurationDrag,
        num_frames_affected: usize,
        original_range: &Range<u64>,
        proposed_range: &Range<u64>,
        snap_to_other_keyframes: bool,
        snap_to_multiples_of: Option<Duration>,
    ) -> (u64, u64) {
        let mut candidates = vec![];

        if snap_to_other_keyframes {
            let snap = animation
                .sequences_iter()
                .flat_map(|(d, s)| {
                    s.keyframe_time_ranges()
                        .into_iter()
                        .enumerate()
                        .filter(|(i, _)| {
                            !drag_state
                                .original_ranges
                                .keys()
                                .any(|(od, oi)| *d == *od && oi <= i)
                        })
                        .map(|(_, r)| r.end)
                })
                .map(|t| (t, proposed_range.end.abs_diff(t)))
                .min_by_key(|(_, d)| *d);
            if let Some(snap) = snap {
                candidates.push(snap);
            }
        };

        if let Some(duration) = snap_to_multiples_of {
            let base = duration.as_millis().max(1) as u64;
            let original_duration = original_range.clone().count() as u64;
            let new_duration = proposed_range.clone().count() as u64;
            let snapped_duration = ((new_duration + base / 2) / base) * base;
            let snap_to = original_range.end as i64
                + num_frames_affected as i64 * (snapped_duration as i64 - original_duration as i64);
            let snap_to = snap_to.max(0) as u64;
            let snap_distance = snap_to.abs_diff(proposed_range.end);
            candidates.push((snap_to, snap_distance));
        }

        candidates
            .into_iter()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .unwrap_or((0, u64::MAX))
    }

    pub(super) fn end_drag_keyframe_duration(&mut self) {
        self.transient.keyframe_duration_drag = None;
    }

    pub fn is_dragging_keyframe_duration(&self) -> bool {
        self.transient.keyframe_duration_drag.is_some()
    }

    pub(super) fn begin_nudge_keyframe(
        &mut self,
        direction: Direction,
        index: usize,
    ) -> DocumentResult<()> {
        let (animation_name, _) = self.workbench_animation()?;

        if !self
            .view
            .selection
            .is_keyframe_selected(animation_name, direction, index)
        {
            self.select_keyframe_only(animation_name.clone(), direction, index);
        }

        let (_, animation) = self.workbench_animation()?;
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
    ) -> DocumentResult<()> {
        let zoom = self.workbench_zoom();
        let nudge = self
            .transient
            .keyframe_nudge
            .clone()
            .ok_or(DocumentError::NotNudgingKeyframe)?;

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

            let (_, animation) = self.workbench_animation_mut()?;
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

    pub(super) fn end_nudge_keyframe(&mut self) {
        self.transient.keyframe_nudge = None;
    }

    pub(super) fn nudge_keyframe<P: Paths>(keyframe: &mut Keyframe<P>, new_offset: Vector2D<i32>) {
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
    ) -> DocumentResult<()> {
        let (animation_name, _) = self.workbench_animation()?;
        let ((direction, index), _) = self.workbench_keyframe()?;

        if !self.view.selection.is_hitbox_selected(
            animation_name,
            direction,
            index,
            hitbox_name.as_ref(),
        ) {
            self.select_hitbox_only(
                animation_name.clone(),
                direction,
                index,
                hitbox_name.as_ref(),
            );
        }

        let (_, keyframe) = self.workbench_keyframe()?;
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
    ) -> DocumentResult<()> {
        let zoom = self.workbench_zoom();
        let nudge = self
            .transient
            .hitbox_nudge
            .clone()
            .ok_or(DocumentError::NotNudgingHitbox)?;

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

        let (_, keyframe) = self.workbench_keyframe_mut()?;
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

    pub(super) fn end_nudge_hitbox(&mut self) {
        self.transient.hitbox_nudge = None;
    }

    pub fn hitboxes_being_nudged(&self) -> HashSet<&str> {
        match self.transient.hitbox_nudge.is_some() {
            true => self
                .view
                .selection
                .hitboxes()
                .map(|(_, _, _, hitbox_name)| hitbox_name.as_str())
                .collect(),
            false => HashSet::new(),
        }
    }

    pub(super) fn begin_resize_hitbox<T: AsRef<str>>(
        &mut self,
        hitbox_name: T,
        axis: ResizeAxis,
    ) -> DocumentResult<()> {
        let (_, keyframe) = self.workbench_keyframe()?;
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
    ) -> DocumentResult<()> {
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
        let (_, keyframe) = self.workbench_keyframe_mut()?;

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

    pub(super) fn end_resize_hitbox(&mut self) {
        self.transient.hitbox_resize = None;
    }

    pub fn hitboxes_being_resized(&self) -> HashSet<&str> {
        match self.transient.hitbox_resize.is_some() {
            true => self
                .view
                .selection
                .hitboxes()
                .map(|(_, _, _, hitbox_name)| hitbox_name.as_str())
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

#[cfg(test)]
mod tests {

    use sugar_path::SugarPath;

    use super::*;
    use crate::dto;
    use crate::mock::TigerAppMock;

    #[test]
    fn can_rename_animations() {
        let mut d = Document::new("tmp");
        d.sheet
            .add_test_animation::<_, &Path>("walk_cycle", HashMap::new());
        d.select_animation("walk_cycle", false, false);
        d.begin_rename_selection();
        assert_eq!("walk_cycle", d.animation_being_renamed().unwrap().as_str());
        d.end_rename_animation("renamed".to_owned()).unwrap();
        assert_eq!(None, d.animation_being_renamed());

        assert!(d.sheet().animation("renamed").is_some());
    }

    #[test]
    fn can_rename_hitboxes() {
        let mut d = Document::new("tmp");
        d.sheet.add_frames(&vec!["walk_0", "walk_1", "walk_2"]);
        d.sheet.add_test_animation(
            "walk_cycle",
            HashMap::from([(Direction::North, vec!["walk_0", "walk_1", "walk_2"])]),
        );
        let keyframe = d.sheet.keyframe_mut("walk_cycle", Direction::North, 0);
        keyframe.create_hitbox("my_hitbox");

        d.edit_animation("walk_cycle").unwrap();
        d.select_hitbox("my_hitbox", false, false).unwrap();
        d.begin_rename_selection();
        assert_eq!("my_hitbox", d.hitbox_being_renamed().unwrap().as_str());
        d.end_rename_hitbox("renamed".to_owned()).unwrap();
        assert_eq!(None, d.hitbox_being_renamed());

        assert!(d
            .sheet()
            .animation("walk_cycle")
            .unwrap()
            .sequence(Direction::North)
            .unwrap()
            .keyframe(0)
            .unwrap()
            .has_hitbox("renamed"));
    }

    #[test]
    fn can_drag_and_drop_frame_to_timeline() {
        let app = TigerAppMock::new();
        app.new_document("tmp");
        app.import_frames(vec!["walk_0", "walk_1", "walk_2"]);
        app.create_animation();
        app.begin_drag_and_drop_frame("walk_1");
        app.drop_frame_on_timeline(dto::Direction::North, 0);
        app.begin_drag_and_drop_frame("walk_0");
        app.drop_frame_on_timeline(dto::Direction::North, 0);
        app.begin_drag_and_drop_frame("walk_2");
        app.drop_frame_on_timeline(dto::Direction::North, 2);

        let keyframes = app.client_state().documents[0].sheet.animations[0]
            .keyframes(dto::Direction::North)
            .iter()
            .map(|k| k.frame.to_owned())
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
        let app = TigerAppMock::new();
        app.new_document("tmp");
        app.import_frames(vec!["walk_0", "walk_2"]);
        app.create_animation();
        app.select_frame("walk_0", false, true);
        app.select_frame("walk_2", false, true);
        app.begin_drag_and_drop_frame("walk_0");
        app.drop_frame_on_timeline(dto::Direction::North, 0);

        let keyframes = app.client_state().documents[0].sheet.animations[0]
            .keyframes(dto::Direction::North)
            .iter()
            .map(|k| k.frame.to_owned())
            .collect::<Vec<_>>();

        assert_eq!(keyframes, vec![Path::new("walk_0"), Path::new("walk_2")]);
    }

    #[tokio::test]
    async fn keeps_track_of_frames_being_dragged() {
        let frame_0 = PathBuf::from("test-data/samurai-walk-south-0.png").resolve();
        let frame_1 = PathBuf::from("test-data/samurai-walk-south-1.png").resolve();

        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;

        app.select_frame(&frame_0, false, true);
        app.select_frame(&frame_1, false, true);
        assert!(app.client_state().documents[0]
            .frames_being_dragged
            .is_empty());

        app.begin_drag_and_drop_frame(&frame_0);
        assert_eq!(
            app.client_state().documents[0].frames_being_dragged,
            HashSet::from([frame_0, frame_1]),
        );

        app.end_drag_and_drop_frame();
        assert!(app.client_state().documents[0]
            .frames_being_dragged
            .is_empty());
    }

    #[tokio::test]
    async fn can_drag_and_drop_keyframe_to_reorder() {
        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        app.edit_animation("walk");
        app.begin_drag_and_drop_keyframe(dto::Direction::South, 1);
        app.drop_keyframe_on_timeline(dto::Direction::South, 0);

        let keyframes = app.client_state().documents[0]
            .keyframes("walk", dto::Direction::South)
            .iter()
            .map(|k| k.frame.to_owned())
            .collect::<Vec<_>>();

        assert_eq!(
            keyframes,
            vec![
                PathBuf::from("test-data/samurai-walk-south-1.png").resolve(),
                PathBuf::from("test-data/samurai-walk-south-0.png").resolve(),
                PathBuf::from("test-data/samurai-walk-south-2.png").resolve(),
                PathBuf::from("test-data/samurai-walk-south-3.png").resolve()
            ]
        );
    }

    #[tokio::test]
    async fn can_drag_and_drop_multiple_keyframes_to_reorder() {
        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        app.edit_animation("walk");
        app.select_keyframe(dto::Direction::South, 0, false, false);
        app.select_keyframe(dto::Direction::South, 1, false, true);
        app.begin_drag_and_drop_keyframe(dto::Direction::South, 1);
        app.drop_keyframe_on_timeline(dto::Direction::South, 3);

        let keyframes = app.client_state().documents[0]
            .keyframes("walk", dto::Direction::South)
            .iter()
            .map(|k| k.frame.to_owned())
            .collect::<Vec<_>>();

        assert_eq!(
            keyframes,
            vec![
                PathBuf::from("test-data/samurai-walk-south-2.png").resolve(),
                PathBuf::from("test-data/samurai-walk-south-0.png").resolve(),
                PathBuf::from("test-data/samurai-walk-south-1.png").resolve(),
                PathBuf::from("test-data/samurai-walk-south-3.png").resolve()
            ]
        );
    }

    #[tokio::test]
    async fn can_drag_and_drop_keyframes_to_different_direction() {
        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        app.edit_animation("walk");
        app.select_keyframe(dto::Direction::North, 1, false, false);
        app.select_keyframe(dto::Direction::North, 2, false, true);
        app.begin_drag_and_drop_keyframe(dto::Direction::North, 1);
        app.drop_keyframe_on_timeline(dto::Direction::South, 0);

        {
            let keyframes = app.client_state().documents[0]
                .keyframes("walk", dto::Direction::North)
                .iter()
                .map(|k| k.frame.to_owned())
                .collect::<Vec<_>>();

            assert_eq!(
                keyframes,
                vec![
                    PathBuf::from("test-data/samurai-walk-north-0.png").resolve(),
                    PathBuf::from("test-data/samurai-walk-north-3.png").resolve()
                ]
            );
        }

        {
            let keyframes = app.client_state().documents[0]
                .keyframes("walk", dto::Direction::South)
                .iter()
                .map(|k| k.frame.to_owned())
                .collect::<Vec<_>>();

            assert_eq!(
                keyframes,
                vec![
                    PathBuf::from("test-data/samurai-walk-north-1.png").resolve(),
                    PathBuf::from("test-data/samurai-walk-north-2.png").resolve(),
                    PathBuf::from("test-data/samurai-walk-south-0.png").resolve(),
                    PathBuf::from("test-data/samurai-walk-south-1.png").resolve(),
                    PathBuf::from("test-data/samurai-walk-south-2.png").resolve(),
                    PathBuf::from("test-data/samurai-walk-south-3.png").resolve()
                ]
            );
        }
    }

    #[tokio::test]
    async fn keeps_track_of_keyframes_being_dragged() {
        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        app.edit_animation("walk");

        app.select_keyframe(dto::Direction::North, 0, false, false);
        app.select_keyframe(dto::Direction::North, 1, false, true);
        assert!(app.client_state().documents[0]
            .keyframes_being_dragged
            .is_empty());

        app.begin_drag_and_drop_keyframe(dto::Direction::North, 1);
        assert_eq!(
            app.client_state().documents[0].keyframes_being_dragged,
            HashSet::from([(dto::Direction::North, 0), (dto::Direction::North, 1)])
        );

        app.end_drag_and_drop_keyframe();
        assert!(app.client_state().documents[0]
            .keyframes_being_dragged
            .is_empty());
    }

    #[test]
    fn can_drag_keyframe_duration() {
        let mut d = Document::new("tmp");
        d.sheet.add_frames(&vec!["walk_0", "walk_1", "walk_2"]);
        d.sheet.add_test_animation(
            "walk_cycle",
            HashMap::from([(Direction::North, vec!["walk_0", "walk_1", "walk_2"])]),
        );

        d.edit_animation("walk_cycle").unwrap();
        d.begin_drag_keyframe_duration(Direction::North, 1).unwrap();
        d.update_drag_keyframe_duration(50).unwrap();
        d.end_drag_keyframe_duration();

        let new_duration = d
            .sheet
            .keyframe("walk_cycle", Direction::North, 1)
            .duration_millis();
        assert_eq!(new_duration, 150);
    }

    #[test]
    fn drag_keyframe_duration_can_snap_to_other_keyframe() {
        let mut d = Document::new("tmp");
        d.sheet.add_frames(&vec!["walk_0", "walk_1", "walk_2"]);
        d.sheet.add_test_animation(
            "walk_cycle",
            HashMap::from([
                (Direction::North, vec!["walk_0", "walk_1", "walk_2"]),
                (Direction::South, vec!["walk_0", "walk_1", "walk_2"]),
            ]),
        );

        d.edit_animation("walk_cycle").unwrap();
        d.begin_drag_keyframe_duration(Direction::North, 1).unwrap();
        d.update_drag_keyframe_duration(99).unwrap();
        d.end_drag_keyframe_duration();
        let new_duration = d
            .sheet
            .keyframe("walk_cycle", Direction::North, 1)
            .duration_millis();

        assert_eq!(new_duration, 200);
    }

    #[test]
    fn drag_keyframe_duration_does_not_snap_to_moving_keyframes() {
        let mut d = Document::new("tmp");
        d.sheet.add_frames(&vec!["walk_0", "walk_1", "walk_2"]);
        d.sheet.add_test_animation(
            "walk_cycle",
            HashMap::from([
                (Direction::North, vec!["walk_0", "walk_1", "walk_2"]),
                (Direction::South, vec!["walk_0", "walk_1", "walk_2"]),
            ]),
        );

        d.edit_animation("walk_cycle").unwrap();
        d.select_keyframes_only(vec![
            ("walk_cycle".to_owned(), Direction::North, 0),
            ("walk_cycle".to_owned(), Direction::South, 2),
        ]);
        d.begin_drag_keyframe_duration(Direction::South, 2).unwrap();
        d.update_drag_keyframe_duration(49).unwrap();
        d.update_drag_keyframe_duration(50).unwrap();
        d.end_drag_keyframe_duration();
        let new_duration = d
            .sheet
            .keyframe("walk_cycle", Direction::South, 2)
            .duration_millis();

        assert_eq!(new_duration, 150);
    }

    #[test]
    fn drag_keyframe_duration_can_snap_to_duration() {
        let mut d = Document::new("tmp");
        d.sheet.add_frames(&vec!["walk_0", "walk_1", "walk_2"]);
        d.sheet.add_test_animation(
            "walk_cycle",
            HashMap::from([(Direction::North, vec!["walk_0", "walk_1", "walk_2"])]),
        );

        d.view.snap_keyframes_to_multiples_of_duration = true;
        d.view.keyframe_snapping_base_duration = Duration::from_millis(50);
        d.edit_animation("walk_cycle").unwrap();
        d.begin_drag_keyframe_duration(Direction::North, 1).unwrap();
        d.update_drag_keyframe_duration(49).unwrap();
        d.end_drag_keyframe_duration();
        let new_duration = d
            .sheet
            .keyframe("walk_cycle", Direction::North, 1)
            .duration_millis();

        assert_eq!(new_duration, 150);
    }

    #[test]
    fn can_drag_multiple_keyframe_durations() {
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
        d.begin_drag_keyframe_duration(Direction::North, 1).unwrap();
        d.update_drag_keyframe_duration(50).unwrap();
        d.end_drag_keyframe_duration();

        assert_eq!(
            d.sheet
                .keyframe("walk_cycle", Direction::North, 0)
                .duration_millis(),
            125
        );

        assert_eq!(
            d.sheet
                .keyframe("walk_cycle", Direction::North, 1)
                .duration_millis(),
            125
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

    #[test]
    fn can_nudge_keyframe() {
        let mut d = Document::new("tmp");
        d.sheet.add_frames(&vec!["walk_0", "walk_1", "walk_2"]);
        d.sheet.add_test_animation(
            "walk_cycle",
            HashMap::from([(Direction::North, vec!["walk_0", "walk_1", "walk_2"])]),
        );
        d.edit_animation("walk_cycle").unwrap();
        d.view.set_workbench_zoom_factor(1);

        let keyframe = d.sheet.keyframe_mut("walk_cycle", Direction::North, 0);
        keyframe.create_hitbox("my_hitbox");
        let initial = d
            .sheet
            .hitbox("walk_cycle", Direction::North, 0, "my_hitbox")
            .rectangle();

        d.begin_nudge_keyframe(Direction::North, 0).unwrap();
        d.update_nudge_keyframe(vec2(5, 10), false).unwrap();
        assert_eq!(
            d.sheet.keyframe("walk_cycle", Direction::North, 0).offset(),
            vec2(0, 10),
        );
        assert_eq!(
            d.sheet
                .hitbox("walk_cycle", Direction::North, 0, "my_hitbox")
                .rectangle(),
            euclid::rect(
                initial.origin.x,
                initial.origin.y + 10,
                initial.size.width,
                initial.size.height
            ),
        );
    }

    #[test]
    fn can_nudge_multiple_keyframes() {
        let mut d = Document::new("tmp");
        d.sheet.add_frames(&vec!["walk_0", "walk_1", "walk_2"]);
        d.sheet.add_test_animation(
            "walk_cycle",
            HashMap::from([(Direction::North, vec!["walk_0", "walk_1", "walk_2"])]),
        );
        d.edit_animation("walk_cycle").unwrap();
        d.view.set_workbench_zoom_factor(1);

        d.select_keyframes_only([
            ("walk_cycle".to_owned(), Direction::North, 0),
            ("walk_cycle".to_owned(), Direction::North, 1),
        ]);

        d.begin_nudge_keyframe(Direction::North, 0).unwrap();
        d.update_nudge_keyframe(vec2(5, 10), true).unwrap();
        assert_eq!(
            d.sheet.keyframe("walk_cycle", Direction::North, 0).offset(),
            vec2(5, 10),
        );
        assert_eq!(
            d.sheet.keyframe("walk_cycle", Direction::North, 1).offset(),
            vec2(5, 10),
        );
    }

    #[test]
    fn can_nudge_hitbox() {
        let mut d = Document::new("tmp");
        d.sheet.add_frames(&vec!["walk_0", "walk_1", "walk_2"]);
        d.sheet.add_test_animation(
            "walk_cycle",
            HashMap::from([(Direction::North, vec!["walk_0", "walk_1", "walk_2"])]),
        );
        d.edit_animation("walk_cycle").unwrap();
        d.view.set_workbench_zoom_factor(1);

        let keyframe = d.sheet.keyframe_mut("walk_cycle", Direction::North, 0);
        keyframe.create_hitbox("my_hitbox");
        let initial = d
            .sheet
            .hitbox("walk_cycle", Direction::North, 0, "my_hitbox")
            .rectangle();

        d.begin_nudge_hitbox("my_hitbox").unwrap();
        d.update_nudge_hitbox(vec2(5, 10), false).unwrap();
        d.end_nudge_hitbox();
        let hitbox = d
            .sheet
            .hitbox("walk_cycle", Direction::North, 0, "my_hitbox");
        assert_eq!(
            hitbox.rectangle(),
            euclid::rect(
                initial.origin.x,
                initial.origin.y + 10,
                initial.size.width,
                initial.size.height
            )
        );
    }

    #[test]
    fn keeps_track_of_hitboxes_being_nudged() {
        let mut d = Document::new("tmp");
        d.sheet.add_frames(&vec!["walk_0", "walk_1", "walk_2"]);
        d.sheet.add_test_animation(
            "walk_cycle",
            HashMap::from([(Direction::North, vec!["walk_0", "walk_1", "walk_2"])]),
        );
        d.edit_animation("walk_cycle").unwrap();
        d.view.set_workbench_zoom_factor(1);

        let keyframe = d.sheet.keyframe_mut("walk_cycle", Direction::North, 0);
        keyframe.create_hitbox("my_hitbox");

        assert!(d.hitboxes_being_nudged().is_empty());
        d.select_hitbox_only("walk_cycle", Direction::North, 0, "my_hitbox");
        d.begin_nudge_hitbox("my_hitbox").unwrap();
        assert_eq!(d.hitboxes_being_nudged(), HashSet::from(["my_hitbox"]));
        d.update_nudge_hitbox(vec2(5, 10), true).unwrap();
        d.end_nudge_hitbox();
        assert!(d.hitboxes_being_nudged().is_empty());
    }

    #[test]
    fn can_resize_hitbox() {
        use euclid::rect;

        let test_cases: Vec<(_, Vector2D<i32>, euclid::default::Rect<i32>)> = vec![
            (ResizeAxis::NW, vec2(10, 10), rect(10, 10, 90, 90)),
            (ResizeAxis::NE, vec2(10, 10), rect(0, 10, 110, 90)),
            (ResizeAxis::SW, vec2(10, 10), rect(10, 0, 90, 110)),
            (ResizeAxis::SE, vec2(10, 10), rect(0, 0, 110, 110)),
            (ResizeAxis::N, vec2(10, 10), rect(0, 10, 100, 90)),
            (ResizeAxis::W, vec2(10, 10), rect(10, 0, 90, 100)),
            (ResizeAxis::S, vec2(10, 10), rect(0, 0, 100, 110)),
            (ResizeAxis::E, vec2(10, 10), rect(0, 0, 110, 100)),
        ];

        for (axis, delta, expected) in test_cases {
            let mut d = Document::new("tmp");
            d.sheet.add_frames(&vec!["walk_0", "walk_1", "walk_2"]);
            d.sheet.add_test_animation(
                "walk_cycle",
                HashMap::from([(Direction::North, vec!["walk_0", "walk_1", "walk_2"])]),
            );
            d.edit_animation("walk_cycle").unwrap();
            d.view.set_workbench_zoom_factor(1);

            let keyframe = d.sheet.keyframe_mut("walk_cycle", Direction::North, 0);
            keyframe.create_hitbox("my_hitbox");
            let hitbox = d
                .sheet
                .hitbox_mut("walk_cycle", Direction::North, 0, "my_hitbox");
            hitbox.set_position(vec2(0, 0));
            hitbox.set_size(vec2(100, 100));

            d.select_hitbox_only("walk_cycle", Direction::North, 0, "my_hitbox");
            d.begin_resize_hitbox("my_hitbox", axis).unwrap();
            d.update_resize_hitbox(delta, false).unwrap();
            d.end_resize_hitbox();
            let hitbox = d
                .sheet
                .hitbox("walk_cycle", Direction::North, 0, "my_hitbox");
            assert_eq!(
                hitbox.rectangle(),
                euclid::rect(
                    expected.origin.x,
                    expected.origin.y,
                    expected.size.width,
                    expected.size.height
                )
            );
        }
    }

    #[test]
    fn can_resize_hitbox_while_preserving_aspect_ratio() {
        let mut d = Document::new("tmp");
        d.sheet.add_frames(&vec!["walk_0", "walk_1", "walk_2"]);
        d.sheet.add_test_animation(
            "walk_cycle",
            HashMap::from([(Direction::North, vec!["walk_0", "walk_1", "walk_2"])]),
        );
        d.edit_animation("walk_cycle").unwrap();
        d.view.set_workbench_zoom_factor(1);

        let keyframe = d.sheet.keyframe_mut("walk_cycle", Direction::North, 0);
        keyframe.create_hitbox("my_hitbox");
        let initial = d
            .sheet
            .hitbox("walk_cycle", Direction::North, 0, "my_hitbox")
            .rectangle();

        d.select_hitbox_only("walk_cycle", Direction::North, 0, "my_hitbox");
        d.begin_resize_hitbox("my_hitbox", ResizeAxis::SE).unwrap();
        d.update_resize_hitbox(vec2(40, 80), true).unwrap();
        d.end_resize_hitbox();
        let hitbox = d
            .sheet
            .hitbox("walk_cycle", Direction::North, 0, "my_hitbox");
        assert_eq!(
            hitbox.rectangle(),
            euclid::rect(
                initial.origin.x,
                initial.origin.y,
                initial.size.width + 80,
                initial.size.height + 80
            )
        );
    }

    #[test]
    fn keeps_track_of_hitboxes_being_resized() {
        let mut d = Document::new("tmp");
        d.sheet.add_frames(&vec!["walk_0", "walk_1", "walk_2"]);
        d.sheet.add_test_animation(
            "walk_cycle",
            HashMap::from([(Direction::North, vec!["walk_0", "walk_1", "walk_2"])]),
        );
        d.edit_animation("walk_cycle").unwrap();
        d.view.set_workbench_zoom_factor(1);

        let keyframe = d.sheet.keyframe_mut("walk_cycle", Direction::North, 0);
        keyframe.create_hitbox("my_hitbox");

        assert!(d.hitboxes_being_resized().is_empty());
        d.select_hitbox_only("walk_cycle", Direction::North, 0, "my_hitbox");
        d.begin_resize_hitbox("my_hitbox", ResizeAxis::SE).unwrap();
        assert_eq!(d.hitboxes_being_resized(), HashSet::from(["my_hitbox"]));
        d.update_resize_hitbox(vec2(40, 80), false).unwrap();
        d.end_resize_hitbox();
        assert!(d.hitboxes_being_resized().is_empty());
    }
}
