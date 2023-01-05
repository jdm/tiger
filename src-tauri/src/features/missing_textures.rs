use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::time::Duration;

use crate::app::AppState;
use crate::dto::AppTrim;
use crate::utils::texture_list::TextureList;
use crate::TigerApp;

pub fn init<A: TigerApp + Send + Clone + 'static>(tiger_app: A, period: Duration) {
    std::thread::spawn(move || loop {
        std::thread::sleep(period);

        let (all_textures, old_missing_textures) = {
            let app_state = tiger_app.state::<AppState>();
            let app = app_state.0.lock();
            let mut all_textures = HashMap::new();
            let mut old_missing_textures = HashMap::new();
            for document in app.documents_iter() {
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
            tiger_app.patch_state(AppTrim::Full, |app| {
                for document in app.documents_iter_mut() {
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

    use std::{fs::File, path::PathBuf, time::Instant};

    use super::*;
    use crate::{app::AppState, document::Command, mock::TigerAppMock};

    fn assert_with_timeout<F: Fn() -> bool>(check: F) {
        let start = Instant::now();
        while Instant::now().duration_since(start) < Duration::from_secs(5) {
            if check() {
                return;
            }
        }
        panic!("assertion timeout");
    }

    #[test]
    fn detects_texture_addition_and_removal() {
        let filename = "test-output/detects_removed_texture.png";
        std::fs::remove_file(filename).ok();

        let is_missing = |app: &TigerAppMock| {
            let app_state = app.state::<AppState>();
            let app = app_state.0.lock();
            app.current_document()
                .unwrap()
                .is_frame_missing_on_disk(filename)
        };

        let app = TigerAppMock::new();
        init(app.clone(), Duration::from_millis(50));

        {
            let app_state = app.state::<AppState>();
            let mut app = app_state.0.lock();
            app.new_document("tmp.tiger");
            app.current_document_mut()
                .unwrap()
                .process_command(Command::ImportFrames(vec![PathBuf::from(filename)]))
                .unwrap();
        }
        assert_with_timeout(|| is_missing(&app));

        File::create(filename).unwrap();
        assert_with_timeout(|| !is_missing(&app));

        std::fs::remove_file(filename).ok();
        assert_with_timeout(|| is_missing(&app));
    }
}
