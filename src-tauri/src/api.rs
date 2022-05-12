use std::path::PathBuf;

use crate::dto;
use crate::state::AppState;

#[tauri::command]
pub async fn open_document(
    app_state: tauri::State<'_, AppState>,
    path: PathBuf,
) -> Result<dto::App, String> {
    let mut app = app_state.0.lock().unwrap();
    app.open_document(path)?;
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
