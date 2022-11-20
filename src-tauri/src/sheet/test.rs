use std::collections::HashMap;

use super::*;

impl Sheet {
    pub fn add_test_animation<N: AsRef<str>, P: AsRef<Path>>(
        &mut self,
        animation_name: N,
        content: HashMap<Direction, Vec<P>>,
    ) {
        let (effective_animation_name, animation) = self.create_animation(animation_name.as_ref());
        assert_eq!(animation_name.as_ref(), effective_animation_name);

        if content.is_empty() {
            animation.apply_direction_preset(DirectionPreset::EightDirections);
        } else {
            let direction_preset =
                DirectionPreset::from_directions(content.keys().copied()).unwrap();
            animation.apply_direction_preset(direction_preset);
        }

        for (direction, frames) in content {
            for frame in &frames {
                self.add_frame(frame);
            }
            let animation = self.animation_mut(animation_name.as_ref()).unwrap();
            let sequence = animation.sequence_mut(direction).unwrap();
            for frame in frames.iter().rev() {
                sequence.insert_keyframe(Keyframe::new(frame), 0).unwrap();
            }
        }
    }

    pub fn keyframe<T: AsRef<str>>(
        &self,
        animation_name: T,
        direction: Direction,
        index: usize,
    ) -> &Keyframe {
        let animation = self.animation(animation_name).unwrap();
        let sequence = animation.sequence(direction).unwrap();
        sequence.keyframe(index).unwrap()
    }

    pub fn keyframe_mut<T: AsRef<str>>(
        &mut self,
        animation_name: T,
        direction: Direction,
        index: usize,
    ) -> &mut Keyframe {
        let animation = self.animation_mut(animation_name).unwrap();
        let sequence = animation.sequence_mut(direction).unwrap();
        sequence.keyframe_mut(index).unwrap()
    }

    pub fn hitbox<T: AsRef<str>, U: AsRef<str>>(
        &self,
        animation_name: T,
        direction: Direction,
        index: usize,
        hitbox_name: U,
    ) -> &Hitbox {
        let keyframe = self.keyframe(animation_name, direction, index);
        keyframe.hitboxes.get(hitbox_name.as_ref()).unwrap()
    }

    pub fn hitbox_mut<T: AsRef<str>, U: AsRef<str>>(
        &mut self,
        animation_name: T,
        direction: Direction,
        index: usize,
        hitbox_name: U,
    ) -> &mut Hitbox {
        let keyframe = self.keyframe_mut(animation_name, direction, index);
        keyframe.hitboxes.get_mut(hitbox_name.as_ref()).unwrap()
    }
}
