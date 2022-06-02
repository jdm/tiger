use crate::state::*;

impl Document {
    pub(super) fn create_animation(&mut self) -> Result<(), DocumentError> {
        let animation_name = {
            let (animation_name, _) = self.sheet.create_animation();
            animation_name
        };
        self.view.selection.select_animation(animation_name.clone());
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
        self.view
            .selection
            .select_animation(new_name.as_ref().to_owned());
        if Some(old_name.as_ref()) == self.view.current_animation().as_deref() {
            self.view.current_animation = Some(new_name.as_ref().to_owned());
        }
        Ok(())
    }

    pub(super) fn delete_animation<T: AsRef<str>>(&mut self, name: T) {
        self.sheet.delete_animation(&name);
    }
}
