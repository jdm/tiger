use euclid::vec2;

use crate::state::*;

impl Document {
    pub(super) fn set_hitbox_position_x(&mut self, x: i32) -> Result<(), DocumentError> {
        for (_, hitbox) in self.get_selected_hitboxes_mut()? {
            let new_position = vec2(x, hitbox.position().y);
            hitbox.set_position(new_position)
        }
        Ok(())
    }

    pub(super) fn set_hitbox_position_y(&mut self, y: i32) -> Result<(), DocumentError> {
        for (_, hitbox) in self.get_selected_hitboxes_mut()? {
            let new_position = vec2(hitbox.position().x, y);
            hitbox.set_position(new_position)
        }
        Ok(())
    }

    pub(super) fn set_hitbox_width(&mut self, new_width: u32) -> Result<(), DocumentError> {
        let preserve_ar = self.persistent.preserve_aspect_ratio;
        for (_, hitbox) in self.get_selected_hitboxes_mut()? {
            let new_height = match (preserve_ar, hitbox.size().x) {
                (false, _) => hitbox.size().y,
                (true, 0) => hitbox.size().y,
                (true, old_width) => new_width * hitbox.size().y / old_width,
            };
            hitbox.set_size(vec2(new_width, new_height))
        }
        Ok(())
    }

    pub(super) fn set_hitbox_height(&mut self, new_height: u32) -> Result<(), DocumentError> {
        let preserve_ar = self.persistent.preserve_aspect_ratio;
        for (_, hitbox) in self.get_selected_hitboxes_mut()? {
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
