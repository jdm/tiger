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
    let mut app = app_state.0.lock().unwrap();
    app.save_current_document()?;
    Ok((&*app).into())
}
