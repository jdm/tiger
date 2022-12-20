use enum_iterator::{all, reverse_all};
use euclid::default::Vector2D;
use euclid::vec2;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::document::*;
use crate::sheet::{Animation, Direction};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SelectionState {
    pub(super) frames: Selection<PathBuf>,
    pub(super) animations: Selection<String>,
    pub(super) hitboxes: Selection<(String, Direction, usize, String)>,
    pub(super) keyframes: Selection<(String, Direction, usize)>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NudgeDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
pub enum BrowseDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Selection<T>
where
    T: std::cmp::Eq + std::hash::Hash + std::clone::Clone,
{
    pivot: Option<T>,
    last_interacted: Option<T>,
    selected_items: HashSet<T>,
}

impl Document {
    pub(super) fn delete_selection(&mut self) -> DocumentResult<()> {
        self.delete_selected_frames();
        self.delete_selected_animations();
        self.delete_selected_keyframes()?;
        self.delete_selected_hitboxes()?;
        Ok(())
    }

    pub(super) fn begin_rename_selection(&mut self) {
        if let Some(ref name) = self.view.selection.animations.last_interacted {
            self.begin_rename_animation(name.clone());
        } else if let Some((_, _, _, ref name)) = self.view.selection.hitboxes.last_interacted {
            self.begin_rename_hitbox(name.clone());
        }
    }

    pub(super) fn nudge_selection(
        &mut self,
        direction: NudgeDirection,
        large_nudge: bool,
    ) -> DocumentResult<()> {
        let mut delta = match direction {
            NudgeDirection::Up => vec2(0, -1),
            NudgeDirection::Down => vec2(0, 1),
            NudgeDirection::Left => vec2(-1, 0),
            NudgeDirection::Right => vec2(1, 0),
        };
        if large_nudge {
            delta *= 10;
        }

        for (_, _, keyframe) in self.selected_keyframes_mut()? {
            Document::nudge_keyframe(keyframe, keyframe.offset() + delta);
        }

        for (_, hitbox) in self.selected_hitboxes_mut()? {
            hitbox.set_position(hitbox.position() + delta);
        }

        Ok(())
    }

    pub fn select_frame_only(&mut self, frame: PathBuf) {
        self.view.selection.clear();
        self.view.selection.frames.only(vec![frame]);
    }

    pub fn select_frames_only<T>(&mut self, frames: T)
    where
        T: IntoIterator<Item = PathBuf>,
    {
        self.view.selection.clear();
        self.view.selection.frames.only(frames);
    }

    pub fn select_animation_only(&mut self, animation: String) {
        self.view.selection.clear();
        self.view.selection.animations.only(vec![animation]);
    }

    pub fn select_animations_only<T>(&mut self, animations: T)
    where
        T: IntoIterator<Item = String>,
    {
        self.view.selection.clear();
        self.view.selection.animations.only(animations);
    }

    pub fn select_keyframe_only(&mut self, animation: String, direction: Direction, index: usize) {
        self.view.selection.clear();
        self.view
            .selection
            .keyframes
            .only(vec![(animation, direction, index)]);
    }

    pub fn select_keyframes_only<T>(&mut self, keyframes: T)
    where
        T: IntoIterator<Item = (String, Direction, usize)>,
    {
        self.view.selection.clear();
        self.view.selection.keyframes.only(keyframes);
    }

    pub fn select_hitbox_only<T: AsRef<str>, U: AsRef<str>>(
        &mut self,
        animation: T,
        direction: Direction,
        index: usize,
        hitbox: U,
    ) {
        self.view.selection.clear();
        self.view.selection.hitboxes.only(vec![(
            animation.as_ref().to_owned(),
            direction,
            index,
            hitbox.as_ref().to_owned(),
        )]);
    }

    pub fn select_hitboxes_only<T>(&mut self, hitboxes: T)
    where
        T: IntoIterator<Item = (String, Direction, usize, String)>,
    {
        self.view.selection.clear();
        self.view.selection.hitboxes.only(hitboxes);
    }

    pub(super) fn select_all(&mut self) -> DocumentResult<()> {
        let mut new_selection = SelectionState::default();
        if !self.view.selection.frames.is_empty() {
            new_selection.frames.only(self.selectable_frames());
        } else if !self.view.selection.animations.is_empty() {
            new_selection.animations.only(self.selectable_animations());
        } else if !self.view.selection.hitboxes.is_empty() {
            new_selection.hitboxes.only(self.selectable_hitboxes()?);
        } else {
            let (animation_name, animation) = self.workbench_animation()?;
            let keyframes: Vec<(String, Direction, usize)> = animation
                .sequences_iter()
                .flat_map(|(d, s)| {
                    s.keyframes_iter()
                        .enumerate()
                        .map(|(i, _)| (animation_name.clone(), *d, i))
                })
                .collect();
            new_selection.keyframes.only(keyframes);
        }
        self.view.selection = new_selection;
        Ok(())
    }

    pub(super) fn select_frame<T: AsRef<Path>>(&mut self, path: T, shift: bool, ctrl: bool) {
        self.view.selection.animations.clear();
        self.view.selection.keyframes.clear();
        self.view.selection.hitboxes.clear();

        self.view.selection.frames.alter(
            path.as_ref().to_owned(),
            &self.selectable_frames(),
            shift,
            ctrl,
        );
    }

    pub(super) fn select_animation<T: AsRef<str>>(&mut self, name: T, shift: bool, ctrl: bool) {
        self.view.selection.frames.clear();
        self.view.selection.keyframes.clear();
        self.view.selection.hitboxes.clear();

        self.view.selection.animations.alter(
            name.as_ref().to_owned(),
            &self.selectable_animations(),
            shift,
            ctrl,
        );
    }

    pub(super) fn select_keyframe(
        &mut self,
        direction: Direction,
        index: usize,
        shift: bool,
        ctrl: bool,
    ) -> DocumentResult<()> {
        self.select_keyframe_internal(direction, index, shift, ctrl)?;

        if !self.persistent.timeline_is_playing {
            let (_, sequence) = self.workbench_sequence()?;
            self.view.timeline_clock = Duration::from_millis(
                sequence
                    .keyframe_times()
                    .get(index)
                    .copied()
                    .unwrap_or_default(),
            );
        }
        Ok(())
    }

    fn select_keyframe_internal(
        &mut self,
        direction: Direction,
        index: usize,
        shift: bool,
        ctrl: bool,
    ) -> DocumentResult<()> {
        self.view.selection.frames.clear();
        self.view.selection.animations.clear();
        self.view.selection.hitboxes.clear();

        self.view.current_sequence = Some(direction);
        let (animation_name, _) = self.workbench_animation()?;
        let animation_name = animation_name.clone();
        let animation = self
            .sheet
            .animation(&animation_name)
            .ok_or_else(|| DocumentError::AnimationNotInDocument(animation_name.clone()))?;

        self.view.selection.keyframes.alter(
            (animation_name, direction, index),
            animation,
            shift,
            ctrl,
        );

        Ok(())
    }

    pub(super) fn select_current_keyframe(&mut self) -> DocumentResult<()> {
        let (animation_name, _) = self.workbench_animation()?;
        let animation_name = animation_name.clone();
        let ((direction, keyframe_index), _) = self.workbench_keyframe()?;
        self.select_keyframe_only(animation_name, direction, keyframe_index);
        Ok(())
    }

    pub(super) fn select_hitbox<T: AsRef<str>>(
        &mut self,
        name: T,
        shift: bool,
        ctrl: bool,
    ) -> DocumentResult<()> {
        self.view.selection.frames.clear();
        self.view.selection.animations.clear();
        self.view.selection.keyframes.clear();

        let (animation_name, _) = self.workbench_animation()?;
        let animation_name = animation_name.clone();
        let ((direction, index), _) = self.workbench_keyframe()?;
        self.view.selection.hitboxes.alter(
            (animation_name, direction, index, name.as_ref().to_owned()),
            &self.selectable_hitboxes()?,
            shift,
            ctrl,
        );

        Ok(())
    }

    pub(super) fn browse_selection(
        &mut self,
        direction: BrowseDirection,
        shift: bool,
    ) -> DocumentResult<()> {
        if !self.view.selection.frames.is_empty() {
            self.browse_frames(direction, shift);
        } else if !self.view.selection.animations.is_empty() {
            self.browse_animations(direction, shift);
        } else if !self.view.selection.hitboxes.is_empty() {
            self.browse_hitboxes(direction, shift)?;
        } else if shift {
            self.browse_keyframes(direction, shift)?;
        } else {
            match direction {
                BrowseDirection::Left => self.jump_to_previous_frame()?,
                BrowseDirection::Right => self.jump_to_next_frame()?,
                BrowseDirection::Up => self.cycle_directions_backward()?,
                BrowseDirection::Down => self.cycle_directions_forward()?,
            };
        }
        Ok(())
    }

    pub(super) fn browse_to_end(&mut self, shift: bool) -> DocumentResult<()> {
        if !self.view.selection.frames.is_empty() {
            if let Some(frame) = self.selectable_frames().last() {
                self.select_frame(frame, shift, false);
            }
        } else if !self.view.selection.animations.is_empty() {
            if let Some(animation) = self.selectable_animations().last() {
                self.select_animation(animation, shift, false);
            }
        } else if !self.view.selection.hitboxes.is_empty() {
            if let Some(hitbox) = self.selectable_hitboxes()?.last() {
                self.select_hitbox(hitbox.3.clone(), shift, false)?;
            }
        } else if shift {
            let (direction, sequence) = self.workbench_sequence()?;
            let index = match sequence.num_keyframes() {
                0 => None,
                n => Some(n - 1),
            }
            .ok_or(DocumentError::SequenceHasNoKeyframes)?;
            self.select_keyframe(direction, index, shift, false)?;
        } else {
            self.jump_to_animation_end()?;
        }
        Ok(())
    }

    pub(super) fn browse_to_start(&mut self, shift: bool) -> DocumentResult<()> {
        if !self.view.selection.frames.is_empty() {
            if let Some(frame) = self.selectable_frames().first() {
                self.select_frame(frame, shift, false);
            }
        } else if !self.view.selection.animations.is_empty() {
            if let Some(animation) = self.selectable_animations().first() {
                self.select_animation(animation, shift, false);
            }
        } else if !self.view.selection.hitboxes.is_empty() {
            if let Some(hitbox) = self.selectable_hitboxes()?.first() {
                self.select_hitbox(hitbox.3.clone(), shift, false)?;
            }
        } else if shift {
            let (direction, sequence) = self.workbench_sequence()?;
            let index = match sequence.num_keyframes() {
                0 => None,
                _ => Some(0),
            }
            .ok_or(DocumentError::SequenceHasNoKeyframes)?;
            self.select_keyframe(direction, index, shift, false)?;
        } else {
            self.jump_to_animation_start()?;
        }
        Ok(())
    }

    fn browse_frames(&mut self, direction: BrowseDirection, shift: bool) {
        let item_pool = self.selectable_frames();
        let delta = direction.as_list_offset(self.view.frames_list_mode);
        if let Some(interacted_item) =
            (&item_pool).offset_from(self.view.selection.frames.last_interacted.as_ref(), delta)
        {
            self.select_frame(interacted_item, shift, false);
        }
    }

    fn browse_animations(&mut self, direction: BrowseDirection, shift: bool) {
        let item_pool = self.selectable_animations();
        let delta = direction.as_list_offset(ListMode::Linear);
        if let Some(interacted_item) = (&item_pool).offset_from(
            self.view.selection.animations.last_interacted.as_ref(),
            delta,
        ) {
            self.select_animation(interacted_item, shift, false);
        }
    }

    fn browse_hitboxes(&mut self, direction: BrowseDirection, shift: bool) -> DocumentResult<()> {
        let item_pool = self.selectable_hitboxes()?;
        let delta = direction.as_list_offset(ListMode::Linear);
        if let Some((_, _, _, hitbox_name)) =
            (&item_pool).offset_from(self.view.selection.hitboxes.last_interacted.as_ref(), delta)
        {
            self.select_hitbox(hitbox_name, shift, false)?;
        }
        Ok(())
    }

    fn browse_keyframes(
        &mut self,
        browse_direction: BrowseDirection,
        shift: bool,
    ) -> DocumentResult<()> {
        let from_keyframe = {
            let (animation_name, _) = self.workbench_animation()?;
            let animation_name = animation_name.clone();
            let ((direction, index), _) = self.workbench_keyframe()?;
            match self.view.selection.keyframes.last_interacted.as_ref() {
                Some(k) => k.to_owned(),
                None => {
                    self.select_keyframe_only(animation_name.clone(), direction, index);
                    (animation_name, direction, index)
                }
            }
        };

        let (_, pivot_direction, pivot_index) = self
            .view
            .selection
            .keyframes
            .pivot
            .clone()
            .unwrap_or_else(|| from_keyframe.clone());

        let keyframe_ranges = self
            .workbench_animation()?
            .1
            .sequences_iter()
            .map(|(d, s)| (*d, s.keyframe_time_ranges()))
            .collect::<HashMap<_, _>>();

        let (_, animation) = self.workbench_animation()?;
        let to_keyframe = animation.offset_from(Some(&from_keyframe), browse_direction);
        if let Some((_, direction, index)) = to_keyframe {
            self.select_keyframe_internal(direction, index, shift, false)?;

            let pivot_range = keyframe_ranges
                .get(&pivot_direction)
                .ok_or(DocumentError::SequenceNotInAnimation(pivot_direction))?
                .get(pivot_index)
                .ok_or(DocumentError::NoKeyframeAtIndex(pivot_index))?;

            let to_keyframe_range = keyframe_ranges
                .get(&direction)
                .ok_or(DocumentError::SequenceNotInAnimation(direction))?
                .get(index)
                .ok_or(DocumentError::NoKeyframeAtIndex(index))?;

            let new_clock = if to_keyframe_range.start <= pivot_range.start {
                to_keyframe_range.start
            } else {
                to_keyframe_range.end - 1
            };

            self.view.timeline_clock = Duration::from_millis(new_clock);
        }

        Ok(())
    }

    fn selectable_frames(&self) -> Vec<PathBuf> {
        self.sheet
            .sorted_frames()
            .into_iter()
            .map(|f| f.source().to_owned())
            .collect()
    }

    fn selectable_animations(&self) -> Vec<String> {
        self.sheet
            .sorted_animations()
            .into_iter()
            .map(|(n, _)| n.clone())
            .collect()
    }

    fn selectable_hitboxes(&self) -> DocumentResult<Vec<(String, Direction, usize, String)>> {
        let (animation_name, _) = self.workbench_animation()?;
        let ((direction, index), keyframe) = self.workbench_keyframe()?;
        Ok(keyframe
            .sorted_hitboxes()
            .into_iter()
            .map(|(n, _)| (animation_name.clone(), direction, index, n.clone()))
            .collect())
    }
}

impl SelectionState {
    pub fn clear(&mut self) {
        *self = Default::default();
    }

    pub fn is_frame_selected(&self, path: &Path) -> bool {
        self.frames.contains(path)
    }

    pub fn is_animation_selected<T: AsRef<str>>(&self, name: T) -> bool {
        self.animations.contains(name.as_ref())
    }

    pub fn is_hitbox_selected<T: AsRef<str>, U: AsRef<str>>(
        &self,
        animation_name: T,
        direction: Direction,
        index: usize,
        hitbox_name: U,
    ) -> bool {
        self.hitboxes.contains(
            (
                animation_name.as_ref(),
                direction,
                index,
                hitbox_name.as_ref(),
            )
                .borrow() as &dyn HitboxID,
        )
    }

    pub fn is_keyframe_selected<T: AsRef<str>>(
        &self,
        animation_name: T,
        direction: Direction,
        index: usize,
    ) -> bool {
        self.keyframes
            .contains((animation_name.as_ref(), direction, index).borrow() as &dyn KeyframeID)
    }

    pub fn last_interacted_frame(&self) -> &Option<PathBuf> {
        &self.frames.last_interacted
    }

    pub fn last_interacted_animation(&self) -> &Option<String> {
        &self.animations.last_interacted
    }

    pub fn last_interacted_hitbox(&self) -> &Option<(String, Direction, usize, String)> {
        &self.hitboxes.last_interacted
    }

    pub fn frames(&self) -> impl Iterator<Item = &PathBuf> {
        self.frames.iter()
    }

    pub fn animations(&self) -> impl Iterator<Item = &String> {
        self.animations.iter()
    }

    pub fn keyframes(&self) -> impl Iterator<Item = &(String, Direction, usize)> {
        self.keyframes.iter()
    }

    pub fn hitboxes(&self) -> impl Iterator<Item = &(String, Direction, usize, String)> {
        self.hitboxes.iter()
    }
}

impl<T: std::cmp::Eq + std::hash::Hash + std::clone::Clone + std::cmp::Ord> Default
    for Selection<T>
{
    fn default() -> Self {
        Self {
            pivot: Default::default(),
            last_interacted: Default::default(),
            selected_items: Default::default(),
        }
    }
}

impl<T: std::cmp::Eq + std::hash::Hash + std::clone::Clone + std::cmp::Ord> Selection<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            pivot: items.last().cloned(),
            last_interacted: items.last().cloned(),
            selected_items: items.into_iter().collect(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.selected_items.is_empty()
    }

    pub fn contains<Q: ?Sized>(&self, item: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: std::hash::Hash + Eq,
    {
        self.selected_items.contains(item)
    }

    pub fn remove<Q: ?Sized>(&mut self, item: &Q)
    where
        T: Borrow<Q>,
        Q: std::hash::Hash + Eq,
    {
        self.selected_items.remove(item);
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.selected_items.retain(f);
    }

    pub fn clear(&mut self) {
        *self = Default::default();
    }

    fn only<U>(&mut self, items: U)
    where
        U: IntoIterator<Item = T>,
    {
        *self = Self::new(items.into_iter().collect());
    }

    // Desired behavior: https://stackoverflow.com/a/16530782
    fn alter<U>(&mut self, interacted_item: T, all_items: U, shift: bool, ctrl: bool)
    where
        U: ItemPool<T>,
    {
        if shift {
            let affected_items = all_items.range(self.pivot.as_ref(), &interacted_item);
            if ctrl {
                let contains_pivot = self
                    .pivot
                    .as_ref()
                    .map(|p| self.contains(p))
                    .unwrap_or_default();
                if contains_pivot {
                    self.insert_items(affected_items);
                } else {
                    self.remove_items(&affected_items);
                }
            } else {
                self.selected_items = HashSet::from_iter(affected_items.into_iter());
            }
        } else if ctrl {
            self.toggle(&interacted_item);
        } else {
            *self = interacted_item.clone().into();
        }

        if ctrl {
            self.pivot = Some(interacted_item.clone());
        }
        self.last_interacted = Some(interacted_item);
    }

    fn toggle(&mut self, item: &T) {
        if self.contains(item) {
            self.remove(item);
        } else {
            self.insert(item.clone());
        }
    }

    fn insert(&mut self, item: T) {
        self.selected_items.insert(item);
    }

    fn insert_items<U>(&mut self, items: U)
    where
        U: IntoIterator<Item = T>,
    {
        self.selected_items.extend(items);
    }

    fn remove_items(&mut self, items: &[T]) {
        for item in items {
            self.selected_items.remove(item);
        }
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        self.selected_items.iter()
    }
}

impl<T: std::cmp::Eq + std::hash::Hash + std::clone::Clone + std::cmp::Ord> From<T>
    for Selection<T>
{
    fn from(selected_item: T) -> Self {
        Selection::new(vec![selected_item])
    }
}

trait ItemPool<T> {
    fn range(&self, from: Option<&T>, to: &T) -> Vec<T>;
}

trait ItemPool1D<T> {
    fn offset_from(&self, from: Option<&T>, delta: i32) -> Option<T>;
}

trait ItemPoolTimeline {
    fn offset_from(
        &self,
        from: Option<&(String, Direction, usize)>,
        direction: BrowseDirection,
    ) -> Option<(String, Direction, usize)>;
}

// General case for 1D ordered list
impl<T> ItemPool<T> for &[T]
where
    T: Eq + Clone,
{
    fn range(&self, from: Option<&T>, to: &T) -> Vec<T> {
        let from_index = from
            .and_then(|f| self.iter().position(|item| item == f))
            .unwrap_or_default();
        let to_index = self.iter().position(|item| item == to).unwrap_or_default();
        let min_index = from_index.min(to_index);
        let max_index = from_index.max(to_index);
        self[min_index..=max_index].to_vec()
    }
}

impl<T> ItemPool1D<T> for &[T]
where
    T: Eq + Clone,
{
    fn offset_from(&self, from: Option<&T>, delta: i32) -> Option<T> {
        let from_index = from.and_then(|f| self.iter().position(|item| item == f))?;
        let to_index = from_index as i32 + delta;
        if to_index >= 0 {
            self.get(to_index as usize).cloned()
        } else {
            None
        }
    }
}

impl<T> ItemPool<T> for &Vec<T>
where
    T: Eq + Clone,
{
    fn range(&self, from: Option<&T>, to: &T) -> Vec<T> {
        (&self[..]).range(from, to)
    }
}

impl<T> ItemPool1D<T> for &Vec<T>
where
    T: Eq + Clone,
{
    fn offset_from(&self, from: Option<&T>, delta: i32) -> Option<T> {
        (&self[..]).offset_from(from, delta)
    }
}

impl<T, const N: usize> ItemPool<T> for &[T; N]
where
    T: Eq + Clone,
{
    fn range(&self, from: Option<&T>, to: &T) -> Vec<T> {
        (&self[..]).range(from, to)
    }
}

impl<T, const N: usize> ItemPool1D<T> for &[T; N]
where
    T: Eq + Clone,
{
    fn offset_from(&self, from: Option<&T>, delta: i32) -> Option<T> {
        (&self[..]).offset_from(from, delta)
    }
}

// Specialization for keyframe selection, where shift+select needs to select keyframes based on their durations and directions
impl<P: Paths> ItemPool<(String, Direction, usize)> for &Animation<P> {
    fn range(
        &self,
        from: Option<&(String, Direction, usize)>,
        to: &(String, Direction, usize),
    ) -> Vec<(String, Direction, usize)> {
        let from = from
            .filter(|from| from.0 == to.0)
            .cloned()
            .unwrap_or_default();

        let animation_name = &from.0;

        let from_direction = from.1;
        let to_direction = to.1;
        let min_direction = from_direction.min(to_direction);
        let max_direction = from_direction.max(to_direction);
        let affected_directions = min_direction..=max_direction;

        let affected_times = {
            let from_index = from.2;
            let from_range = self
                .sequence(from_direction)
                .map(|s| s.keyframe_time_ranges())
                .and_then(|times| times.get(from_index).cloned())
                .unwrap_or(0..0);

            let to_index = to.2;
            let to_range = self
                .sequence(to_direction)
                .map(|s| s.keyframe_time_ranges())
                .and_then(|times| times.get(to_index).cloned())
                .unwrap_or(0..0);

            let min_time = from_range.start.min(to_range.start);
            let max_time = from_range.end.max(to_range.end);
            min_time..max_time
        };

        self.sequences_iter()
            .flat_map(|(direction, sequence)| {
                sequence
                    .keyframe_time_ranges()
                    .into_iter()
                    .enumerate()
                    .map(|(index, range)| (*direction, index, range))
            })
            .filter_map(|(direction, index, range)| {
                let intersection =
                    affected_times.start.max(range.start)..affected_times.end.min(range.end);
                let has_big_overlap = {
                    let intersection_size = intersection.end.saturating_sub(intersection.start);
                    let keyframe_size = range.end.saturating_sub(range.start);
                    intersection_size >= keyframe_size / 2
                };
                let covers_whole_selection = intersection == affected_times;
                let in_time_range = has_big_overlap || covers_whole_selection;
                let in_direction_range = affected_directions.contains(&direction);
                if in_direction_range && in_time_range {
                    Some((animation_name.clone(), direction, index))
                } else {
                    None
                }
            })
            .collect()
    }
}

impl<P: Paths> ItemPoolTimeline for &Animation<P> {
    fn offset_from(
        &self,
        from: Option<&(String, Direction, usize)>,
        browse_direction: BrowseDirection,
    ) -> Option<(String, Direction, usize)> {
        let (animation_name, direction, index) = from?;
        let animation_name = animation_name.clone();
        let sequence = self.sequence(*direction)?;
        let range = sequence.keyframe_time_ranges().get(*index)?.clone();
        let reference_time = Duration::from_millis((range.start + range.end) / 2);

        match browse_direction {
            BrowseDirection::Left => {
                if *index > 0 {
                    Some((animation_name, *direction, index - 1))
                } else {
                    None
                }
            }
            BrowseDirection::Right => {
                let sequence = self.sequence(*direction)?;
                if sequence.keyframe(index + 1).is_some() {
                    Some((animation_name, *direction, index + 1))
                } else {
                    None
                }
            }
            BrowseDirection::Up => reverse_all::<Direction>()
                .skip_while(|d| d != direction)
                .skip(1)
                .find_map(|d| {
                    if let Some(sequence) = self.sequence(d) {
                        if let Some((new_index, _)) = sequence.keyframe_at(reference_time) {
                            return Some((animation_name.clone(), d, new_index));
                        }
                    }
                    None
                }),
            BrowseDirection::Down => all::<Direction>()
                .skip_while(|d| d != direction)
                .skip(1)
                .find_map(|d| {
                    if let Some(sequence) = self.sequence(d) {
                        if let Some((new_index, _)) = sequence.keyframe_at(reference_time) {
                            return Some((animation_name.clone(), d, new_index));
                        }
                    }
                    None
                }),
        }
    }
}

impl BrowseDirection {
    fn as_vec2(&self) -> Vector2D<i32> {
        match self {
            BrowseDirection::Up => vec2(0, -1),
            BrowseDirection::Down => vec2(0, 1),
            BrowseDirection::Left => vec2(-1, 0),
            BrowseDirection::Right => vec2(1, 0),
        }
    }

    fn as_list_offset(&self, list_mode: ListMode) -> i32 {
        let delta_2d = self.as_vec2();
        match list_mode {
            ListMode::Linear => delta_2d.y,
            ListMode::Grid4xN => delta_2d.x + 4 * delta_2d.y,
        }
    }
}

trait KeyframeID {
    fn to_key(&self) -> (&str, Direction, usize);
}

impl Hash for dyn KeyframeID + '_ {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_key().hash(state)
    }
}

