use std::time::Duration;
use tauri::Manager;

use crate::api::Stateful;
use crate::app::AppState;
use crate::dto::AppTrim;
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
        if let Ok(Ok(_)) = events_receiver.recv() {
            tauri_app_handle.patch_state(AppTrim::Full, |app| {
                for document in app.documents_iter_mut() {
                    let missing_textures = document
                        .list_textures()
                        .into_iter()
                        .filter(|t| !t.exists())
                        .collect();
                    document.set_missing_textures(missing_textures);
                }
            });
        }
    });
}
