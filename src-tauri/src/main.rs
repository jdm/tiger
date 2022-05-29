#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod api;
mod dto;
mod sheet;
mod state;

use state::AppState;
use tauri::WindowEvent;

fn main() {
    let app_state = AppState(Default::default());
    tauri::Builder::default()
        .manage(app_state.clone())
        .invoke_handler(tauri::generate_handler![
            // App
            api::get_state,
            api::new_document,
            api::open_documents,
            api::focus_document,
            api::close_document,
            api::close_current_document,
            api::close_all_documents,
            api::request_exit,
            api::cancel_exit,
            // Document
            api::close_without_saving,
            api::save,
            api::undo,
            api::redo,
            api::focus_content_tab,
            api::import_frames,
            api::clear_selection,
            api::select_frame,
            api::select_animation,
            api::pan,
            api::center_workbench,
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
        .on_window_event(move |event| match event.event() {
            WindowEvent::CloseRequested { api, .. } => {
                let mut app = app_state.0.lock().unwrap();
                app.request_exit();
                if !app.should_exit() {
                    api.prevent_close();
                    let new_state: dto::App = (&*app).into();
                    event
                        .window()
                        .emit("force-refresh-state", new_state)
                        .unwrap();
                }
            }
            _ => (),
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
