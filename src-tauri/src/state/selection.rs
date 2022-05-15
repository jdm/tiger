use std::borrow::Borrow;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MultiSelection {
    content: Option<TaggedMultiSelection>,
}

pub enum MultiSelectionEdit {
    Frames(PathBuf, Vec<PathBuf>),
    Animations(String, Vec<String>),
    Hitboxes(String, Vec<String>),
    Keyframes(usize, Vec<usize>),
}

#[derive(Clone, Debug, PartialEq)]
enum TaggedMultiSelection {
    Frames(MultiSelectionData<PathBuf>),
    Animations(MultiSelectionData<String>),
    Hitboxes(MultiSelectionData<String>),
    Keyframes(MultiSelectionData<usize>),
}

#[derive(Clone, Debug, PartialEq)]
struct MultiSelectionData<T>
where
    T: std::cmp::Eq + std::hash::Hash + std::clone::Clone,
{
    pub pivot: T,
    pub selected_items: HashSet<T>,
}

impl MultiSelection {
    pub fn alter(mut self, edit: MultiSelectionEdit, shift: bool, ctrl: bool) {
        if let Some(content) = self.content {
            self.content = content.alter(edit, shift, ctrl);
        } else {
            self.content = Some(edit.into());
        }
    }

    pub fn is_frame_selected(&self, path: &Path) -> bool {
        match &self.content {
            Some(TaggedMultiSelection::Frames(s)) => s.contains(path),
            _ => false,
        }
    }

    pub fn is_animation_selected<T: AsRef<str>>(&self, name: T) -> bool {
        match &self.content {
            Some(TaggedMultiSelection::Animations(s)) => s.contains(name.as_ref()),
            _ => false,
        }
    }

    pub fn is_hitbox_selected<T: AsRef<str>>(&self, name: T) -> bool {
        match &self.content {
            Some(TaggedMultiSelection::Hitboxes(s)) => s.contains(name.as_ref()),
            _ => false,
        }
    }

    pub fn is_keyframe_selected(&self, index: usize) -> bool {
        match &self.content {
            Some(TaggedMultiSelection::Keyframes(s)) => s.contains(&index),
            _ => false,
        }
    }
}

impl TaggedMultiSelection {
    pub fn alter(
        self,
        edit: MultiSelectionEdit,
        shift: bool,
        ctrl: bool,
    ) -> Option<TaggedMultiSelection> {
        match (self, edit) {
            // Alter frame selection
            (
                TaggedMultiSelection::Frames(data),
                MultiSelectionEdit::Frames(frame, ref all_frames),
            ) => data
                .alter(frame, all_frames, shift, ctrl)
                .map(TaggedMultiSelection::Frames),

            // Alter animation selection
            (
                TaggedMultiSelection::Animations(data),
                MultiSelectionEdit::Animations(animation, ref all_animations),
            ) => data
                .alter(animation, all_animations, shift, ctrl)
                .map(TaggedMultiSelection::Animations),

            // Alter hitbox selection
            (
                TaggedMultiSelection::Hitboxes(data),
                MultiSelectionEdit::Hitboxes(hitbox, ref all_hitboxes),
            ) => data
                .alter(hitbox, all_hitboxes, shift, ctrl)
                .map(TaggedMultiSelection::Hitboxes),

            // Alter keyframe selection
            (
                TaggedMultiSelection::Keyframes(data),
                MultiSelectionEdit::Keyframes(keyframe, ref all_keyframes),
            ) => data
                .alter(keyframe, all_keyframes, shift, ctrl)
                .map(TaggedMultiSelection::Keyframes),

            // Change selection type altogether
            (_, edit) => Some(edit.into()),
        }
    }
}