impl PartialEq for dyn KeyframeID + '_ {
    fn eq(&self, other: &Self) -> bool {
        self.to_key() == other.to_key()
    }
}

impl KeyframeID for (String, Direction, usize) {
    fn to_key(&self) -> (&str, Direction, usize) {
        (&self.0, self.1, self.2)
    }
}

impl<'a> KeyframeID for (&'a str, Direction, usize) {
    fn to_key(&self) -> (&str, Direction, usize) {
        (self.0, self.1, self.2)
    }
}

impl<'a> Borrow<dyn KeyframeID + 'a> for (String, Direction, usize) {
    fn borrow(&self) -> &(dyn KeyframeID + 'a) {
        self
    }
}

impl<'a> Borrow<dyn KeyframeID + 'a> for (&'a str, Direction, usize) {
    fn borrow(&self) -> &(dyn KeyframeID + 'a) {
        self
    }
}

impl Eq for dyn KeyframeID + '_ {}

trait HitboxID {
    fn to_key(&self) -> (&str, Direction, usize, &str);
}

impl Hash for dyn HitboxID + '_ {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_key().hash(state)
    }
}

impl PartialEq for dyn HitboxID + '_ {
    fn eq(&self, other: &Self) -> bool {
        self.to_key() == other.to_key()
    }
}

