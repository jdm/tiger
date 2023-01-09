#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use log::{error, LevelFilter};
use serde::Serialize;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
use std::time::Duration;
use tauri::{ClipboardManager, Manager};

use dto::StateTrim;
use features::texture_cache;
use state::State;
use utils::paths;

mod api;
mod document;
mod dto;
mod export;
mod features;
#[cfg(test)]
mod mock;
mod sheet;
mod state;
mod utils;

static EVENT_PATCH_STATE: &str = "patch-state";
static EVENT_REPLACE_STATE: &str = "replace-state";

fn main() {
    let paths = paths::Paths::default();

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            std::fs::File::create(&paths.log_file).unwrap(),
        ),
    ])
    .unwrap();

    tauri::Builder::default()
        .manage(state::Handle::default())
        .manage(texture_cache::Handle::default())
        .manage(paths::Handle::new(paths))
        .setup(|tauri_app| {
            init_window_shadow(tauri_app);
            tauri_app
                .texture_cache()
                .init(tauri_app.handle(), Duration::from_millis(1_000));
            features::missing_textures::init(tauri_app.handle(), Duration::from_millis(500));
            features::recent_documents::init(tauri_app.handle());
            features::template_hot_reload::init(tauri_app.handle(), Duration::from_millis(1_000));
            features::texture_hot_reload::init(tauri_app.handle(), Duration::from_millis(1_000));
            features::clipboard_analysis::init(tauri_app.handle(), Duration::from_millis(100));
            Ok(())
        })
        .on_window_event(handle_window_event)
        .invoke_handler(tauri::generate_handler![
            // App
            api::acknowledge_error,
            api::cancel_exit,
            api::close_all_documents,
            api::close_current_document,
            api::close_document,
            api::focus_document,
            api::get_state,
            api::new_document,
            api::open_documents,
            api::request_exit,
            api::reveal_in_explorer,
            api::save_all,
            api::show_error_message,
            // Document
            api::apply_direction_preset,
            api::begin_drag_and_drop_frame,
            api::begin_drag_and_drop_keyframe,
            api::begin_drag_keyframe_duration,
            api::begin_export_as,
            api::begin_nudge_hitbox,
            api::begin_nudge_keyframe,
            api::begin_rename_animation,
            api::begin_rename_hitbox,
            api::begin_rename_selection,
            api::begin_resize_hitbox,
            api::browse_selection,
            api::browse_to_end,
            api::browse_to_start,
            api::cancel_export_as,
            api::cancel_rename,
            api::center_workbench,
            api::clear_selection,
            api::close_without_saving,
            api::copy,
            api::create_animation,
            api::create_hitbox,
            api::cut,
            api::delete_animation,
            api::delete_frame,
            api::delete_hitbox,
            api::delete_selected_animations,
            api::delete_selected_frames,
            api::delete_selected_hitboxes,
            api::delete_selected_keyframes,
            api::delete_selection,
            api::disable_sprite_darkening,
            api::drop_frame_on_timeline,
            api::drop_keyframe_on_timeline,
            api::edit_animation,
            api::enable_sprite_darkening,
            api::end_drag_and_drop_frame,
            api::end_drag_and_drop_keyframe,
            api::end_drag_keyframe_duration,
            api::end_export_as,
            api::end_nudge_hitbox,
            api::end_nudge_keyframe,
            api::end_rename_animation,
            api::end_rename_hitbox,
            api::end_resize_hitbox,
            api::export,
            api::filter_animations,
            api::filter_frames,
            api::hide_hitboxes,
            api::hide_origin,
            api::hide_sprite,
            api::import_frames,
            api::jump_to_animation_end,
            api::jump_to_animation_start,
            api::jump_to_next_frame,
            api::jump_to_previous_frame,
            api::lock_hitboxes,
            api::nudge_selection,
            api::pan_timeline,
            api::pan,
            api::paste,
            api::pause,
            api::play,
            api::redo,
            api::begin_relocate_frames,
            api::relocate_frame,
            api::end_relocate_frames,
            api::cancel_relocate_frames,
            api::reset_timeline_zoom,
            api::reset_workbench_zoom,
            api::save_as,
            api::save,
            api::scrub_timeline,
            api::select_all,
            api::select_animation,
            api::select_direction,
            api::select_frame,
            api::select_hitbox,
            api::select_keyframe,
            api::set_animation_looping,
            api::set_animations_list_offset,
            api::set_export_metadata_file,
            api::set_export_metadata_paths_root,
            api::set_export_template_file,
            api::set_export_texture_file,
            api::set_frames_list_mode,
            api::set_frames_list_offset,
            api::set_hitbox_height,
            api::set_hitbox_position_x,
            api::set_hitbox_position_y,
            api::set_hitbox_width,
            api::set_hitboxes_list_offset,
            api::set_keyframe_duration,
            api::set_keyframe_offset_x,
            api::set_keyframe_offset_y,
            api::set_keyframe_snapping_base_duration,
            api::set_snap_keyframe_durations,
            api::set_snap_keyframes_to_multiples_of_duration,
            api::set_snap_keyframes_to_other_keyframes,
            api::set_timeline_offset,
            api::set_timeline_zoom_amount,
            api::set_workbench_zoom_factor,
            api::show_hitboxes,
            api::show_origin,
            api::show_sprite,
            api::tick,
            api::toggle_preserve_aspect_ratio,
            api::undo,
            api::unlock_hitboxes,
            api::update_drag_keyframe_duration,
            api::update_nudge_hitbox,
            api::update_nudge_keyframe,
            api::update_resize_hitbox,
            api::zoom_in_timeline_around,
            api::zoom_in_timeline,
            api::zoom_in_workbench_around,
            api::zoom_in_workbench,
            api::zoom_out_timeline_around,
            api::zoom_out_timeline,
            api::zoom_out_workbench_around,
            api::zoom_out_workbench,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn handle_window_event(event: tauri::GlobalWindowEvent) {
    if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
        event
            .window()
            .app_handle()
            .patch_state(dto::StateTrim::Full, |state| {
                state.request_exit();
                if !state.should_exit() {
                    api.prevent_close();
                }
            });
    }
}

fn init_window_shadow(tauri_app: &mut tauri::App) {
    let Some(window) = tauri_app.get_window("main") else {
        error!("Could not access app window to initialize shadow");
        return;
    };
    if let Err(e) = window_shadows::set_shadow(&window, true) {
        error!("Failed to initialize window shadows: `{e}`");
    }
}

pub trait TigerApp {
    fn state(&self) -> state::Handle;
    fn texture_cache(&self) -> texture_cache::Handle;
    fn paths(&self) -> paths::Handle;
    fn patch_state<F: FnOnce(&mut State)>(&self, state_trim: StateTrim, operation: F);
    fn replace_state(&self);
    fn emit_all<S: Serialize + Clone>(&self, event: &str, payload: S);
    fn read_clipboard(&self) -> Option<String>;
    fn write_clipboard<S: Into<String>>(&self, content: S);
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
        let new_state = state.to_dto(dto::StateTrim::Full);
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
}
