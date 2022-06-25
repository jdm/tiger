use crate::sheet::DirectionPreset;
use crate::state::*;

impl Document {
    pub(super) fn delete_selected_frames(&mut self) {
        let selected_frames = self.view.selection.frames().collect::<Vec<_>>();
        for frame in selected_frames {
            self.sheet.delete_frame(frame);
        }
    }

    pub(super) fn create_animation(&mut self) -> Result<(), DocumentError> {
        let (animation_name, animation) = self.sheet.create_animation();
        animation.apply_direction_preset(DirectionPreset::FourDirections);
        self.select_animation_only(animation_name.clone());
        self.edit_animation(animation_name)
    }

    pub(super) fn edit_animation<T: AsRef<str>>(&mut self, name: T) -> Result<(), DocumentError> {
        self.view.current_animation = Some(name.as_ref().to_owned());
        self.view.center_workbench();
        self.view.skip_to_timeline_start();
        self.persistent.timeline_is_playing = false;
        Ok(())
    }

    pub(super) fn rename_animation<T: AsRef<str>, U: AsRef<str>>(
        &mut self,
        old_name: T,
        new_name: U,
    ) -> Result<(), DocumentError> {
        self.sheet.rename_animation(&old_name, &new_name)?;
        self.select_animation_only(new_name.as_ref().to_owned());
        if Some(old_name.as_ref()) == self.current_animation().as_deref() {
            self.view.current_animation = Some(new_name.as_ref().to_owned());
        }
        Ok(())
    }

    pub(super) fn delete_animation<T: AsRef<str>>(&mut self, name: T) {
        self.sheet.delete_animation(&name);
    }

    pub(super) fn delete_selected_animations(&mut self) {
        let selected_animations = self.view.selection.animations().collect::<Vec<_>>();
        for animation in selected_animations {
            self.sheet.delete_animation(animation);
        }
    }
}
