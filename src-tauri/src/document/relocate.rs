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

    use retry::{delay::Fixed, retry};
    use sugar_path::SugarPath;

    use super::*;
    use crate::app::mock::TigerAppMock;

    #[test]
    fn can_relocate_frames() {
        let bad_dead = PathBuf::from("dead-all.png").resolve();
        let bad_idle = PathBuf::from("idle-west.png").resolve();
        let good_dead = PathBuf::from("test-data/samurai/dead-all.png").resolve();
        let good_idle = PathBuf::from("test-data/samurai/idle-west.png").resolve();
        let unrelated = PathBuf::from("unrelated.png").resolve();

        let app = TigerAppMock::new();
        app.new_document("tmp.tiger");
        app.import_frames(vec![&bad_dead, &bad_idle, &unrelated]);
        let has_frame = |p: &PathBuf| app.document().sheet.frames.iter().any(|f| &f.path == p);

        let detected_missing_frames = retry(Fixed::from_millis(500).take(10), || {
            if !app.document().frame("dead-all").missing_on_disk {
                Err("dead-all not detected as missing")
            } else if !app.document().frame("idle-west").missing_on_disk {
                Err("idle-west not detected as missing")
            } else {
                Ok(())
            }
        });
        assert_eq!(Ok(()), detected_missing_frames);

        app.begin_relocate_frames();
        assert!(app.document().frames_being_relocated.is_some());
        app.relocate_frame(&bad_dead, &good_dead);
        app.cancel_relocate_frames();

        assert!(app.document().frames_being_relocated.is_none());
        assert!(has_frame(&bad_dead));
        assert!(has_frame(&bad_idle));
        assert!(has_frame(&unrelated));
        assert!(!has_frame(&good_dead));
        assert!(!has_frame(&good_idle));

        app.begin_relocate_frames();
        app.relocate_frame(&bad_dead, &good_dead);
        app.end_relocate_frames();

        assert!(!has_frame(&bad_dead));
        assert!(!has_frame(&bad_idle));
        assert!(has_frame(&good_dead));
        assert!(has_frame(&good_idle));
        assert!(has_frame(&unrelated));
    }
}
