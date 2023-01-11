use image::DynamicImage;
use log::error;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{collections::HashSet, time::Duration};

use crate::{
    app::TigerApp,
    utils::{file_watcher::FileWatcher, handle, texture_list::TextureList},
};

pub type Handle = handle::Handle<HashMap<PathBuf, DynamicImage>>;

impl Handle {
    pub fn init<A: TigerApp + Send + Clone + 'static>(&self, app: A, period: Duration) {
        let (mut file_watcher, events_receiver) = FileWatcher::new({
            let app = app.clone();
            move || {
                let state_handle = app.state();
                let state = state_handle.lock();
                state.list_textures()
            }
        });

        std::thread::spawn(move || loop {
            file_watcher.update_watched_files();
            std::thread::sleep(period);
        });

        std::thread::spawn({
            let texture_cache = self.clone();
            move || loop {
                if let Ok(Ok(events)) = events_receiver.recv() {
                    remove(
                        &events.iter().map(|e| e.path.to_owned()).collect(),
                        &texture_cache,
                    )
                }
            }
        });

        std::thread::spawn({
            let texture_cache = self.clone();
            move || loop {
                let current_entries: HashSet<PathBuf> = {
                    let cache = texture_cache.lock();
                    cache.keys().cloned().collect()
                };
                let desired_entries = {
                    let state_handle = app.state();
                    let state = state_handle.lock();
                    state.list_textures()
                };
                let missing_entries = desired_entries
                    .iter()
                    .filter(|p| !current_entries.contains(*p))
                    .collect::<HashSet<_>>();
                let extraneous_entries = current_entries
                    .iter()
                    .filter(|p| !desired_entries.contains(*p))
                    .collect::<HashSet<_>>();
                remove(&extraneous_entries, &texture_cache);
                add(&missing_entries, &texture_cache);
                std::thread::sleep(period);
            }
        });
    }
}

fn remove<P: AsRef<Path>>(textures: &HashSet<P>, texture_cache: &Handle) {
    let mut cache = texture_cache.lock();
    for path in textures {
        cache.remove(path.as_ref());
    }
}

fn add<P: AsRef<Path>>(textures: &HashSet<P>, texture_cache: &Handle) {
    {
        let mut cache = texture_cache.lock();
        for texture in textures {
            cache.remove(texture.as_ref());
        }
    }

    let mut new_textures = HashMap::<PathBuf, DynamicImage>::new();
    for path in textures {
        match image::open(path) {
            Ok(i) => {
                new_textures.insert(path.as_ref().to_owned(), i);
            }
            Err(e) => {
                error!(
                    "Error while preloading `{0}`: {e}",
                    path.as_ref().to_string_lossy()
                );
            }
        };
    }

    {
        let mut cache = texture_cache.lock();
        for (path, texture) in new_textures {
            cache.insert(path.to_owned(), texture);
        }
    }
}

#[cfg(test)]
mod tests {

    use std::path::PathBuf;
    use sugar_path::SugarPath;

    use crate::{app::TigerApp, mock::TigerAppMock};

    #[tokio::test]
    async fn follows_frame_additions_and_removals() {
        let frame_path = PathBuf::from("test-data/samurai-dead-all.png").resolve();
        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        app.assert_eventually(|| app.texture_cache().lock().contains_key(&frame_path));
        app.delete_frame(frame_path.clone());
        app.assert_eventually(|| !app.texture_cache().lock().contains_key(&frame_path));
    }

    #[tokio::test]
    async fn detects_texture_changes() {
        let frame = PathBuf::from("test-output/detects_texture_changes.png").resolve();
        let before_frame = PathBuf::from("test-data/samurai-dead-all.png").resolve();
        let after_frame = PathBuf::from("test-data/samurai-attack-north.png").resolve();

        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        app.import_frames(vec![frame.clone()]);

        std::fs::copy(&before_frame, &frame).unwrap();
        app.assert_eventually({
            let app = app.clone();
            let frame = frame.clone();
            move || {
                app.texture_cache().lock().get(&frame) == Some(&image::open(&before_frame).unwrap())
            }
        });

        std::fs::copy(&after_frame, &frame).unwrap();
        app.assert_eventually({
            let app = app.clone();
            let frame = frame.clone();
            move || {
                app.texture_cache().lock().get(&frame) == Some(&image::open(&after_frame).unwrap())
            }
        });
    }
}
