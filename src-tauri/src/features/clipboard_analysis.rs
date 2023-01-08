use log::error;
use std::time::Duration;
use tauri::ClipboardManager;

use crate::document::clipboard_manifest;
use crate::dto::AppTrim;
use crate::TigerApp;

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

    {
        let state_handle = app_handle.state();
        let state = state_handle.0.lock();
        if *state.clipboard_manifest() == new_manifest {
            return;
        }
    }

    app_handle.patch_state(AppTrim::NoDocuments, |state| {
        state.set_clipboard_manifest(new_manifest);
    });
}