impl HitboxID for (String, Direction, usize, String) {
    fn to_key(&self) -> (&str, Direction, usize, &str) {
        (&self.0, self.1, self.2, &self.3)
    }
}

impl<'a> HitboxID for (&'a str, Direction, usize, &'a str) {
    fn to_key(&self) -> (&str, Direction, usize, &str) {
        (self.0, self.1, self.2, self.3)
    }
}

impl<'a> Borrow<dyn HitboxID + 'a> for (String, Direction, usize, String) {
    fn borrow(&self) -> &(dyn HitboxID + 'a) {
        self
    }
}

impl<'a> Borrow<dyn HitboxID + 'a> for (&'a str, Direction, usize, &'a str) {
    fn borrow(&self) -> &(dyn HitboxID + 'a) {
        self
    }
}

impl Eq for dyn HitboxID + '_ {}

#[cfg(test)]
mod test {

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn can_replace_selection() {
        let mut selection: Selection<i32> = 0.into();
        assert!(selection.contains(&0));

        selection.alter(0, &[0, 1, 2, 3], false, false);
        assert!(selection.contains(&0));

        selection.alter(2, &[0, 1, 2, 3], false, false);
        assert!(!selection.contains(&0));
        assert!(selection.contains(&2));
    }

    #[test]
    fn can_toggle_individual_items() {
        let mut selection: Selection<i32> = 0.into();
        assert!(selection.contains(&0));

        selection.alter(2, &[0, 1, 2, 3], false, true);
        assert!(selection.contains(&0));
        assert!(selection.contains(&2));

        selection.alter(0, &[0, 1, 2, 3], false, true);
        assert!(!selection.contains(&0));
        assert!(selection.contains(&2));
    }

