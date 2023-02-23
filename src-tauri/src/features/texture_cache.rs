use image::DynamicImage;
use log::error;
use parking_lot::RwLock;
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
    sync::Arc,
    thread,
    time::Duration,
};

use crate::{
    app::TigerApp,
    utils::{file_watcher::FileWatcher, handle, texture_list::TextureList},
};

#[cfg(not(test))]
static PERIOD: Duration = Duration::from_millis(1_000);
#[cfg(test)]
static PERIOD: Duration = Duration::from_millis(100);

pub type Handle = handle::Handle<HashMap<PathBuf, DynamicImage>>;

#[derive(Clone)]
pub struct TextureCacheInfo {
    #[cfg(test)]
    file_watcher: Arc<RwLock<FileWatcher>>,
}

pub fn init<A: TigerApp + Send + Sync + Clone + 'static>(app: A) -> TextureCacheInfo {
    let (file_watcher, events_receiver) = FileWatcher::new({
        let app = app.clone();
        move || {
            let state_handle = app.state();
            let state = state_handle.lock();
            state.list_textures()
        }
    });
    let file_watcher = Arc::new(RwLock::new(file_watcher));

    thread::Builder::new()
        .name("texture-cache-update-watcher-thread".to_owned())
        .spawn({
            #[cfg(test)]
            let file_watcher = file_watcher.clone();
            move || loop {
                file_watcher.write().update_watched_files();
                thread::sleep(PERIOD);
            }
        })
        .unwrap();

    let texture_cache = app.texture_cache();

    thread::Builder::new()
        .name("texture-cache-eviction-thread".to_owned())
        .spawn({
            let texture_cache = texture_cache.clone();
            move || loop {
                if let Ok(Ok(events)) = events_receiver.recv() {
                    remove(
                        &events.iter().map(|e| e.path.to_owned()).collect(),
                        &texture_cache,
                    )
                }
            }
        })
        .unwrap();

    thread::Builder::new()
        .name("texture-cache-update-thread".to_owned())
        .spawn(move || loop {
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
            thread::sleep(PERIOD);
        })
        .unwrap();

    TextureCacheInfo {
        #[cfg(test)]
        file_watcher,
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

    use retry::{delay::Fixed, retry};
    use std::path::PathBuf;
    use sugar_path::SugarPath;

    use super::*;
    use crate::app::{mock::TigerAppMock, TigerApp};

    #[tokio::test]
    async fn follows_frame_additions_and_removals() {
        let frame_path = PathBuf::from("test-data/samurai/dead-all.png").resolve();
        let app = TigerAppMock::new();

        let is_cached = |app: &TigerAppMock| app.texture_cache().lock().contains_key(&frame_path);

        let check_cached = |app: &TigerAppMock| {
            retry(Fixed::from(PERIOD).take(100), || {
                is_cached(app).then_some(()).ok_or(())
            })
        };

        let check_evicted = |app: &TigerAppMock| {
            retry(Fixed::from(PERIOD).take(100), || {
                (!is_cached(app)).then_some(()).ok_or(())
            })
        };

        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        assert!(check_cached(&app).is_ok());
        app.delete_frame(frame_path.clone());
        assert!(check_evicted(&app).is_ok());
    }

    #[tokio::test]
    async fn detects_texture_changes() {
        let frame = PathBuf::from("test-output/detects_texture_changes.png").resolve();
        let before_frame = PathBuf::from("test-data/samurai/dead-all.png").resolve();
        let after_frame = PathBuf::from("test-data/samurai/attack-north.png").resolve();

        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        app.import_frames(vec![frame.clone()]);

        std::fs::copy(&before_frame, &frame).unwrap();

        let validate_cached_image = |cache_key: &PathBuf, reference_image: &PathBuf| {
            retry(Fixed::from(PERIOD).take(100), || {
                match app.texture_cache().lock().get(cache_key) {
                    None => Err("Not in cache"),
                    Some(image) => {
                        let Ok(reference_image) = image::open(reference_image) else {
                            return Err("Could not open reference_image");
                        };
                        if image == &reference_image {
                            Ok(())
                        } else {
                            Err("Image mismatch")
                        }
                    }
                }
            })
        };

        let has_old_version = validate_cached_image(&frame, &before_frame);
        assert_eq!(has_old_version, Ok(()));

        let is_watching = retry(Fixed::from(PERIOD).take(100), || {
            app.texture_cache_info()
                .file_watcher
                .read()
                .is_watching(&frame)
                .then_some(())
                .ok_or(())
        });
        assert!(is_watching.is_ok());

        std::fs::copy(&after_frame, &frame).unwrap();
        let has_new_version = validate_cached_image(&frame, &after_frame);
        assert_eq!(has_new_version, Ok(()));
    }
}
