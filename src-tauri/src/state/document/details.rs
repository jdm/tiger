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

    pub(super) fn set_hitbox_width(&mut self, width: u32) -> Result<(), DocumentError> {
        for (_, hitbox) in self.get_selected_hitboxes_mut()? {
            let new_size = vec2(width, hitbox.size().y);
            hitbox.set_size(new_size)
        }
        Ok(())
    }

    pub(super) fn set_hitbox_height(&mut self, height: u32) -> Result<(), DocumentError> {
        for (_, hitbox) in self.get_selected_hitboxes_mut()? {
            let new_size = vec2(hitbox.size().x, height);
            hitbox.set_size(new_size)
        }
        Ok(())
    }
}
