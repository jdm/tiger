use crate::document::*;
use crate::sheet::DirectionPreset;

impl Document {
    pub(super) fn import_frames(&mut self, frames: &Vec<PathBuf>) {
        self.sheet.add_frames(frames);
        self.select_frames_only(frames.clone());
    }

    pub(super) fn delete_selected_frames(&mut self) {
        let selected_frames = self.view.selection.frames().collect::<Vec<_>>();
        for frame in selected_frames {
            self.sheet.delete_frame(frame);
        }
    }

    pub(super) fn create_animation(&mut self) -> DocumentResult<()> {
        let (animation_name, animation) = self.sheet.create_animation("New Animation");
        animation.apply_direction_preset(DirectionPreset::FourDirections);
        self.select_animation_only(animation_name.clone());
        self.edit_animation(&animation_name)?;
        self.begin_rename_animation(animation_name);
        Ok(())
    }

    pub(super) fn edit_animation<T: AsRef<str>>(&mut self, name: T) -> DocumentResult<()> {
        self.view.current_animation = Some(name.as_ref().to_owned());
        self.view.center_workbench();
        self.view.skip_to_timeline_start();
        self.view.reset_timeline_offset();
        self.persistent.timeline_is_playing = false;

        let (_, animation) = self.workbench_animation()?;
        if self.workbench_sequence().is_err() {
            let any_direction = animation.sequences_iter().next().map(|(d, _s)| *d);
            self.view.current_sequence = any_direction;
        }
        self.select_current_keyframe().ok();

        Ok(())
    }

    pub(super) fn rename_animation<T: AsRef<str>, U: AsRef<str>>(
        &mut self,
        old_name: T,
        new_name: U,
    ) -> DocumentResult<()> {
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

    pub fn set_missing_textures(&mut self, missing_textures: HashSet<PathBuf>) {
        self.persistent.missing_textures = missing_textures;
    }

    pub fn missing_textures(&self) -> &HashSet<PathBuf> {
        &self.persistent.missing_textures
    }

    pub fn is_frame_missing_on_disk<T: AsRef<Path>>(&self, frame: T) -> bool {
        self.persistent.missing_textures.contains(frame.as_ref())
    }
}

#[cfg(test)]
mod tests {

    use crate::app::mock::TigerAppMock;

    #[test]
    fn can_name_new_animation() {
        let app = TigerAppMock::new();
        app.new_document("tmp");
        app.create_animation();
        assert!(app.document().animation_being_renamed.is_some());
        app.end_rename_animation("can_create_animation");
        assert!(app.document().animations()[0].name == "can_create_animation");
    }

    #[test]
    fn can_create_and_delete_animation() {
        let app = TigerAppMock::new();
        app.new_document("tmp");
        app.create_animation();
        assert!(!app.document().animations().is_empty());
        app.delete_animation(app.document().current_animation_name.unwrap());
        assert!(app.document().animations().is_empty());
    }

    #[test]
    fn scrolling_does_not_cancel_animation_rename() {
        let app = TigerAppMock::new();
        app.new_document("tmp");
        app.create_animation();
        assert!(app.document().animation_being_renamed.is_some());
        app.set_animations_list_offset(50.0);
        app.set_hitboxes_list_offset(50.0);
        app.set_frames_list_offset(50.0);
        app.pan_timeline(50.0);
        assert!(app.document().animation_being_renamed.is_some());
    }
}