impl From<MultiSelectionEdit> for TaggedMultiSelection {
    fn from(edit: MultiSelectionEdit) -> Self {
        match edit {
            MultiSelectionEdit::Frames(f, _) => TaggedMultiSelection::Frames(f.into()),
            MultiSelectionEdit::Animations(a, _) => TaggedMultiSelection::Animations(a.into()),
            MultiSelectionEdit::Hitboxes(h, _) => TaggedMultiSelection::Hitboxes(h.into()),
            MultiSelectionEdit::Keyframes(k, _) => TaggedMultiSelection::Keyframes(k.into()),
        }
    }
}

impl<T: std::cmp::Eq + std::hash::Hash + std::clone::Clone + std::cmp::Ord> MultiSelectionData<T> {
    pub fn new(items: Vec<T>) -> Self {
        assert!(items.len() > 0);
        Self {
            pivot: items[items.len() - 1].clone(),
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

    // Desired behavior: https://stackoverflow.com/a/16530782
    pub fn alter(
        mut self,
        interacted_item: T,
        all_items: &Vec<T>,
        shift: bool,
        ctrl: bool,
    ) -> Option<Self> {
        assert!(self.selected_items.len() > 0);

        let interacted_item_index = all_items.iter().position(|item| *item == interacted_item)?;

        if shift {
            let pivot_index = all_items.iter().position(|item| item == &self.pivot)?;
            let range_start = pivot_index.min(interacted_item_index);
            let range_end = pivot_index.max(interacted_item_index);
            if ctrl {
                if self.contains(&self.pivot) {
                    self.insert_items(all_items[range_start..=range_end].iter().cloned().collect());
                } else {
                    self.remove_items(&all_items[range_start..=range_end].iter().collect());
                }
            } else {
                self.selected_items = all_items[range_start..=range_end].iter().cloned().collect();
            }
        } else if ctrl {
            self.toggle(&interacted_item);
        } else {
            self = interacted_item.clone().into();
        }

        if ctrl {
            self.pivot = interacted_item.clone();
        }

        if self.selected_items.is_empty() {
            None
        } else {
            Some(self)
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

    fn remove(&mut self, item: &T) {
        self.selected_items.remove(item);
    }

    fn insert_items(&mut self, items: Vec<T>) {
        self.selected_items.extend(items);
    }

    fn remove_items(&mut self, items: &Vec<&T>) {
        for item in items {
            self.selected_items.remove(item);
        }
    }
}

impl<T: std::cmp::Eq + std::hash::Hash + std::clone::Clone + std::cmp::Ord> From<T>
    for MultiSelectionData<T>
{
    fn from(selected_item: T) -> Self {
        MultiSelectionData::new(vec![selected_item])
    }
}

#[test]
fn can_replace_selection() {
    let selection: MultiSelectionData<i32> = 0.into();
    assert!(selection.contains(&0));

    let same_selection = selection
        .clone()
        .alter(0, &vec![0, 1, 2, 3], false, false)
        .unwrap();
    assert!(same_selection.contains(&0));

    let changed_selection = selection
        .clone()
        .alter(2, &vec![0, 1, 2, 3], false, false)
        .unwrap();
    assert!(!changed_selection.contains(&0));
    assert!(changed_selection.contains(&2));
}

#[test]
fn can_toggle_individual_items() {
    let selection: MultiSelectionData<i32> = 0.into();
    assert!(selection.contains(&0));

    let with_2 = selection
        .clone()
        .alter(2, &vec![0, 1, 2, 3], false, true)
        .unwrap();
    assert!(with_2.contains(&0));
    assert!(with_2.contains(&2));

    let only_2 = with_2
        .clone()
        .alter(0, &vec![0, 1, 2, 3], false, true)
        .unwrap();
    assert!(!only_2.contains(&0));
    assert!(only_2.contains(&2));
}

#[test]
fn cannot_be_empty() {
    let selection: MultiSelectionData<i32> = 0.into();
    assert!(selection.alter(0, &vec![0, 1, 2, 3], false, true).is_none());
}

#[test]
fn can_select_a_range() {
    let selection: MultiSelectionData<i32> = 2.into();
    let up_to_5 = selection.alter(5, &(0..=8).collect(), true, false).unwrap();
    assert!(!up_to_5.contains(&1));
    assert!(up_to_5.contains(&2));
    assert!(up_to_5.contains(&3));
    assert!(up_to_5.contains(&4));
    assert!(up_to_5.contains(&5));
    assert!(!up_to_5.contains(&6));
}

#[test]
fn can_adjust_a_range() {
    let selection: MultiSelectionData<i32> = 10.into();

    let up_to_15 = selection
        .alter(15, &(0..=20).collect(), true, false)
        .unwrap();
    let up_to_18 = up_to_15
        .alter(18, &(0..=20).collect(), true, false)
        .unwrap();
    assert!(!up_to_18.contains(&9));
    for i in 10..=18 {
        assert!(up_to_18.contains(&i));
    }
    assert!(!up_to_18.contains(&19));

    let down_to_15 = up_to_18
        .alter(15, &(0..=20).collect(), true, false)
        .unwrap();
    assert!(!down_to_15.contains(&9));
    for i in 10..=15 {
        assert!(down_to_15.contains(&i));
    }
    assert!(!down_to_15.contains(&16));

    let down_to_5 = down_to_15
        .alter(5, &(0..=20).collect(), true, false)
        .unwrap();
    assert!(!down_to_5.contains(&4));
    for i in 5..=10 {
        assert!(down_to_5.contains(&i));
    }
    assert!(!down_to_5.contains(&16));
}

#[test]
fn can_select_multiple_ranges() {
    let selection: MultiSelectionData<i32> = 2.into();
    let up_to_5 = selection
        .alter(5, &(0..=20).collect(), true, false)
        .unwrap();
    let also_10 = up_to_5.alter(10, &(0..=20).collect(), false, true).unwrap();
    let up_to_15 = also_10.alter(15, &(0..=20).collect(), true, true).unwrap();
    for i in 0..=20 {
        assert_eq!(
            up_to_15.contains(&i),
            (i >= 2 && i <= 5) || (i >= 10 && i <= 15)
        );
    }
}

#[test]
fn can_revert_from_multiple_to_single_range() {
    let selection: MultiSelectionData<i32> = 2.into();
    let up_to_5 = selection
        .alter(5, &(0..=20).collect(), true, false)
        .unwrap();
    let also_10 = up_to_5.alter(10, &(0..=20).collect(), false, true).unwrap();
    let up_to_15 = also_10.alter(15, &(0..=20).collect(), true, true).unwrap();
    let down_to_12 = up_to_15
        .alter(12, &(0..=20).collect(), true, false)
        .unwrap();
    for i in 0..=20 {
        assert_eq!(down_to_12.contains(&i), i >= 12 && i <= 15);
    }
}

#[test]
fn can_adjust_multiple_ranges() {
    let selection: MultiSelectionData<i32> = 2.into();
    let up_to_5 = selection
        .alter(5, &(0..=20).collect(), true, false)
        .unwrap();
    let also_10 = up_to_5.alter(10, &(0..=20).collect(), false, true).unwrap();
    let up_to_15 = also_10.alter(15, &(0..=20).collect(), true, true).unwrap();
    let up_to_18 = up_to_15.alter(18, &(0..=20).collect(), true, true).unwrap();
    for i in 0..=20 {
        assert_eq!(
            up_to_18.contains(&i),
            (i >= 2 && i <= 5) || (i >= 10 && i <= 18)
        );
    }
    let except_16 = up_to_18
        .alter(16, &(0..=20).collect(), false, true)
        .unwrap();
    let down_to_12 = except_16
        .alter(12, &(0..=20).collect(), true, true)
        .unwrap();
    for i in 0..=20 {
        assert_eq!(
            down_to_12.contains(&i),
            (i >= 2 && i <= 5) || (i >= 10 && i <= 11) || (i >= 17 && i <= 18)
        );
    }
}
