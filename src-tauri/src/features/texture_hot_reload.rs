use std::path::PathBuf;
use std::time::Duration;
use tauri::Manager;

use crate::app::AppState;
use crate::utils::file_watcher::FileWatcher;
use crate::utils::texture_list::TextureList;

pub fn init(tauri_app: &tauri::App) {
    let tauri_app_handle = tauri_app.handle();
    let (mut file_watcher, events_receiver) = FileWatcher::new(move || {
        let app_state = tauri_app_handle.state::<AppState>();
        let app = app_state.0.lock();
        app.list_textures()
    });

    std::thread::spawn(move || loop {
        file_watcher.update_watched_files();
        std::thread::sleep(Duration::from_millis(1_000));
    });

    let tauri_app_handle = tauri_app.handle();
    std::thread::spawn(move || loop {
        #[derive(Clone, serde::Serialize)]
        struct TextureEvent {
            path: PathBuf,
        }
        if let Ok(Ok(events)) = events_receiver.recv() {
            for event in events {
                tauri_app_handle
                    .emit_all("invalidate-texture", TextureEvent { path: event.path })
                    .ok();
            }
        }
    });
}
