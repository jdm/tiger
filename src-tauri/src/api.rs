use std::path::PathBuf;

use crate::dto;
use crate::state::AppState;

// TODO commands returning errors can fail after modifiying state
// In such cases, updated state will not be communicated to the frontend.
// Consider inserting the errors inside the state and not returning Result<>
// from these functions.

#[tauri::command]
pub async fn open_documents(
    app_state: tauri::State<'_, AppState>,
    paths: Vec<PathBuf>,
) -> Result<dto::App, String> {
    let mut app = app_state.0.lock().unwrap();
    for path in paths {
        app.open_document(path)?;
    }
    Ok((&*app).into())
}

#[tauri::command]
pub async fn save_current_document(
    app_state: tauri::State<'_, AppState>,
) -> Result<dto::App, String> {
    let (sheet, destination, version) = {
        let app = app_state.0.lock().unwrap();
        match app.current_document() {
            Some(d) => (d.sheet().clone(), d.source().to_owned(), d.version()),
            _ => return Err("".to_owned()),
        }
    };
    sheet.write(destination)?;
    let mut app = app_state.0.lock().unwrap();
    if let Some(document) = app.current_document_mut() {
        document.mark_as_saved(version);
    }
    Ok((&*app).into())
}
