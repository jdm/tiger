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
}
