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

#[cfg(test)]
mod tests {

    use crate::{app::TigerApp, mock::TigerAppMock};

    use super::*;

    #[test]
    fn can_relocate_frames() {
        let app = TigerAppMock::new();
        let state_handle = app.state();

        {
            let mut state = state_handle.lock();
            state.new_document("tmp.tiger");
            state
                .current_document_mut()
                .unwrap()
                .sheet
                .add_frames(&vec![
                    "samurai-dead-all.png",
                    "samurai-idle-west.png",
                    "bad-frame.png",
                ]);
        }

        app.wait_for_periodic_scans();

        let mut state = state_handle.lock();
        let d = state.current_document_mut().unwrap();
        d.begin_relocate_frames();
        d.relocate_frame(
            PathBuf::from("samurai-dead-all.png"),
            PathBuf::from("test-data/samurai-dead-all.png"),
        )
        .unwrap();
        d.end_relocate_frames().unwrap();

        assert!(!d.sheet.has_frame("samurai-dead-all.png"));
        assert!(!d.sheet.has_frame("samurai-idle-west.png"));
        assert!(d.sheet.has_frame("bad-frame.png"));
        assert!(d.sheet.has_frame("test-data/samurai-dead-all.png"));
        assert!(d.sheet.has_frame("test-data/samurai-idle-west.png"));
        assert!(d.sheet.has_frame("bad-frame.png"));
    }
}
