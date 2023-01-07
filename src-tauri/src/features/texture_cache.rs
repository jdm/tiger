use image::DynamicImage;
use log::error;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{collections::HashSet, time::Duration};

use crate::utils::file_watcher::FileWatcher;
use crate::utils::texture_list::TextureList;
use crate::TigerApp;

#[derive(Clone, Default)]
pub struct TextureCache {
    content: CacheHandle,
}

pub type CacheHandle = Arc<Mutex<HashMap<PathBuf, DynamicImage>>>;

impl TextureCache {
    pub fn handle(&self) -> CacheHandle {
        self.content.clone()
    }

    pub fn init<A: TigerApp + Send + Clone + 'static>(&self, tauri_app: A, period: Duration) {
        let (mut file_watcher, events_receiver) = FileWatcher::new({
            let tauri_app = tauri_app.clone();
            move || {
                let app_state = tauri_app.app_state();
                let app = app_state.0.lock();
                app.list_textures()
            }
        });

        std::thread::spawn(move || loop {
            file_watcher.update_watched_files();
            std::thread::sleep(period);
        });

        let cache_handle = self.handle();
        std::thread::spawn(move || loop {
            if let Ok(Ok(events)) = events_receiver.recv() {
                remove(
                    &events.iter().map(|e| e.path.to_owned()).collect(),
                    cache_handle.clone(),
                )
            }
        });

        let cache_handle = self.handle();
        std::thread::spawn(move || loop {
            let current_entries: HashSet<PathBuf> = {
                let cache = cache_handle.clone();
                let cache = cache.lock();
                cache.keys().cloned().collect()
            };
            let desired_entries = {
                let app_state = tauri_app.app_state();
                let app = app_state.0.lock();
                app.list_textures()
            };
            let missing_entries = desired_entries
                .iter()
                .filter(|p| !current_entries.contains(*p))
                .collect::<HashSet<_>>();
            let extraneous_entries = current_entries
                .iter()
                .filter(|p| !desired_entries.contains(*p))
                .collect::<HashSet<_>>();
            remove(&extraneous_entries, cache_handle.clone());
            add(&missing_entries, cache_handle.clone());
            std::thread::sleep(period);
        });
    }
}

fn remove<P: AsRef<Path>>(textures: &HashSet<P>, cache_handle: CacheHandle) {
    let mut cache = cache_handle.lock();
    for path in textures {
        cache.remove(path.as_ref());
    }
}

fn add<P: AsRef<Path>>(textures: &HashSet<P>, cache_handle: CacheHandle) {
    {
        let mut cache = cache_handle.lock();
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
        let mut cache = cache_handle.lock();
        for (path, texture) in new_textures {
            cache.insert(path.to_owned(), texture);
        }
    }
}

#[cfg(test)]
mod test {

    use crate::{
        document::{Command, Document},
        mock::TigerAppMock,
        TigerApp,
    };

    #[test]
    fn adds_and_removes_textures() {
        let dir = std::env::current_dir().unwrap();
        let frame_path = dir.join("test-data/samurai-dead-all.png");

        let app = TigerAppMock::new();
        {
            let app_state = app.app_state();
            let mut app = app_state.0.lock();
            app.open_document(Document::open("test-data/samurai.tiger").unwrap());
        }

        app.wait_for_periodic_scans();
        assert!(app
            .texture_cache()
            .handle()
            .lock()
            .contains_key(&frame_path));

        {
            let app_state = app.app_state();
            let mut app = app_state.0.lock();
            app.current_document_mut()
                .unwrap()
                .process_command(Command::DeleteFrame(frame_path.clone()))
                .unwrap();
        }

        app.wait_for_periodic_scans();
        assert!(!app
            .texture_cache()
            .handle()
            .lock()
            .contains_key(&frame_path));
    }
}
