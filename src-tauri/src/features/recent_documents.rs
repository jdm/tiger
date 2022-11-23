use log::error;
use std::fs::File;
use std::path::PathBuf;
use tauri::Manager;

use crate::app::AppState;
use crate::utils::observable::Observer;
use crate::utils::paths;

pub fn init(tauri_app: &tauri::App) {
    let app_state = tauri_app.state::<AppState>();
    let mut app = app_state.0.lock();

    match read_from_disk() {
        Ok(mut documents) => {
            documents.retain(|d| d.exists());
            app.set_recent_documents(documents);
        }
        Err(e) => error!("Error while reading list of recently opened documents: {e}"),
    };

    let tauri_app_handle = tauri_app.handle();
    std::thread::spawn(move || {
        let app_state = tauri_app_handle.state::<AppState>();
        let app = app_state.0.lock();
        app.recent_documents_delegate()
            .subscribe(|recent_documents| {
                if let Err(e) = write_to_disk(recent_documents) {
                    error!("Error while saving list of recently opened documents: {e}");
                }
                Observer::StaySubscribed
            });
    });
}

fn write_to_disk(documents: &Vec<PathBuf>) -> Result<(), std::io::Error> {
    let path = paths::recent_documents_file();
    let file = File::create(path)?;
    serde_json::to_writer_pretty(file, documents)?;
    Ok(())
}

fn read_from_disk() -> Result<Vec<PathBuf>, std::io::Error> {
    let path = paths::recent_documents_file();
    if !path.exists() {
        return Ok(vec![]);
    }
    let file = File::open(path)?;
    let recent_files: Vec<PathBuf> = serde_json::from_reader(file)?;
    Ok(recent_files)
}
