use euclid::{default::Vector2D, vec2};
use std::time::Duration;

use crate::document::*;

impl Document {
    pub(super) fn set_keyframe_duration(&mut self, duration: Duration) -> DocumentResult<()> {
        for (_, _, keyframe) in self.selected_keyframes_mut()? {
            keyframe.set_duration_millis(duration.as_millis() as u64);
        }
        Ok(())
    }

    pub(super) fn set_keyframe_offset_x(&mut self, x: i32) -> DocumentResult<()> {
        for (_, _, keyframe) in self.selected_keyframes_mut()? {
            Document::nudge_keyframe(keyframe, vec2(x, keyframe.offset().y));
        }
        Ok(())
    }

    pub(super) fn set_keyframe_offset_y(&mut self, y: i32) -> DocumentResult<()> {
        for (_, _, keyframe) in self.selected_keyframes_mut()? {
            Document::nudge_keyframe(keyframe, vec2(keyframe.offset().x, y));
        }
        Ok(())
    }

    pub(super) fn create_hitbox(&mut self, position: Option<Vector2D<i32>>) -> DocumentResult<()> {
        let (animation_name, _) = self.workbench_animation()?;
        let animation_name = animation_name.clone();
        let ((direction, index), keyframe) = self.workbench_keyframe_mut()?;
        let (hitbox_name, hitbox) = keyframe.create_hitbox("New Hitbox");
        if let Some(position) = position {
            hitbox.set_position(position);
        }
        self.select_hitbox_only(animation_name, direction, index, hitbox_name);
        Ok(())
    }

    pub(super) fn rename_hitbox<T: AsRef<str>, U: AsRef<str>>(
        &mut self,
        old_name: T,
        new_name: U,
    ) -> DocumentResult<()> {
        let (animation_name, _) = self.workbench_animation()?;
        let animation_name = animation_name.clone();
        let ((direction, index), keyframe) = self.workbench_keyframe_mut()?;
        keyframe.rename_hitbox(&old_name, &new_name)?;

        self.select_hitbox_only(animation_name, direction, index, new_name);
        Ok(())
    }

    pub(super) fn delete_hitbox<T: AsRef<str>>(&mut self, name: T) -> DocumentResult<()> {
        let (_, keyframe) = self.workbench_keyframe_mut()?;
        keyframe.delete_hitbox(&name);
        Ok(())
    }

    pub(super) fn delete_selected_hitboxes(&mut self) -> DocumentResult<()> {
        let selected_hitboxes = self
            .view
            .selection
            .hitboxes()
            .map(|(_, _, _, h)| h.clone())
            .collect::<Vec<_>>();
        if let Ok((_, keyframe)) = self.workbench_keyframe_mut() {
            for hitbox_name in selected_hitboxes {
                keyframe.delete_hitbox(hitbox_name);
            }
        }
        Ok(())
    }

    pub(super) fn set_hitbox_position_x(&mut self, x: i32) -> DocumentResult<()> {
        for (_, hitbox) in self.selected_hitboxes_mut()? {
            let new_position = vec2(x, hitbox.position().y);
            hitbox.set_position(new_position)
        }
        Ok(())
    }

    pub(super) fn set_hitbox_position_y(&mut self, y: i32) -> DocumentResult<()> {
        for (_, hitbox) in self.selected_hitboxes_mut()? {
            let new_position = vec2(hitbox.position().x, y);
            hitbox.set_position(new_position)
        }
        Ok(())
    }

    pub(super) fn set_hitbox_width(&mut self, new_width: u32) -> DocumentResult<()> {
        let preserve_ar = self.persistent.preserve_aspect_ratio;
        for (_, hitbox) in self.selected_hitboxes_mut()? {
            let new_height = match (preserve_ar, hitbox.size().x) {
                (false, _) => hitbox.size().y,
                (true, 0) => hitbox.size().y,
                (true, old_width) => new_width * hitbox.size().y / old_width,
            };
            hitbox.set_size(vec2(new_width, new_height))
        }
        Ok(())
    }

    pub(super) fn set_hitbox_height(&mut self, new_height: u32) -> DocumentResult<()> {
        let preserve_ar = self.persistent.preserve_aspect_ratio;
        for (_, hitbox) in self.selected_hitboxes_mut()? {
            let new_width = match (preserve_ar, hitbox.size().y) {
                (false, _) => hitbox.size().x,
                (true, 0) => hitbox.size().x,
                (true, old_height) => new_height * hitbox.size().x / old_height,
            };
            hitbox.set_size(vec2(new_width, new_height))
        }
        Ok(())
    }
}
