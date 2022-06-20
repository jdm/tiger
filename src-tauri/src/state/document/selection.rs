use enum_iterator::Sequence;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::sheet::{Animation, Direction};
use crate::state::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MultiSelection {
    pub(super) frames: MultiSelectionData<PathBuf>,
    pub(super) animations: MultiSelectionData<String>,
    pub(super) hitboxes: MultiSelectionData<(String, Direction, usize, String)>,
    pub(super) keyframes: MultiSelectionData<(String, Direction, usize)>,
}

#[derive(Clone, Debug)]
pub enum SelectionInput {
    Frame(PathBuf),
    Animation(String),
    Hitbox(String),
    Keyframe(Direction, usize),
}

pub enum MultiSelectionEdit<'a> {
    Frames(PathBuf, Vec<PathBuf>),
    Animations(String, Vec<String>),
    Hitboxes(
        (String, Direction, usize, String),
        Vec<(String, Direction, usize, String)>,
    ),
    Keyframes(
        (String, Direction, usize),
        HashMap<&'a String, &'a Animation>,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub struct MultiSelectionData<T>
where
    T: std::cmp::Eq + std::hash::Hash + std::clone::Clone,
{
    pivot: Option<T>,
    selected_items: HashSet<T>,
}

impl Document {
    pub(super) fn alter_selection(
        &mut self,
        selection: &SelectionInput,
        shift: bool,
        ctrl: bool,
    ) -> Result<(), DocumentError> {
        let edit = match selection {
            SelectionInput::Frame(f) => MultiSelectionEdit::Frames(
                f.clone(),
                self.sheet
                    .frames_iter()
                    .map(|f| f.source().to_owned())
                    .collect(),
            ),
            SelectionInput::Animation(a) => MultiSelectionEdit::Animations(
                a.clone(),
                self.sheet
                    .animations_iter()
                    .map(|(n, _)| n.clone())
                    .collect(),
            ),
            SelectionInput::Hitbox(h) => {
                let (animation_name, _) = self.get_workbench_animation()?;
                let ((direction, index), keyframe) = self.get_workbench_keyframe()?;
                MultiSelectionEdit::Hitboxes(
                    (animation_name.clone(), direction, index, h.clone()),
                    keyframe
                        .hitboxes_iter()
                        .map(|(n, _)| (animation_name.clone(), direction, index, n.clone()))
                        .collect(),
                )
            }
            SelectionInput::Keyframe(d, i) => {
                self.view.current_sequence = Some(*d);
                let (_, sequence) = self.get_workbench_sequence()?;
                if !self.persistent.timeline_is_playing {
                    self.view.timeline_clock = Duration::from_millis(
                        sequence
                            .keyframe_times()
                            .get(*i)
                            .copied()
                            .unwrap_or_default(),
                    );
                }
                let (animation_name, _) = self.get_workbench_animation()?;
                let all_animations: HashMap<&String, &Animation> =
                    self.sheet.animations_iter().collect();

                MultiSelectionEdit::Keyframes((animation_name.clone(), *d, *i), all_animations)
            }
        };
        self.view.selection.alter(edit, shift, ctrl);
        Ok(())
    }
}

impl MultiSelection {
    pub fn clear(&mut self) {
        *self = Default::default();
    }

    pub fn select_frame(&mut self, frame: PathBuf) {
        self.clear();
        self.frames.only(vec![frame]);
    }

    pub fn select_animation(&mut self, animation: String) {
        self.clear();
        self.animations.only(vec![animation]);
    }

    pub fn select_keyframe(&mut self, animation: String, direction: Direction, index: usize) {
        self.clear();
        self.keyframes.only(vec![(animation, direction, index)]);
    }

    pub fn select_keyframes<T>(&mut self, keyframes: T)
    where
        T: IntoIterator<Item = (String, Direction, usize)>,
    {
        self.clear();
        self.keyframes.only(keyframes);
    }

    pub fn select_hitbox(
        &mut self,
        animation: String,
        direction: Direction,
        index: usize,
        hitbox: String,
    ) {
        self.clear();
        self.hitboxes
            .only(vec![(animation, direction, index, hitbox)]);
    }

    pub fn alter(&mut self, edit: MultiSelectionEdit, shift: bool, ctrl: bool) {
        if !matches!(edit, MultiSelectionEdit::Frames(_, _)) {
            self.frames.clear();
        }
        if !matches!(edit, MultiSelectionEdit::Animations(_, _)) {
            self.animations.clear();
        }
        if !matches!(edit, MultiSelectionEdit::Hitboxes(_, _)) {
            self.hitboxes.clear();
        }
        if !matches!(edit, MultiSelectionEdit::Keyframes(_, _)) {
            self.keyframes.clear();
        }
        match edit {
            MultiSelectionEdit::Frames(item, set) => {
                self.frames.alter(item, &set, shift, ctrl);
            }
            MultiSelectionEdit::Animations(item, set) => {
                self.animations.alter(item, &set, shift, ctrl);
            }
            MultiSelectionEdit::Hitboxes(item, set) => {
                self.hitboxes.alter(item, &set, shift, ctrl);
            }
            MultiSelectionEdit::Keyframes(item, animations) => {
                self.keyframes.alter(item, &animations, shift, ctrl);
            }
        }
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
    for MultiSelectionData<T>
{
    fn default() -> Self {
        Self {
            pivot: Default::default(),
            selected_items: Default::default(),
        }
    }
}

impl<T: std::cmp::Eq + std::hash::Hash + std::clone::Clone + std::cmp::Ord> MultiSelectionData<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            pivot: items.last().cloned(),
            selected_items: items.into_iter().collect(),
        }
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

    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.pivot = self.pivot.take().filter(&mut f);
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
    for MultiSelectionData<T>
{
    fn from(selected_item: T) -> Self {
        MultiSelectionData::new(vec![selected_item])
    }
}

trait ItemPool<T> {
    fn get_range(&self, from: Option<&T>, to: &T) -> Vec<T>;
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

impl<T> ItemPool<T> for &Vec<T>
where
    T: Eq + Clone,
{
    fn get_range(&self, from: Option<&T>, to: &T) -> Vec<T> {
        (&self[..]).get_range(from, to)
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

// Special case for keyframe selection, where shift+select needs to select keyframes based on their durations and directions
impl ItemPool<(String, Direction, usize)> for &HashMap<&String, &Animation> {
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

        let animation = match self.get(&to.0) {
            Some(animation) => *animation,
            None => return vec![],
        };
        let animation_name = &from.0;

        let from_direction = from.1;
        let to_direction = to.1;
        let min_direction = from_direction.min(to_direction);
        let max_direction = from_direction.max(to_direction);
        let affected_directions = min_direction..=max_direction;

        let affected_times = {
            let from_index = from.2;
            let from_range = animation
                .sequence(from_direction)
                .map(|s| s.keyframe_time_ranges())
                .and_then(|times| times.get(from_index).cloned())
                .unwrap_or(0..0);

            let to_index = to.2;
            let to_range = animation
                .sequence(to_direction)
                .map(|s| s.keyframe_time_ranges())
                .and_then(|times| times.get(to_index).cloned())
                .unwrap_or(0..0);

            let min_time = from_range.start.min(to_range.start);
            let max_time = from_range.end.max(to_range.end);
            min_time..max_time
        };

        animation
            .sequences_iter()
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
    let mut selection: MultiSelectionData<i32> = 0.into();
    assert!(selection.contains(&0));

    selection.alter(0, &[0, 1, 2, 3], false, false);
    assert!(selection.contains(&0));

    selection.alter(2, &[0, 1, 2, 3], false, false);
    assert!(!selection.contains(&0));
    assert!(selection.contains(&2));
}

#[test]
fn can_toggle_individual_items() {
    let mut selection: MultiSelectionData<i32> = 0.into();
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
    let mut selection: MultiSelectionData<i32> = 2.into();
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
    let mut selection: MultiSelectionData<i32> = 10.into();

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
    let mut selection: MultiSelectionData<i32> = 2.into();
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
    let mut selection: MultiSelectionData<i32> = 2.into();
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
    let mut selection: MultiSelectionData<i32> = 2.into();
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
    let mut selection: MultiSelectionData<i32> = MultiSelectionData::new(vec![3, 4, 5, 6]);
    selection.remove(&4);
    assert!(selection.contains(&3));
    assert!(!selection.contains(&4));
    assert!(selection.contains(&5));
    assert!(selection.contains(&6));
}
