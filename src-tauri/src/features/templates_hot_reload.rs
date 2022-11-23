use std::path::PathBuf;
use std::{collections::HashSet, time::Duration};
use tauri::Manager;

use crate::app::AppState;
use crate::sheet;
use crate::utils::file_watcher::FileWatcher;

pub fn init(tauri_app: &tauri::App) {
    let tauri_app_handle = tauri_app.handle();
    let (mut file_watcher, events_receiver) = FileWatcher::new(move || {
        let app_state = tauri_app_handle.state::<AppState>();
        let app = app_state.0.lock();
        app.documents_iter()
            .flat_map(|d| d.export_settings_edit())
            .map(|s| match s {
                sheet::ExportSettings::Liquid(l) => l.template_file().to_owned(),
            })
            .collect::<HashSet<_>>()
    });

    std::thread::spawn(move || loop {
        file_watcher.update_watched_files();
        std::thread::sleep(Duration::from_millis(1_000));
    });

    let tauri_app_handle = tauri_app.handle();
    std::thread::spawn(move || loop {
        if let Ok(Ok(events)) = events_receiver.recv() {
            #[derive(Clone, serde::Serialize)]
            struct TemplateEvent {
                path: PathBuf,
            }
            for event in events {
                tauri_app_handle
                    .emit_all("invalidate-template", TemplateEvent { path: event.path })
                    .unwrap();
            }
        }
    });
}
