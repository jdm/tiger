use crate::document::*;

impl Document {
    pub fn relocate_frames_edit(&self) -> DocumentResult<&HashMap<PathBuf, PathBuf>> {
        self.persistent
            .relocate_frames_edit
            .as_ref()
            .ok_or(DocumentError::NotRelocatingFrames)
    }

    pub(super) fn relocate_frames_edit_mut(
        &mut self,
    ) -> DocumentResult<&mut HashMap<PathBuf, PathBuf>> {
        self.persistent
            .relocate_frames_edit
            .as_mut()
            .ok_or(DocumentError::NotRelocatingFrames)
    }

    pub(super) fn begin_relocate_frames(&mut self) {
        self.persistent.relocate_frames_edit = Some(HashMap::new());
    }

    pub(super) fn cancel_relocate_frames(&mut self) {
        self.persistent.relocate_frames_edit = None;
    }

    pub(super) fn relocate_frame(&mut self, from: PathBuf, to: PathBuf) -> DocumentResult<()> {
        self.relocate_frames_edit_mut()?
            .insert(from.clone(), to.clone());

        let Some(old_directory) = from.parent() else {
            return Ok(());
        };
        let Some(new_directory) = to.parent() else {
            return Ok(());
        };

        let mut automatic_relocations = HashMap::new();
        for frame in self.sheet().frames_iter() {
            if !self.is_frame_missing_on_disk(frame.source()) {
                continue;
            }
            if self.relocate_frames_edit()?.contains_key(frame.source()) {
                continue;
            }
            if frame.source().parent() != Some(old_directory) {
                continue;
            };
            let Some(file_name) = frame.source().file_name() else {
                continue;
            };
            let candidate_location = new_directory.join(file_name);
            if candidate_location.exists() {
                automatic_relocations.insert(frame.source().to_owned(), candidate_location);
            }
        }

        self.relocate_frames_edit_mut()?
            .extend(automatic_relocations);

        Ok(())
    }

    pub(super) fn end_relocate_frames(&mut self) -> DocumentResult<()> {
        let mapping = std::mem::take(&mut self.persistent.relocate_frames_edit)
            .ok_or(DocumentError::NotRelocatingFrames)?;
        self.sheet.relocate_frames(&mapping);
        Ok(())
    }
}
