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
            api::select_keyframe,
            api::select_hitbox,
            api::pan,
            api::center_workbench,
            api::zoom_in_workbench,
            api::zoom_out_workbench,
            api::reset_workbench_zoom,
            api::create_animation,
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
            api::set_animation_looping,
            api::apply_direction_preset,
            api::select_direction,
            api::begin_drag_and_drop_frame,
            api::drop_frame_on_timeline,
            api::end_drag_and_drop_frame,
            api::begin_drag_and_drop_keyframe,
            api::drop_keyframe_on_timeline,
            api::end_drag_and_drop_keyframe,
            api::begin_drag_keyframe_duration,
            api::update_drag_keyframe_duration,
            api::end_drag_keyframe_duration,
            api::begin_nudge_keyframe,
            api::update_nudge_keyframe,
            api::end_nudge_keyframe,
            api::create_hitbox,
            api::rename_hitbox,
            api::delete_hitbox,
            api::begin_nudge_hitbox,
            api::update_nudge_hitbox,
            api::end_nudge_hitbox,
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
