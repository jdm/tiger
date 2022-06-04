use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::sheet::Direction;
use crate::state::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MultiSelection {
    pub(super) frames: MultiSelectionData<PathBuf>,
    pub(super) animations: MultiSelectionData<String>,
    pub(super) hitboxes: MultiSelectionData<String>,
    pub(super) keyframes: MultiSelectionData<(String, Direction, usize)>,
}

#[derive(Clone, Debug)]
pub enum SelectionInput {
    Frame(PathBuf),
    Animation(String),
    Hitbox(String),
    Keyframe(Direction, usize),
}

pub enum MultiSelectionEdit {
    Frames(PathBuf, Vec<PathBuf>),
    Animations(String, Vec<String>),
    Hitboxes(String, Vec<String>),
    Keyframes((String, Direction, usize), Vec<(String, Direction, usize)>),
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
            SelectionInput::Hitbox(_) => todo!(),
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
                let all_keyframes: Vec<(String, Direction, usize)> = self
                    .sheet
                    .animations_iter()
                    .flat_map(|(name, animation)| {
                        animation
                            .sequences_iter()
                            .flat_map(|(direction, sequence)| {
                                (0..sequence.num_keyframes())
                                    .map(|index| (name.clone(), *direction, index))
                            })
                    })
                    .collect();
                MultiSelectionEdit::Keyframes((animation_name.clone(), *d, *i), all_keyframes)
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
        self.frames.only(frame);
    }

    pub fn select_animation(&mut self, animation: String) {
        self.clear();
        self.animations.only(animation);
    }

    pub fn select_keyframe(&mut self, animation: String, direction: Direction, index: usize) {
        self.clear();
        self.keyframes.only((animation, direction, index));
    }

    pub fn select_keyframes<T>(&mut self, keyframes: T)
    where
        T: IntoIterator<Item = (String, Direction, usize)>,
    {
        self.clear();
        // TODO this doesnt set pivot!!
        self.keyframes.insert_items(keyframes);
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
            MultiSelectionEdit::Keyframes(item, set) => {
                self.keyframes.alter(item, &set, shift, ctrl);
            }
        }
    }

    pub fn is_frame_selected(&self, path: &Path) -> bool {
        self.frames.contains(path)
    }

    pub fn is_animation_selected<T: AsRef<str>>(&self, name: T) -> bool {
        self.animations.contains(name.as_ref())
    }

    pub fn is_hitbox_selected<T: AsRef<str>>(&self, name: T) -> bool {
        self.hitboxes.contains(name.as_ref())
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

    fn only(&mut self, item: T) {
        *self = Self::new(vec![item]);
    }

    // Desired behavior: https://stackoverflow.com/a/16530782
    fn alter(&mut self, interacted_item: T, all_items: &Vec<T>, shift: bool, ctrl: bool) {
        let interacted_item_index = match all_items.iter().position(|item| *item == interacted_item)
        {
            Some(i) => i,
            None => return,
        };

        if shift {
            let pivot_index = self
                .pivot
                .as_ref()
                .and_then(|p| all_items.iter().position(|item| item == p))
                .unwrap_or_default();
            let range_start = pivot_index.min(interacted_item_index);
            let range_end = pivot_index.max(interacted_item_index);
            if ctrl {
                let contains_pivot = self
                    .pivot
                    .as_ref()
                    .map(|p| self.contains(p))
                    .unwrap_or_default();
                if contains_pivot {
                    self.insert_items(all_items[range_start..=range_end].iter().cloned());
                } else {
                    self.remove_items(&all_items[range_start..=range_end].iter().collect());
                }
            } else {
                self.selected_items = all_items[range_start..=range_end].iter().cloned().collect();
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

    fn remove_items(&mut self, items: &Vec<&T>) {
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

#[test]
fn can_replace_selection() {
    let mut selection: MultiSelectionData<i32> = 0.into();
    assert!(selection.contains(&0));

    selection.alter(0, &vec![0, 1, 2, 3], false, false);
    assert!(selection.contains(&0));

    selection.alter(2, &vec![0, 1, 2, 3], false, false);
    assert!(!selection.contains(&0));
    assert!(selection.contains(&2));
}

#[test]
fn can_toggle_individual_items() {
    let mut selection: MultiSelectionData<i32> = 0.into();
    assert!(selection.contains(&0));

    selection.alter(2, &vec![0, 1, 2, 3], false, true);
    assert!(selection.contains(&0));
    assert!(selection.contains(&2));

    selection.alter(0, &vec![0, 1, 2, 3], false, true);
    assert!(!selection.contains(&0));
    assert!(selection.contains(&2));
}

#[test]
fn can_select_a_range() {
    let mut selection: MultiSelectionData<i32> = 2.into();
    selection.alter(5, &(0..=8).collect(), true, false);
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

    selection.alter(15, &(0..=20).collect(), true, false);
    selection.alter(18, &(0..=20).collect(), true, false);
    assert!(!selection.contains(&9));
    for i in 10..=18 {
        assert!(selection.contains(&i));
    }
    assert!(!selection.contains(&19));

    selection.alter(15, &(0..=20).collect(), true, false);
    assert!(!selection.contains(&9));
    for i in 10..=15 {
        assert!(selection.contains(&i));
    }
    assert!(!selection.contains(&16));

    selection.alter(5, &(0..=20).collect(), true, false);
    assert!(!selection.contains(&4));
    for i in 5..=10 {
        assert!(selection.contains(&i));
    }
    assert!(!selection.contains(&16));
}

#[test]
fn can_select_multiple_ranges() {
    let mut selection: MultiSelectionData<i32> = 2.into();
    selection.alter(5, &(0..=20).collect(), true, false);
    selection.alter(10, &(0..=20).collect(), false, true);
    selection.alter(15, &(0..=20).collect(), true, true);
    for i in 0..=20 {
        assert_eq!(
            selection.contains(&i),
            (i >= 2 && i <= 5) || (i >= 10 && i <= 15)
        );
    }
}

#[test]
fn can_revert_from_multiple_to_single_range() {
    let mut selection: MultiSelectionData<i32> = 2.into();
    selection.alter(5, &(0..=20).collect(), true, false);
    selection.alter(10, &(0..=20).collect(), false, true);
    selection.alter(15, &(0..=20).collect(), true, true);
    selection.alter(12, &(0..=20).collect(), true, false);
    for i in 0..=20 {
        assert_eq!(selection.contains(&i), i >= 12 && i <= 15);
    }
}

#[test]
fn can_adjust_multiple_ranges() {
    let mut selection: MultiSelectionData<i32> = 2.into();
    selection.alter(5, &(0..=20).collect(), true, false);
    selection.alter(10, &(0..=20).collect(), false, true);
    selection.alter(15, &(0..=20).collect(), true, true);
    selection.alter(18, &(0..=20).collect(), true, true);
    for i in 0..=20 {
        assert_eq!(
            selection.contains(&i),
            (i >= 2 && i <= 5) || (i >= 10 && i <= 18)
        );
    }
    selection.alter(16, &(0..=20).collect(), false, true);
    selection.alter(12, &(0..=20).collect(), true, true);
    for i in 0..=20 {
        assert_eq!(
            selection.contains(&i),
            (i >= 2 && i <= 5) || (i >= 10 && i <= 11) || (i >= 17 && i <= 18)
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
