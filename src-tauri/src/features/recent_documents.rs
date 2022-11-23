use std::fs::{self, File};
use std::path::PathBuf;
use tauri::Manager;

use crate::app::AppState;
use crate::utils::observable::Observer;

pub fn init(tauri_app: &tauri::App) {
    let app_state = tauri_app.state::<AppState>();
    let mut app = app_state.0.lock();

    if let Some(mut documents) = read_from_disk() {
        documents.retain(|d| d.exists());
        app.set_recent_documents(documents);
    }

    let tauri_app_handle = tauri_app.handle();
    std::thread::spawn(move || {
        let app_state = tauri_app_handle.state::<AppState>();
        let app = app_state.0.lock();
        app.recent_documents_delegate()
            .subscribe(|recent_documents| {
                write_to_disk(recent_documents);
                Observer::StaySubscribed
            });
    });
}

fn write_to_disk(documents: &Vec<PathBuf>) -> Option<()> {
    let path = storage_location()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).ok(); // TODO logging
    }
    let file = File::create(path).ok()?; // TODO logging
    serde_json::to_writer_pretty(file, documents).ok(); // TODO logging
    Some(())
}

fn read_from_disk() -> Option<Vec<PathBuf>> {
    let path = storage_location()?;
    let file = File::open(path).ok()?; // TODO logging
    let recent_files: Vec<PathBuf> = serde_json::from_reader(file).ok()?; // TODO logging
    Some(recent_files)
}

fn storage_location() -> Option<PathBuf> {
    directories::ProjectDirs::from("org", "Permafrost", "Tiger")
        .map(|p| p.data_local_dir().join("recent-files.json"))
}
