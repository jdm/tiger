use enum_iterator::Sequence;
use euclid::default::Vector2D;
use euclid::vec2;
use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::sheet::{Animation, Direction};
use crate::state::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SelectionState {
    pub(super) frames: Selection<PathBuf>,
    pub(super) animations: Selection<String>,
    pub(super) hitboxes: Selection<(String, Direction, usize, String)>,
    pub(super) keyframes: Selection<(String, Direction, usize)>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NudgeDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
pub enum BrowseSelectionDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Selection<T>
where
    T: std::cmp::Eq + std::hash::Hash + std::clone::Clone,
{
    pivot: Option<T>,
    last_interacted: Option<T>,
    selected_items: HashSet<T>,
}

impl Document {
    pub(super) fn delete_selection(&mut self) -> Result<(), DocumentError> {
        self.delete_selected_frames();
        self.delete_selected_animations();
        self.delete_selected_keyframes()?;
        self.delete_selected_hitboxes()?;
        Ok(())
    }

    pub(super) fn nudge_selection(
        &mut self,
        direction: NudgeDirection,
        large_nudge: bool,
    ) -> Result<(), DocumentError> {
        let mut delta = match direction {
            NudgeDirection::Up => vec2(0, -1),
            NudgeDirection::Down => vec2(0, 1),
            NudgeDirection::Left => vec2(-1, 0),
            NudgeDirection::Right => vec2(1, 0),
        };
        if large_nudge {
            delta *= 10;
        }

        for (_, _, keyframe) in self.get_selected_keyframes_mut()? {
            Document::nudge_keyframe(keyframe, keyframe.offset() + delta);
        }

        for (_, hitbox) in self.get_selected_hitboxes_mut()? {
            hitbox.set_position(hitbox.position() + delta);
        }

        Ok(())
    }

    pub fn select_frame_only(&mut self, frame: PathBuf) {
        self.view.selection.clear();
        self.view.selection.frames.only(vec![frame]);
    }

    pub fn select_animation_only(&mut self, animation: String) {
        self.view.selection.clear();
        self.view.selection.animations.only(vec![animation]);
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

    pub fn select_hitbox_only(
        &mut self,
        animation: String,
        direction: Direction,
        index: usize,
        hitbox: String,
    ) {
        self.view.selection.clear();
        self.view
            .selection
            .hitboxes
            .only(vec![(animation, direction, index, hitbox)]);
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
    ) -> Result<(), DocumentError> {
        self.view.selection.frames.clear();
        self.view.selection.animations.clear();
        self.view.selection.hitboxes.clear();

        self.view.current_sequence = Some(direction);
        if !self.persistent.timeline_is_playing {
            let (_, sequence) = self.get_workbench_sequence()?;
            self.view.timeline_clock = Duration::from_millis(
                sequence
                    .keyframe_times()
                    .get(index)
                    .copied()
                    .unwrap_or_default(),
            );
        }
        let (animation_name, _) = self.get_workbench_animation()?;
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

    pub(super) fn select_hitbox<T: AsRef<str>>(
        &mut self,
        name: T,
        shift: bool,
        ctrl: bool,
    ) -> Result<(), DocumentError> {
        self.view.selection.frames.clear();
        self.view.selection.animations.clear();
        self.view.selection.keyframes.clear();

        let (animation_name, _) = self.get_workbench_animation()?;
        let animation_name = animation_name.clone();
        let ((direction, index), _) = self.get_workbench_keyframe()?;
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
        browse_direction: BrowseSelectionDirection,
        shift: bool,
    ) -> Result<(), DocumentError> {
        let ctrl = false;
        if !self.view.selection.frames.is_empty() {
            let item_pool = self.selectable_frames();
            let delta = browse_direction.as_list_offset(self.view.frames_list_mode);
            if let Some(interacted_item) =
                (&item_pool).offset_from(self.view.selection.frames.last_interacted.as_ref(), delta)
            {
                self.select_frame(interacted_item, shift, ctrl);
            }
        } else if !self.view.selection.animations.is_empty() {
            let item_pool = self.selectable_animations();
            let delta = browse_direction.as_list_offset(ListMode::Linear);
            if let Some(interacted_item) = (&item_pool).offset_from(
                self.view.selection.animations.last_interacted.as_ref(),
                delta,
            ) {
                self.select_animation(interacted_item, shift, ctrl);
            }
        } else if !self.view.selection.hitboxes.is_empty() {
            let item_pool = self.selectable_hitboxes()?;
            let delta = browse_direction.as_list_offset(ListMode::Linear);
            if let Some((_, _, _, hitbox_name)) = (&item_pool)
                .offset_from(self.view.selection.hitboxes.last_interacted.as_ref(), delta)
            {
                self.select_hitbox(hitbox_name, shift, ctrl)?;
            }
        } else {
            let (animation_name, _) = self.get_workbench_animation()?;
            let animation = self
                .sheet
                .animation(&animation_name)
                .ok_or_else(|| DocumentError::AnimationNotInDocument(animation_name.clone()))?;
            let ((direction, index), _) = self.get_workbench_keyframe()?;
            let current_keyframe = (animation_name.clone(), direction, index);
            let reference_item = self
                .view
                .selection
                .keyframes
                .last_interacted
                .as_ref()
                .or(Some(&current_keyframe));
            if let Some((_, direction, index)) =
                animation.offset_from(reference_item, browse_direction, self.view.timeline_clock)
            {
                self.select_keyframe(direction, index, shift, ctrl)?;
            }
        }
        Ok(())
    }

    fn selectable_frames(&self) -> Vec<PathBuf> {
        self.sheet
            .frames_iter()
            .map(|f| f.source().to_owned())
            .collect()
    }

    fn selectable_animations(&self) -> Vec<String> {
        self.sheet
            .animations_iter()
            .map(|(n, _)| n.clone())
            .collect()
    }

    fn selectable_hitboxes(
        &self,
    ) -> Result<Vec<(String, Direction, usize, String)>, DocumentError> {
        let (animation_name, _) = self.get_workbench_animation()?;
        let ((direction, index), keyframe) = self.get_workbench_keyframe()?;
        Ok(keyframe
            .hitboxes_iter()
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
            let affected_items = all_items.get_range(self.pivot.as_ref(), &interacted_item);
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
    fn get_range(&self, from: Option<&T>, to: &T) -> Vec<T>;
}

trait ItemPool1D<T> {
    fn offset_from(&self, from: Option<&T>, delta: i32) -> Option<T>;
}

trait ItemPoolTimeline {
    fn offset_from(
        &self,
        from: Option<&(String, Direction, usize)>,
        delta: BrowseSelectionDirection,
        timeline_clock: Duration,
    ) -> Option<(String, Direction, usize)>;
}

// General case for 1D ordered list
impl<T> ItemPool<T> for &[T]
where
    T: Eq + Clone,
{
    fn get_range(&self, from: Option<&T>, to: &T) -> Vec<T> {
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
    fn get_range(&self, from: Option<&T>, to: &T) -> Vec<T> {
        (&self[..]).get_range(from, to)
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
    fn get_range(&self, from: Option<&T>, to: &T) -> Vec<T> {
        (&self[..]).get_range(from, to)
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

// Special case for keyframe selection, where shift+select needs to select keyframes based on their durations and directions
impl ItemPool<(String, Direction, usize)> for &Animation {
    fn get_range(
        &self,
        from: Option<&(String, Direction, usize)>,
        to: &(String, Direction, usize),
    ) -> Vec<(String, Direction, usize)> {
        // TODO switch to unwrap or default once https://github.com/rust-lang/rust/pull/94457 is on stable
        let from = from
            .filter(|from| from.0 == to.0)
            .cloned()
            .unwrap_or_else(|| (to.0.clone(), Direction::first().unwrap(), 0));

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
                let in_time_range = intersection.end.saturating_sub(intersection.start)
                    >= (range.end.saturating_sub(range.start) / 2);
                if affected_directions.contains(&direction) && in_time_range {
                    Some((animation_name.clone(), direction, index))
                } else {
                    None
                }
            })
            .collect()
    }
}

impl ItemPoolTimeline for &Animation {
    fn offset_from(
        &self,
        from: Option<&(String, Direction, usize)>,
        browse_direction: BrowseSelectionDirection,
        timeline_clock: Duration,
    ) -> Option<(String, Direction, usize)> {
        let (animation_name, direction, index) = from?;
        let animation_name = animation_name.clone();
        match browse_direction {
            BrowseSelectionDirection::Left => {
                if *index > 0 {
                    Some((animation_name, *direction, index - 1))
                } else {
                    None
                }
            }
            BrowseSelectionDirection::Right => {
                let sequence = self.sequence(*direction)?;
                if sequence.keyframe(index + 1).is_some() {
                    Some((animation_name, *direction, index + 1))
                } else {
                    None
                }
            }
            BrowseSelectionDirection::Up => {
                let mut new_direction = direction.previous();
                while let Some(d) = new_direction {
                    if let Some(sequence) = self.sequence(d) {
                        if let Some((new_index, _)) = sequence.keyframe_at(timeline_clock) {
                            return Some((animation_name, d, new_index));
                        }
                    }
                    new_direction = d.previous();
                }
                None
            }
            BrowseSelectionDirection::Down => {
                let mut new_direction = direction.next();
                while let Some(d) = new_direction {
                    if let Some(sequence) = self.sequence(d) {
                        if let Some((new_index, _)) = sequence.keyframe_at(timeline_clock) {
                            return Some((animation_name, d, new_index));
                        }
                    }
                    new_direction = d.next();
                }
                None
            }
        }
    }
}

impl BrowseSelectionDirection {
    fn as_vec2(&self) -> Vector2D<i32> {
        match self {
            BrowseSelectionDirection::Up => vec2(0, -1),
            BrowseSelectionDirection::Down => vec2(0, 1),
            BrowseSelectionDirection::Left => vec2(-1, 0),
            BrowseSelectionDirection::Right => vec2(1, 0),
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