    #[test]
    fn can_select_a_range() {
        let mut selection: Selection<i32> = 2.into();
        selection.alter(5, &(0..=8).collect::<Vec<_>>(), true, false);
        assert!(!selection.contains(&1));
        assert!(selection.contains(&2));
        assert!(selection.contains(&3));
        assert!(selection.contains(&4));
        assert!(selection.contains(&5));
        assert!(!selection.contains(&6));
    }

    #[test]
    fn can_adjust_a_range() {
        let mut selection: Selection<i32> = 10.into();

        selection.alter(15, &(0..=20).collect::<Vec<_>>(), true, false);
        selection.alter(18, &(0..=20).collect::<Vec<_>>(), true, false);
        assert!(!selection.contains(&9));
        for i in 10..=18 {
            assert!(selection.contains(&i));
        }
        assert!(!selection.contains(&19));

        selection.alter(15, &(0..=20).collect::<Vec<_>>(), true, false);
        assert!(!selection.contains(&9));
        for i in 10..=15 {
            assert!(selection.contains(&i));
        }
        assert!(!selection.contains(&16));

        selection.alter(5, &(0..=20).collect::<Vec<_>>(), true, false);
        assert!(!selection.contains(&4));
        for i in 5..=10 {
            assert!(selection.contains(&i));
        }
        assert!(!selection.contains(&16));
    }

