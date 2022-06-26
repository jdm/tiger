use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::sheet::*;
use crate::state::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Clipboard {
    Hitboxes(HashMap<String, Hitbox>),
}

impl Document {
    pub fn copy(&self) -> Option<Clipboard> {
        if !self.view.selection.hitboxes.is_empty() {
            let hitboxes = self
                .get_selected_hitboxes()
                .ok()?
                .into_iter()
                .map(|(name, hitbox)| (name.clone(), hitbox.clone()))
                .collect::<HashMap<String, Hitbox>>();
            Some(Clipboard::Hitboxes(hitboxes))
        } else {
            None
        }
    }

    pub(super) fn paste(&mut self, clipboard: Clipboard) -> Result<(), DocumentError> {
        match clipboard {
            Clipboard::Hitboxes(hitboxes) => {
                let (_, keyframe) = self.get_workbench_keyframe_mut()?;
                for (name, hitbox) in hitboxes {
                    let (_, new_hitbox) = keyframe.create_hitbox(name);
                    *new_hitbox = hitbox;
                }
            }
        }
        Ok(())
    }
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
