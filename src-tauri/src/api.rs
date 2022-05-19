use euclid::vec2;
use json_patch::Patch;
use std::path::PathBuf;

use crate::dto;
use crate::state::{App, AppState, Command, Document, DocumentError, SingleSelection};

impl AppState {
    pub fn mutate<F>(&self, operation: F) -> Patch
    where
        F: FnOnce(&mut App),
    {
        let mut app = self.0.lock().unwrap();

        let old_state: dto::App = (&*app).into();
        operation(&mut *app);
        let new_state: dto::App = (&*app).into();

        let old_json = serde_json::to_value(old_state);
        let new_json = serde_json::to_value(new_state);

        match (old_json, new_json) {
            (Ok(o), Ok(n)) => json_patch::diff(&o, &n),
            _ => {
                println!("Patch serialization error");
                Patch(Vec::new())
            }
        }
    }
}

#[tauri::command]
pub fn new_document(app_state: tauri::State<'_, AppState>, path: PathBuf) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        app.new_document(path);
    }))
}

#[tauri::command]
pub async fn open_documents(
    app_state: tauri::State<'_, AppState>,
    paths: Vec<PathBuf>,
) -> Result<Patch, ()> {
    let mut documents: Vec<(PathBuf, Result<Document, DocumentError>)> = Vec::new();
    for path in &paths {
        let open_path = path.to_owned();
        documents.push((
            open_path.clone(),
            tauri::async_runtime::spawn_blocking(move || Document::open(&open_path))
                .await
                .unwrap(),
        ));
    }

    Ok(app_state.mutate(|app| {
        for document in documents {
            match document {
                (_, Ok(d)) => {
                    app.open_document(d);
                }
                (path, Err(e)) => {
                    app.show_error_message(format!(
                        "Could not open `{}`: {e}",
                        path.to_string_lossy()
                    ));
                }
            }
        }
    }))
}

#[tauri::command]
pub fn focus_document(app_state: tauri::State<'_, AppState>, path: PathBuf) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        app.focus_document(&path).ok();
    }))
}

#[tauri::command]
pub fn close_document(app_state: tauri::State<'_, AppState>, path: PathBuf) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        // TODO save on close flow
        app.close_document(&path);
    }))
}

#[tauri::command]
pub async fn save_current_document(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    let (sheet, destination, version) = {
        let app = app_state.0.lock().unwrap();
        match app.current_document() {
            Some(d) => (d.sheet().clone(), d.path().to_owned(), d.version()),
            _ => return Ok(Patch(Vec::new())),
        }
    };

    let write_destination = destination.clone();
    let result = tauri::async_runtime::spawn_blocking(move || sheet.write(&write_destination))
        .await
        .unwrap();

    Ok(app_state.mutate(|app| match result {
        Ok(_) => {
            if let Some(document) = app.document_mut(&destination) {
                document.mark_as_saved(version);
            }
        }
        Err(e) => app.show_error_message(format!(
            "Could not save `{}`: {e}",
            destination.to_string_lossy()
        )),
    }))
}

#[tauri::command]
pub fn focus_content_tab(
    app_state: tauri::State<'_, AppState>,
    content_tab: dto::ContentTab,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::FocusContentTab(content_tab.into()))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn clear_selection(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::ClearSelection).ok();
        }
    }))
}

#[tauri::command]
pub fn select_frame(
    app_state: tauri::State<'_, AppState>,
    path: PathBuf,
    shift: bool,
    ctrl: bool,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::AlterSelection(
                    SingleSelection::Frame(path),
                    shift,
                    ctrl,
                ))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn select_animation(
    app_state: tauri::State<'_, AppState>,
    name: String,
    shift: bool,
    ctrl: bool,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::AlterSelection(
                    SingleSelection::Animation(name),
                    shift,
                    ctrl,
                ))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn pan(app_state: tauri::State<'_, AppState>, delta: (i32, i32)) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::Pan(vec2(delta.0 as f32, delta.1 as f32)))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn edit_animation(app_state: tauri::State<'_, AppState>, name: String) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::EditAnimation(name)).ok();
        }
    }))
}

#[tauri::command]
pub fn rename_animation(
    app_state: tauri::State<'_, AppState>,
    old_name: String,
    new_name: String,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::RenameAnimation(old_name, new_name))
                .ok();
        }
    }))
}