    #[test]
    fn can_select_multiple_ranges() {
        let mut selection: Selection<i32> = 2.into();
        selection.alter(5, &(0..=20).collect::<Vec<_>>(), true, false);
        selection.alter(10, &(0..=20).collect::<Vec<_>>(), false, true);
        selection.alter(15, &(0..=20).collect::<Vec<_>>(), true, true);
        for i in 0..=20 {
            assert_eq!(
                selection.contains(&i),
                (2..=5).contains(&i) || (10..=15).contains(&i)
            );
        }
    }

    #[test]
    fn can_revert_from_multiple_to_single_range() {
        let mut selection: Selection<i32> = 2.into();
        selection.alter(5, &(0..=20).collect::<Vec<_>>(), true, false);
        selection.alter(10, &(0..=20).collect::<Vec<_>>(), false, true);
        selection.alter(15, &(0..=20).collect::<Vec<_>>(), true, true);
        selection.alter(12, &(0..=20).collect::<Vec<_>>(), true, false);
        for i in 0..=20 {
            assert_eq!(selection.contains(&i), (12..=15).contains(&i));
        }
    }

    #[test]
    fn can_adjust_multiple_ranges() {
        let mut selection: Selection<i32> = 2.into();
        selection.alter(5, &(0..=20).collect::<Vec<_>>(), true, false);
        selection.alter(10, &(0..=20).collect::<Vec<_>>(), false, true);
        selection.alter(15, &(0..=20).collect::<Vec<_>>(), true, true);
        selection.alter(18, &(0..=20).collect::<Vec<_>>(), true, true);
        for i in 0..=20 {
            assert_eq!(
                selection.contains(&i),
                (2..=5).contains(&i) || (10..=18).contains(&i)
            );
        }
        selection.alter(16, &(0..=20).collect::<Vec<_>>(), false, true);
        selection.alter(12, &(0..=20).collect::<Vec<_>>(), true, true);
        for i in 0..=20 {
            assert_eq!(
                selection.contains(&i),
                (2..=5).contains(&i) || (10..=11).contains(&i) || (17..=18).contains(&i)
            );
        }
    }

