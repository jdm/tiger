use std::path::PathBuf;

use crate::dto;
use crate::state::AppState;

#[tauri::command]
pub async fn open_documents(
    app_state: tauri::State<'_, AppState>,
    paths: Vec<PathBuf>,
) -> Result<dto::App, ()> {
    let mut app = app_state.0.lock().unwrap();
    for path in &paths {
        if let Err(e) = app.open_document(path) {
            app.show_error_message(format!("Could not open `{}`: {e}", path.to_string_lossy()));
        }
    }
    Ok((&*app).into())
}

#[tauri::command]
pub async fn focus_document(
    app_state: tauri::State<'_, AppState>,
    path: PathBuf,
) -> Result<dto::App, ()> {
    let mut app = app_state.0.lock().unwrap();
    app.focus_document(&path).ok();
    Ok((&*app).into())
}

#[tauri::command]
pub async fn close_document(
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

    let result = sheet.write(&destination);

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
