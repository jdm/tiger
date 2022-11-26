use log::error;
use std::time::Duration;
use tauri::{ClipboardManager, Manager};

use crate::app::AppState;
use crate::document::clipboard_manifest;
use crate::dto::AppTrim;
use crate::EVENT_FORCE_PATCH_STATE;

pub fn init(tauri_app: &tauri::App) {
    let tauri_app_handle = tauri_app.handle();
    std::thread::spawn(move || loop {
        std::thread::sleep(Duration::from_millis(100));
        update_clipboard_manifest(&tauri_app_handle);
    });
}

fn update_clipboard_manifest(app_handle: &tauri::AppHandle) {
    let clipboard_manager = app_handle.clipboard_manager();
    let clipboard_content = match clipboard_manager.read_text() {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to read clipboard content: `{e}`");
            return;
        }
    };

    let new_manifest = match clipboard_content {
        None => None,
        Some(s) => clipboard_manifest(s),
    };

    let app_state = app_handle.state::<AppState>();
    let patch = app_state.mutate(AppTrim::NoDocuments, |app| {
        app.set_clipboard_manifest(new_manifest);
    });

    if !patch.0.is_empty() {
        if let Err(e) = app_handle.emit_all(EVENT_FORCE_PATCH_STATE, patch) {
            error!("Failed to push updated clipboard state: `{e}`");
        }
    }
}