    #[test]
    fn can_remove_individual_item() {
        let mut selection: Selection<i32> = Selection::new(vec![3, 4, 5, 6]);
        selection.remove(&4);
        assert!(selection.contains(&3));
        assert!(!selection.contains(&4));
        assert!(selection.contains(&5));
        assert!(selection.contains(&6));
    }

    #[test]
    fn can_browse_animations() {
        let mut d = Document::new("tmp");
        d.sheet.add_frames(&vec!["walk_0", "walk_1", "walk_2"]);
        d.sheet.add_test_animation::<_, &str>("A", HashMap::new());
        d.sheet.add_test_animation::<_, &str>("B", HashMap::new());
        d.sheet.add_test_animation::<_, &str>("C", HashMap::new());
        d.select_animation("A", false, false);
        let just_a = HashSet::from(["A".to_owned()]);
        let just_b = HashSet::from(["B".to_owned()]);
        let b_and_c = HashSet::from(["B".to_owned(), "C".to_owned()]);
        assert_eq!(&d.view.selection.animations.selected_items, &just_a,);
        d.browse_selection(BrowseDirection::Down, false).unwrap();
        assert_eq!(&d.view.selection.animations.selected_items, &just_b,);
        d.browse_selection(BrowseDirection::Down, true).unwrap();
        assert_eq!(&d.view.selection.animations.selected_items, &b_and_c);
        d.browse_selection(BrowseDirection::Up, false).unwrap();
        assert_eq!(&d.view.selection.animations.selected_items, &just_b,);
        d.browse_selection(BrowseDirection::Left, false).unwrap();
        assert_eq!(&d.view.selection.animations.selected_items, &just_b,);
    }

