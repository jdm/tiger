use image::DynamicImage;
use log::error;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::{collections::HashSet, time::Duration};
use tauri::Manager;

use crate::app::AppState;
use crate::utils::file_watcher::FileWatcher;

#[derive(Clone, Default)]
pub struct TextureCache {
    content: CacheHandle,
}

pub type CacheHandle = Arc<Mutex<HashMap<PathBuf, DynamicImage>>>;

impl TextureCache {
    pub fn handle(&self) -> CacheHandle {
        self.content.clone()
    }

    pub fn init(&self, tauri_app: &tauri::App) {
        let tauri_app_handle = tauri_app.handle();

        let (mut file_watcher, events_receiver) =
            FileWatcher::new(move || list_textures(&tauri_app_handle));

        std::thread::spawn(move || loop {
            file_watcher.update_watched_files();
            std::thread::sleep(Duration::from_millis(1_000));
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

        let tauri_app_handle = tauri_app.handle();
        let cache_handle = self.handle();
        std::thread::spawn(move || loop {
            let current_entries: HashSet<PathBuf> = {
                let cache = cache_handle.clone();
                let cache = cache.lock();
                cache.keys().cloned().collect()
            };
            let desired_entries = list_textures(&tauri_app_handle);
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
            std::thread::sleep(Duration::from_millis(1_000));
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

fn list_textures(tauri_app_handle: &tauri::AppHandle) -> HashSet<PathBuf> {
    let app_state = tauri_app_handle.state::<AppState>();
    let app = app_state.0.lock();
    app.documents_iter()
        .flat_map(|d| d.sheet().frames_iter())
        .map(|f| f.source().to_owned())
        .collect::<HashSet<_>>()
}
