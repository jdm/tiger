use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::sheet::*;
use crate::state::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Clipboard {
    Animations(HashMap<String, Animation>),
    Hitboxes(HashMap<String, Hitbox>),
}

impl Document {
    pub fn copy(&self) -> Option<Clipboard> {
        if !self.view.selection.animations.is_empty() {
            self.copy_animations()
        } else if !self.view.selection.hitboxes.is_empty() {
            self.copy_hitboxes()
        } else {
            None
        }
    }

    pub(super) fn paste(&mut self, clipboard: Clipboard) -> Result<(), DocumentError> {
        match clipboard {
            Clipboard::Animations(animations) => self.paste_animations(animations),
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
        for (name, animation) in animations {
            let (_, new_animation) = self.sheet.create_animation(name);
            *new_animation = animation.duplicate();
        }
        // TODO select new animations
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
        let (_, keyframe) = self.get_workbench_keyframe_mut()?;
        for (name, hitbox) in hitboxes {
            let (_, new_hitbox) = keyframe.create_hitbox(name);
            *new_hitbox = hitbox.duplicate();
        }
        // TODO select pasted hitboxes
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
    document.select_hitbox_only(
        "animation".to_owned(),
        Direction::East,
        0,
        "hitbox".to_owned(),
    );
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
}