    #[test]
    fn can_browse_frames_as_a_list() {
        let mut d = Document::new("tmp");
        d.sheet.add_frames(&vec!["A", "B", "C"]);
        d.view.frames_list_mode = ListMode::Linear;
        d.select_frame("A", false, false);
        let just_a = HashSet::from([PathBuf::from("A")]);
        let just_b = HashSet::from([PathBuf::from("B")]);
        let b_and_c = HashSet::from([PathBuf::from("B"), PathBuf::from("C")]);
        assert_eq!(&d.view.selection.frames.selected_items, &just_a,);
        d.browse_selection(BrowseDirection::Down, false).unwrap();
        assert_eq!(&d.view.selection.frames.selected_items, &just_b,);
        d.browse_selection(BrowseDirection::Down, true).unwrap();
        assert_eq!(&d.view.selection.frames.selected_items, &b_and_c);
        d.browse_selection(BrowseDirection::Up, false).unwrap();
        assert_eq!(&d.view.selection.frames.selected_items, &just_b,);
        d.browse_selection(BrowseDirection::Left, false).unwrap();
        assert_eq!(&d.view.selection.frames.selected_items, &just_b,);
    }

    #[test]
    fn can_browse_frames_as_a_4xn_grid() {
        let mut d = Document::new("tmp");
        d.sheet.add_frames(&vec![
            "00", "10", "20", "30", //
            "01", "11", "21", "31", //
            "02", "12", "22", "32",
        ]);
        d.view.frames_list_mode = ListMode::Grid4xN;
        d.select_frame("00", false, false);
        let to_set = |v: Vec<&str>| v.iter().map(PathBuf::from).collect();

        d.browse_selection(BrowseDirection::Right, false).unwrap();
        assert_eq!(d.view.selection.frames.selected_items, to_set(vec!["10"]));
        d.browse_selection(BrowseDirection::Right, false).unwrap();
        assert_eq!(d.view.selection.frames.selected_items, to_set(vec!["20"]));
        d.browse_selection(BrowseDirection::Right, false).unwrap();
        assert_eq!(d.view.selection.frames.selected_items, to_set(vec!["30"]));
        d.browse_selection(BrowseDirection::Right, false).unwrap();
        assert_eq!(d.view.selection.frames.selected_items, to_set(vec!["01"]));
        d.browse_selection(BrowseDirection::Left, false).unwrap();
        assert_eq!(d.view.selection.frames.selected_items, to_set(vec!["30"]));
        d.browse_selection(BrowseDirection::Down, true).unwrap();
        assert_eq!(
            d.view.selection.frames.selected_items,
            to_set(vec!["30", "01", "11", "21", "31"])
        );
        d.browse_selection(BrowseDirection::Down, true).unwrap();
        assert_eq!(
            d.view.selection.frames.selected_items,
            to_set(vec!["30", "01", "11", "21", "31", "02", "12", "22", "32"])
        );
        d.browse_selection(BrowseDirection::Left, true).unwrap();
        assert_eq!(
            d.view.selection.frames.selected_items,
            to_set(vec!["30", "01", "11", "21", "31", "02", "12", "22"])
        );
    }
}
