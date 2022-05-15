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
    pub last_touched_item: T,
    pub last_touched_item_in_range: T,
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
            Some(TaggedMultiSelection::Frames(s)) => s.selected_items.contains(path),
            _ => false,
        }
    }

    pub fn is_animation_selected<T: AsRef<str>>(&self, name: T) -> bool {
        match &self.content {
            Some(TaggedMultiSelection::Animations(s)) => s.selected_items.contains(name.as_ref()),
            _ => false,
        }
    }

    pub fn is_hitbox_selected<T: AsRef<str>>(&self, name: T) -> bool {
        match &self.content {
            Some(TaggedMultiSelection::Hitboxes(s)) => s.selected_items.contains(name.as_ref()),
            _ => false,
        }
    }

    pub fn is_keyframe_selected(&self, index: usize) -> bool {
        match &self.content {
            Some(TaggedMultiSelection::Keyframes(s)) => s.selected_items.contains(&index),
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
            last_touched_item: items[items.len() - 1].clone(),
            last_touched_item_in_range: items[items.len() - 1].clone(),
            selected_items: items.into_iter().collect(),
        }
    }

    pub fn alter(
        mut self,
        interacted_item: T,
        all_items: &Vec<T>,
        shift: bool,
        ctrl: bool,
    ) -> Option<Self> {
        assert!(self.selected_items.len() > 0);

        let interacted_item_index = all_items
            .iter()
            .position(|item| *item == interacted_item)
            .unwrap_or(0);

        if shift {
            let from = {
                let last_touched_index = all_items
                    .iter()
                    .position(|item| item == &self.last_touched_item)
                    .unwrap_or(0);
                if last_touched_index < interacted_item_index {
                    last_touched_index + 1
                } else if last_touched_index > interacted_item_index {
                    last_touched_index - 1
                } else {
                    last_touched_index
                }
            };

            let mut affected_items = all_items
                [from.min(interacted_item_index)..=from.max(interacted_item_index)]
                .iter()
                .cloned()
                .collect::<Vec<T>>();

            if from > interacted_item_index {
                affected_items = affected_items.into_iter().rev().collect();
            }

            if ctrl {
                self.toggle(&affected_items);
            } else {
                self.add(&affected_items);
            }
        } else if ctrl {
            self.toggle(&vec![interacted_item.clone()]);
        } else {
            self = interacted_item.clone().into();
        }

        if self.selected_items.is_empty() {
            None
        } else {
            Some(self)
        }
    }

    fn add(&mut self, added_items: &Vec<T>) {
        if added_items.len() == 0 {
            return;
        }

        self.last_touched_item = added_items[added_items.len() - 1].clone();
        self.last_touched_item_in_range = added_items[added_items.len() - 1].clone();

        let added: HashSet<T> = added_items.iter().cloned().collect();
        self.selected_items = self.selected_items.union(&added).cloned().collect();
    }

    fn toggle(&mut self, toggled_items: &Vec<T>) {
        if toggled_items.len() == 0 {
            return;
        }

        self.last_touched_item = toggled_items[toggled_items.len() - 1].clone();

        let toggled: HashSet<T> = toggled_items.iter().cloned().collect();
        self.selected_items = self
            .selected_items
            .symmetric_difference(&toggled)
            .cloned()
            .collect();

        if self.selected_items.len() > 0 {
            self.last_touched_item_in_range = self.selected_items.iter().max().unwrap().clone();
            for item in toggled_items {
                if self.selected_items.contains(&item) {
                    self.last_touched_item_in_range = item.clone();
                }
            }
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
