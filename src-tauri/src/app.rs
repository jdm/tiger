use log::error;
use serde::Serialize;
use tauri::{ClipboardManager, Manager};

use crate::{
    dto::StateTrim,
    features::texture_cache,
    state::{self, State},
    utils::paths,
};

static EVENT_PATCH_STATE: &str = "patch-state";
static EVENT_REPLACE_STATE: &str = "replace-state";

pub trait TigerApp {
    fn state(&self) -> state::Handle;
    fn texture_cache(&self) -> texture_cache::Handle;
    fn paths(&self) -> paths::Handle;
    fn patch_state<F: FnOnce(&mut State)>(&self, state_trim: StateTrim, operation: F);
    fn replace_state(&self);
    fn emit_all<S: Serialize + Clone>(&self, event: &str, payload: S);
    fn read_clipboard(&self) -> Option<String>;
    fn write_clipboard<S: Into<String>>(&self, content: S);
    fn close_window(&self);
}

impl TigerApp for tauri::App {
    fn state(&self) -> state::Handle {
        TigerApp::state(&self.handle())
    }

    fn texture_cache(&self) -> texture_cache::Handle {
        self.handle().texture_cache()
    }

    fn paths(&self) -> paths::Handle {
        self.handle().paths()
    }

    fn patch_state<F: FnOnce(&mut State)>(&self, state_trim: StateTrim, operation: F) {
        TigerApp::patch_state(&self.handle(), state_trim, operation)
    }

    fn replace_state(&self) {
        TigerApp::replace_state(&self.handle())
    }

    fn emit_all<S: Serialize + Clone>(&self, event: &str, payload: S) {
        TigerApp::emit_all(&self.handle(), event, payload)
    }

    fn read_clipboard(&self) -> Option<String> {
        self.handle().read_clipboard()
    }

    fn write_clipboard<S: Into<String>>(&self, content: S) {
        self.handle().write_clipboard(content)
    }

    fn close_window(&self) {
        self.handle().close_window()
    }
}

impl TigerApp for tauri::AppHandle {
    fn state(&self) -> state::Handle {
        let state = tauri::Manager::state::<state::Handle>(self);
        state::Handle::clone(&state)
    }

    fn texture_cache(&self) -> texture_cache::Handle {
        let cache = tauri::Manager::state::<texture_cache::Handle>(self);
        texture_cache::Handle::clone(&cache)
    }

    fn paths(&self) -> paths::Handle {
        let paths = tauri::Manager::state::<paths::Handle>(self);
        paths::Handle::clone(&paths)
    }

    fn patch_state<F>(&self, state_trim: StateTrim, operation: F)
    where
        F: FnOnce(&mut State),
    {
        let state_handle = tauri::Manager::state::<state::Handle>(self);
        let patch = state_handle.mutate(state_trim, operation);
        if !patch.0.is_empty() {
            if let Err(e) = tauri::Manager::emit_all(self, EVENT_PATCH_STATE, patch) {
                error!("Error while pushing state patch: {e}");
            }
        }
    }

    fn replace_state(&self) {
        let state_handle = tauri::Manager::state::<state::Handle>(self);
        let state = state_handle.lock();
        let new_state = state.to_dto(StateTrim::Full);
        if let Err(e) = tauri::Manager::emit_all(self, EVENT_REPLACE_STATE, new_state) {
            error!("Error while replacing state: {e}");
        }
    }

    fn emit_all<S: Serialize + Clone>(&self, event: &str, payload: S) {
        tauri::Manager::emit_all(self, event, payload).ok();
    }

    fn read_clipboard(&self) -> Option<String> {
        match self.clipboard_manager().read_text() {
            Ok(t) => t,
            Err(e) => {
                error!("Failed to read clipboard content: `{e}`");
                None
            }
        }
    }

    fn write_clipboard<S: Into<String>>(&self, content: S) {
        if let Err(e) = self.clipboard_manager().write_text(content.into()) {
            error!("Failed to write clipboard content: `{e}`");
        }
    }

    fn close_window(&self) {
        if let Some(window) = self.get_window("main") {
            window.close().ok();
        } else {
            error!("Could not access app window to close it");
        }
    }
}
