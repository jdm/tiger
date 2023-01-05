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
        self.relocate_frames_edit_mut()?.insert(from, to);
        Ok(())
    }

    pub(super) fn end_relocate_frames(&mut self) -> DocumentResult<()> {
        let mapping = std::mem::take(&mut self.persistent.relocate_frames_edit)
            .ok_or(DocumentError::NotRelocatingFrames)?;
        self.sheet.relocate_frames(&mapping);
        Ok(())
    }
}
