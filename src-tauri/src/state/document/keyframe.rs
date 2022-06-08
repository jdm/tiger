use crate::state::*;

impl Document {
    pub(super) fn rename_hitbox<T: AsRef<str>, U: AsRef<str>>(
        &mut self,
        old_name: T,
        new_name: U,
    ) -> Result<(), DocumentError> {
        let (animation_name, _) = self.get_workbench_animation()?;
        let animation_name = animation_name.clone();
        let ((direction, index), keyframe) = self.get_workbench_keyframe_mut()?;
        keyframe.rename_hitbox(&old_name, &new_name)?;

        self.view.selection.select_hitbox(
            animation_name,
            direction,
            index,
            new_name.as_ref().to_owned(),
        );
        Ok(())
    }

    pub(super) fn delete_hitbox<T: AsRef<str>>(&mut self, name: T) -> Result<(), DocumentError> {
        let (_, keyframe) = self.get_workbench_keyframe_mut()?;
        keyframe.delete_hitbox(&name);
        Ok(())
    }
}
