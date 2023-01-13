use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::time::Duration;

use crate::{app::TigerApp, dto::StateTrim, utils::texture_list::TextureList};

pub fn init<A: TigerApp + Send + Clone + 'static>(app: A, period: Duration) {
    std::thread::spawn(move || loop {
        std::thread::sleep(period);

        let (all_textures, old_missing_textures) = {
            let state_handle = app.state();
            let state = state_handle.lock();
            let mut all_textures = HashMap::new();
            let mut old_missing_textures = HashMap::new();
            for document in state.documents_iter() {
                all_textures.insert(document.path().to_owned(), document.list_textures());
                old_missing_textures.insert(
                    document.path().to_owned(),
                    document.missing_textures().clone(),
                );
            }
            (all_textures, old_missing_textures)
        };

        let mut new_missing_textures: HashMap<PathBuf, HashSet<PathBuf>> = all_textures
            .into_iter()
            .map(|(p, t)| (p, t.into_iter().filter(|t| !t.exists()).collect()))
            .collect();

        if old_missing_textures != new_missing_textures {
            app.patch_state(StateTrim::Full, |state| {
                for document in state.documents_iter_mut() {
                    if let Some(textures) = new_missing_textures.remove(document.path()) {
                        document.set_missing_textures(textures);
                    }
                }
            });
        }
    });
}

#[cfg(test)]
mod tests {

    use std::{fs::File, path::PathBuf};

    use crate::mock::TigerAppMock;

    #[test]
    fn detects_texture_addition_and_removal() {
        let filename = "test-output/detects_texture_addition_and_removal.png";
        std::fs::remove_file(filename).ok();

        let is_missing = |app: &TigerAppMock| app.document().sheet.frames[0].missing_on_disk;

        let app = TigerAppMock::new();

        app.new_document("tmp.tiger");
        app.import_frames(vec![PathBuf::from(filename)]);
        app.assert_eventually(|| is_missing(&app));

        File::create(filename).unwrap();
        app.assert_eventually(|| !is_missing(&app));

        std::fs::remove_file(filename).ok();
        app.assert_eventually(|| is_missing(&app));
    }
}
