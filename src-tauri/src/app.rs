use json_patch::Patch;
use serde::Serialize;

use crate::{
    dto::StateTrim,
    features::texture_cache,
    state::{self, State},
    utils::paths,
};

#[cfg(test)]
pub mod mock;
pub mod tauri;

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
    fn patch<F: FnOnce(&mut State)>(&self, state_trim: StateTrim, operation: F) -> Patch {
        let state_handle = self.state();
        let mut state = state_handle.lock();
        state.patch(state_trim, operation)
    }
}
