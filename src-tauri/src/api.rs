use euclid::vec2;
use std::path::PathBuf;

use crate::dto;
use crate::state::{AppState, Command, Document, SingleSelection};

#[tauri::command]
pub async fn open_documents(
    app_state: tauri::State<'_, AppState>,
    paths: Vec<PathBuf>,
) -> Result<dto::App, ()> {
    for path in &paths {
        let open_path = path.to_owned();
        match tauri::async_runtime::spawn_blocking(move || Document::open(&open_path))
            .await
            .unwrap()
        {
            Ok(d) => {
                let mut app = app_state.0.lock().unwrap();
                app.open_document(d);
            }
            Err(e) => {
                let mut app = app_state.0.lock().unwrap();
                app.show_error_message(format!("Could not open `{}`: {e}", path.to_string_lossy()));
            }
        }
    }
    let app = app_state.0.lock().unwrap();
    Ok((&*app).into())
}

#[tauri::command]
pub fn focus_document(
    app_state: tauri::State<'_, AppState>,
    path: PathBuf,
) -> Result<dto::App, ()> {
    let mut app = app_state.0.lock().unwrap();
    app.focus_document(&path).ok();
    Ok((&*app).into())
}

#[tauri::command]
pub fn close_document(
    app_state: tauri::State<'_, AppState>,
    path: PathBuf,
) -> Result<dto::App, ()> {
    let mut app = app_state.0.lock().unwrap();
    // TODO save on close flow
    app.close_document(&path);
    Ok((&*app).into())
}

#[tauri::command]
pub async fn save_current_document(app_state: tauri::State<'_, AppState>) -> Result<dto::App, ()> {
    let (sheet, destination, version) = {
        let app = app_state.0.lock().unwrap();
        match app.current_document() {
            Some(d) => (d.sheet().clone(), d.path().to_owned(), d.version()),
            _ => return Ok((&*app).into()),
        }
    };

    let write_destination = destination.clone();
    let result = tauri::async_runtime::spawn_blocking(move || sheet.write(&write_destination))
        .await
        .unwrap();

    let mut app = app_state.0.lock().unwrap();
    match result {
        Ok(_) => {
            if let Some(document) = app.document_mut(&destination) {
                document.mark_as_saved(version);
            }
        }
        Err(e) => app.show_error_message(format!(
            "Could not save `{}`: {e}",
            destination.to_string_lossy()
        )),
    }

    Ok((&*app).into())
}

#[tauri::command]
pub fn focus_content_tab(
    app_state: tauri::State<'_, AppState>,
    content_tab: dto::ContentTab,
) -> Result<dto::App, ()> {
    let mut app = app_state.0.lock().unwrap();
    if let Some(document) = app.current_document_mut() {
        document.process_command(Command::FocusContentTab(content_tab.into()));
    }
    Ok((&*app).into())
}

#[tauri::command]
pub fn select_frame(
    app_state: tauri::State<'_, AppState>,
    path: PathBuf,
    shift: bool,
    ctrl: bool,
) -> Result<dto::App, ()> {
    let mut app = app_state.0.lock().unwrap();
    if let Some(document) = app.current_document_mut() {
        document.process_command(Command::AlterSelection(
            SingleSelection::Frame(path),
            shift,
            ctrl,
        ));
    }
    Ok((&*app).into())
}

#[tauri::command]
pub fn select_animation(
    app_state: tauri::State<'_, AppState>,
    name: String,
    shift: bool,
    ctrl: bool,
) -> Result<dto::App, ()> {
    let mut app = app_state.0.lock().unwrap();
    if let Some(document) = app.current_document_mut() {
        document.process_command(Command::AlterSelection(
            SingleSelection::Animation(name),
            shift,
            ctrl,
        ));
    }
    Ok((&*app).into())
}

#[tauri::command]
pub fn pan(app_state: tauri::State<'_, AppState>, delta: (i32, i32)) -> Result<dto::App, ()> {
    let mut app = app_state.0.lock().unwrap();
    if let Some(document) = app.current_document_mut() {
        document.process_command(Command::Pan(vec2(delta.0 as f32, delta.1 as f32)));
    }
    Ok((&*app).into())
}
