use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::document::*;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Clipboard {
    Animations(HashMap<String, Animation<Absolute>>),
    Keyframes(Vec<Keyframe<Absolute>>),
    Hitboxes(HashMap<String, Hitbox>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClipboardManifest {
    Animations,
    Keyframes,
    Hitboxes,
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

    pub(super) fn paste(&mut self, clipboard: Clipboard) -> DocumentResult<()> {
        match clipboard {
            Clipboard::Animations(animations) => self.paste_animations(animations),
            Clipboard::Keyframes(keyframes) => self.paste_keyframes(keyframes),
            Clipboard::Hitboxes(hitboxes) => self.paste_hitboxes(hitboxes),
        }
    }

    fn copy_animations(&self) -> Option<Clipboard> {
        let animations = self
            .selected_animations()
            .into_iter()
            .map(|(name, animation)| (name.clone(), animation.clone()))
            .collect::<HashMap<String, Animation<Absolute>>>();
        Some(Clipboard::Animations(animations))
    }

    fn paste_animations(
        &mut self,
        animations: HashMap<String, Animation<Absolute>>,
    ) -> DocumentResult<()> {
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
            .selected_keyframes()
            .ok()?
            .into_iter()
            .map(|(_, _, keyframe)| keyframe.clone())
            .collect();
        Some(Clipboard::Keyframes(keyframes))
    }

    fn paste_keyframes(&mut self, keyframes: Vec<Keyframe<Absolute>>) -> DocumentResult<()> {
        let (_, sequence) = self.workbench_sequence()?;
        let at_sequence_end = self.timeline_clock() >= sequence.duration().unwrap_or_default();
        let index = if at_sequence_end {
            sequence.num_keyframes()
        } else {
            self.workbench_keyframe()
                .map(|((_, index), _)| index)
                .unwrap_or_default()
        };
        let (animation_name, _) = self.workbench_animation_mut()?;
        let num_keyframes = keyframes.len();
        let (direction, sequence) = self.workbench_sequence_mut()?;
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
            .selected_hitboxes()
            .ok()?
            .into_iter()
            .map(|(name, hitbox)| (name.clone(), hitbox.clone()))
            .collect::<HashMap<String, Hitbox>>();
        Some(Clipboard::Hitboxes(hitboxes))
    }

    fn paste_hitboxes(&mut self, hitboxes: HashMap<String, Hitbox>) -> DocumentResult<()> {
        let (animation_name, _) = self.workbench_animation_mut()?;
        let ((direction, index), keyframe) = self.workbench_keyframe_mut()?;
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

impl Clipboard {
    pub fn manifest(&self) -> ClipboardManifest {
        match self {
            Clipboard::Animations(_) => ClipboardManifest::Animations,
            Clipboard::Keyframes(_) => ClipboardManifest::Keyframes,
            Clipboard::Hitboxes(_) => ClipboardManifest::Hitboxes,
        }
    }
}

pub fn clipboard_manifest<S: AsRef<str>>(clipboard_content: S) -> Option<ClipboardManifest> {
    serde_json::from_str::<Clipboard>(clipboard_content.as_ref())
        .ok()
        .map(|c| c.manifest())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn can_copy_paste_animation() {
        let mut document = Document::new("tmp");
        document.sheet.create_animation("animation");

        document.select_animation_only("animation".to_owned());
        let clipboard = document.copy().unwrap();
        document.paste(clipboard).unwrap();

        assert_eq!(document.sheet.animations_iter().count(), 2);
        assert!(document.view.selection.is_animation_selected("animation 2"));
    }

    #[test]
    fn can_copy_paste_keyframe() {
        let mut document = Document::new("tmp");

        document.sheet.add_test_animation(
            "animation",
            HashMap::from([
                (Direction::East, vec!["frame.png"]),
                (Direction::West, vec![]),
            ]),
        );

        let keyframe = document.sheet.keyframe_mut("animation", Direction::East, 0);
        keyframe.create_hitbox("hitbox");

        document.edit_animation("animation").unwrap();
        document.select_keyframe_only("animation".to_owned(), Direction::East, 0);
        let clipboard = document.copy().unwrap();

        document.select_direction(Direction::West).unwrap();
        document.paste(clipboard).unwrap();

        assert!(document
            .sheet
            .animation("animation")
            .unwrap()
            .sequence(Direction::West)
            .unwrap()
            .keyframe(0)
            .is_some());
    }

    #[test]
    fn can_copy_paste_hitbox() {
        let mut document = Document::new("tmp");

        document.sheet.add_test_animation(
            "animation",
            HashMap::from([
                (Direction::East, vec!["frame.png"]),
                (Direction::West, vec!["frame.png"]),
            ]),
        );

        let keyframe = document.sheet.keyframe_mut("animation", Direction::East, 0);
        keyframe.create_hitbox("hitbox");

        document.edit_animation("animation").unwrap();
        document.select_hitbox_only("animation", Direction::East, 0, "hitbox");
        let clipboard = document.copy().unwrap();

        document.select_direction(Direction::West).unwrap();
        document.paste(clipboard).unwrap();
        assert!(document
            .sheet
            .animation("animation")
            .unwrap()
            .sequence(Direction::West)
            .unwrap()
            .keyframe(0)
            .unwrap()
            .has_hitbox("hitbox"));
        assert!(document.view.selection.is_hitbox_selected(
            "animation",
            Direction::West,
            0,
            "hitbox"
        ));
    }
}
