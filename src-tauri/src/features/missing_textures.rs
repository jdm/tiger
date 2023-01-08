use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::time::Duration;

use crate::dto::StateTrim;
use crate::utils::texture_list::TextureList;
use crate::TigerApp;

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
mod test {

    use std::{fs::File, path::PathBuf};

    use crate::mock::TigerAppMock;

    #[test]
    fn detects_texture_addition_and_removal() {
        let filename = "test-output/detects_removed_texture.png";
        std::fs::remove_file(filename).ok();

        let is_missing =
            |app: &TigerAppMock| app.client_state().documents[0].sheet.frames[0].missing_on_disk;

        let app = TigerAppMock::new();

        app.new_document("tmp.tiger".into());
        app.import_frames(vec![PathBuf::from(filename)]);
        app.wait_for_periodic_scans();
        assert!(is_missing(&app));

        File::create(filename).unwrap();
        app.wait_for_periodic_scans();
        assert!(!is_missing(&app));

        std::fs::remove_file(filename).ok();
        app.wait_for_periodic_scans();
        assert!(is_missing(&app));
    }
}
