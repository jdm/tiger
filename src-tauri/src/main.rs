#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod api;
mod dto;
mod sheet;
mod state;

use state::AppState;

fn main() {
    tauri::Builder::default()
        .manage(AppState(Default::default()))
        .invoke_handler(tauri::generate_handler![
            // App
            api::get_state,
            api::new_document,
            api::open_documents,
            api::focus_document,
            api::close_document,
            api::save_current_document,
            api::focus_content_tab,
            // Document
            api::undo,
            api::redo,
            api::clear_selection,
            api::select_frame,
            api::select_animation,
            api::pan,
            api::zoom_in_workbench,
            api::zoom_out_workbench,
            api::reset_workbench_zoom,
            api::edit_animation,
            api::rename_animation,
            api::delete_animation,
            api::tick,
            api::play,
            api::pause,
            api::scrub_timeline,
            api::zoom_in_timeline,
            api::zoom_out_timeline,
            api::reset_timeline_zoom,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
