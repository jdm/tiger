use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::sheet::*;
use crate::state::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Clipboard {
    Animations(HashMap<String, Animation>),
    Keyframes(Vec<Keyframe>),
    Hitboxes(HashMap<String, Hitbox>),
}

impl Document {
    pub fn copy(&self) -> Option<Clipboard> {
        if !self.view.selection.animations.is_empty() {
            self.copy_animations()
        } else if !self.view.selection.keyframes.is_empty() {
            self.copy_keyframes()
        } else if !self.view.selection.hitboxes.is_empty() {
            self.copy_hitboxes()
        } else {
            None
        }
    }

    pub(super) fn paste(&mut self, clipboard: Clipboard) -> Result<(), DocumentError> {
        match clipboard {
            Clipboard::Animations(animations) => self.paste_animations(animations),
            Clipboard::Keyframes(keyframes) => self.paste_keyframes(keyframes),
            Clipboard::Hitboxes(hitboxes) => self.paste_hitboxes(hitboxes),
        }
    }

    fn copy_animations(&self) -> Option<Clipboard> {
        let animations = self
            .get_selected_animations()
            .into_iter()
            .map(|(name, animation)| (name.clone(), animation.clone()))
            .collect::<HashMap<String, Animation>>();
        Some(Clipboard::Animations(animations))
    }

    fn paste_animations(
        &mut self,
        animations: HashMap<String, Animation>,
    ) -> Result<(), DocumentError> {
        let mut new_animation_names = vec![];
        for (name, animation) in animations {
            let (new_animation_name, new_animation) = self.sheet.create_animation(name);
            new_animation_names.push(new_animation_name);
            *new_animation = animation.duplicate();
        }
        self.select_animations_only(new_animation_names);
        Ok(())
    }

    fn copy_keyframes(&self) -> Option<Clipboard> {
        let keyframes = self
            .get_selected_keyframes()
            .ok()?
            .into_iter()
            .map(|(_, _, keyframe)| keyframe.clone())
            .collect();
        Some(Clipboard::Keyframes(keyframes))
    }

    fn paste_keyframes(&mut self, keyframes: Vec<Keyframe>) -> Result<(), DocumentError> {
        let index = self
            .get_workbench_keyframe()
            .map(|((_, index), _)| index)
            .unwrap_or_default();
        let (animation_name, _) = self.get_workbench_animation_mut()?;
        let num_keyframes = keyframes.len();
        let (direction, sequence) = self.get_workbench_sequence_mut()?;
        for keyframe in keyframes.into_iter().rev() {
            let new_keyframe = keyframe.duplicate();
            sequence.insert_keyframe(new_keyframe, index)?;
        }
        self.select_keyframes_only(
            (index..(index + num_keyframes)).map(|i| (animation_name.clone(), direction, i)),
        );
        Ok(())
    }

    fn copy_hitboxes(&self) -> Option<Clipboard> {
        let hitboxes = self
            .get_selected_hitboxes()
            .ok()?
            .into_iter()
            .map(|(name, hitbox)| (name.clone(), hitbox.clone()))
            .collect::<HashMap<String, Hitbox>>();
        Some(Clipboard::Hitboxes(hitboxes))
    }

    fn paste_hitboxes(&mut self, hitboxes: HashMap<String, Hitbox>) -> Result<(), DocumentError> {
        let (animation_name, _) = self.get_workbench_animation_mut()?;
        let ((direction, index), keyframe) = self.get_workbench_keyframe_mut()?;
        let mut new_hitbox_names = vec![];
        for (name, hitbox) in hitboxes {
            let (new_hitbox_name, new_hitbox) = keyframe.create_hitbox(name);
            new_hitbox_names.push(new_hitbox_name);
            *new_hitbox = hitbox.duplicate();
        }
        self.select_hitboxes_only(
            new_hitbox_names
                .into_iter()
                .map(|n| (animation_name.clone(), direction, index, n)),
        );
        Ok(())
    }
}

#[test]
fn can_copy_paste_animation() {
    let mut document = Document::new("tmp");
    document.sheet = Sheet::default();
    let (_, animation) = document.sheet.create_animation("animation");
    animation.apply_direction_preset(DirectionPreset::EightDirections);

    document.select_animation_only("animation".to_owned());
    let clipboard = document.copy().unwrap();
    document.paste(clipboard).unwrap();

    assert_eq!(document.sheet.animations_iter().count(), 2);
    assert!(document.view.selection.is_animation_selected("animation 2"));
}

#[test]
fn can_copy_paste_keyframe() {
    let mut document = Document::new("tmp");
    document.sheet = Sheet::default();
    let (_, animation) = document.sheet.create_animation("animation");
    animation.apply_direction_preset(DirectionPreset::EightDirections);

    {
        let mut keyframe = Keyframe::new("frame.png");
        keyframe.create_hitbox("hitbox");
        animation
            .sequence_mut(Direction::East)
            .unwrap()
            .insert_keyframe(keyframe, 0)
            .unwrap();
    }

    document.edit_animation("animation").unwrap();
    document.select_keyframe_only("animation".to_owned(), Direction::East, 0);
    let clipboard = document.copy().unwrap();

    document.select_direction(Direction::South).unwrap();
    document.paste(clipboard).unwrap();

    assert!(document
        .sheet
        .animation("animation")
        .unwrap()
        .sequence(Direction::South)
        .unwrap()
        .keyframe(0)
        .is_some());
}

#[test]
fn can_copy_paste_hitbox() {
    let mut document = Document::new("tmp");
    document.sheet = Sheet::default();
    document.sheet.add_frame("frame.png");
    let (_, animation) = document.sheet.create_animation("animation");
    animation.apply_direction_preset(DirectionPreset::EightDirections);

    {
        let mut keyframe = Keyframe::new("frame.png");
        keyframe.create_hitbox("hitbox");
        animation
            .sequence_mut(Direction::East)
            .unwrap()
            .insert_keyframe(keyframe, 0)
            .unwrap();
    }

    {
        let keyframe = Keyframe::new("frame.png");
        animation
            .sequence_mut(Direction::North)
            .unwrap()
            .insert_keyframe(keyframe, 0)
            .unwrap();
    }

    document.edit_animation("animation").unwrap();
    document.select_hitbox_only("animation", Direction::East, 0, "hitbox");
    let clipboard = document.copy().unwrap();

    document.select_direction(Direction::North).unwrap();
    document.paste(clipboard).unwrap();
    assert!(document
        .sheet
        .animation("animation")
        .unwrap()
        .sequence(Direction::North)
        .unwrap()
        .keyframe(0)
        .unwrap()
        .has_hitbox("hitbox"));
    assert!(document
        .view
        .selection
        .is_hitbox_selected("animation", Direction::North, 0, "hitbox"));
}
