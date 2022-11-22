#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{collections::HashSet, path::PathBuf, time::Duration};
use tauri::{Manager, WindowEvent};

mod api;
mod app;
mod document;
mod dto;
mod export;
mod file_watcher;
mod sheet;

use app::AppState;
use file_watcher::FileWatcher;

fn main() {
    let app_state = AppState(Default::default());

    let texture_watcher_app_state = app_state.clone();
    let (mut texture_watcher, texture_events_receiver) = FileWatcher::new(move || {
        let app = texture_watcher_app_state.0.lock().unwrap();
        app.documents_iter()
            .flat_map(|d| d.sheet().frames_iter())
            .map(|f| f.source().to_owned())
            .collect::<HashSet<_>>()
    });

    let template_watcher_app_state = app_state.clone();
    let (mut template_watcher, template_events_receiver) = FileWatcher::new(move || {
        let app = template_watcher_app_state.0.lock().unwrap();
        app.documents_iter()
            .flat_map(|d| d.export_settings_edit())
            .map(|s| match s {
                sheet::ExportSettings::Liquid(l) => l.template_file().to_owned(),
            })
            .collect::<HashSet<_>>()
    });

    tauri::Builder::default()
        .manage(app_state.clone())
        .invoke_handler(tauri::generate_handler![
            // App
            api::get_state,
            api::show_error_message,
            api::acknowledge_error,
            api::new_document,
            api::open_documents,
            api::save_all,
            api::focus_document,
            api::close_document,
            api::close_current_document,
            api::close_all_documents,
            api::request_exit,
            api::cancel_exit,
            // Document
            api::close_without_saving,
            api::save,
            api::save_as,
            api::undo,
            api::redo,
            api::cut,
            api::copy,
            api::paste,
            api::set_frames_list_mode,
            api::filter_frames,
            api::filter_animations,
            api::import_frames,
            api::delete_frame,
            api::delete_selected_frames,
            api::delete_selection,
            api::nudge_selection,
            api::browse_selection,
            api::clear_selection,
            api::select_frame,
            api::select_animation,
            api::select_keyframe,
            api::select_hitbox,
            api::pan,
            api::center_workbench,
            api::zoom_in_workbench,
            api::zoom_out_workbench,
            api::set_workbench_zoom_factor,
            api::reset_workbench_zoom,
            api::enable_sprite_darkening,
            api::disable_sprite_darkening,
            api::hide_sprite,
            api::show_sprite,
            api::hide_hitboxes,
            api::show_hitboxes,
            api::hide_origin,
            api::show_origin,
            api::create_animation,
            api::edit_animation,
            api::rename_animation,
            api::delete_animation,
            api::delete_selected_animations,
            api::tick,
            api::play,
            api::pause,
            api::scrub_timeline,
            api::jump_to_animation_start,
            api::jump_to_animation_end,
            api::jump_to_previous_frame,
            api::jump_to_next_frame,
            api::zoom_in_timeline,
            api::zoom_out_timeline,
            api::set_timeline_zoom_amount,
            api::reset_timeline_zoom,
            api::set_animation_looping,
            api::apply_direction_preset,
            api::select_direction,
            api::begin_drag_and_drop_frame,
            api::drop_frame_on_timeline,
            api::end_drag_and_drop_frame,
            api::delete_selected_keyframes,
            api::set_keyframe_duration,
            api::set_keyframe_offset_x,
            api::set_keyframe_offset_y,
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
            api::delete_selected_hitboxes,
            api::lock_hitboxes,
            api::unlock_hitboxes,
            api::set_hitbox_position_x,
            api::set_hitbox_position_y,
            api::set_hitbox_width,
            api::set_hitbox_height,
            api::toggle_preserve_aspect_ratio,
            api::begin_nudge_hitbox,
            api::update_nudge_hitbox,
            api::end_nudge_hitbox,
            api::begin_resize_hitbox,
            api::update_resize_hitbox,
            api::end_resize_hitbox,
            api::export,
            api::begin_export_as,
            api::set_export_template_file,
            api::set_export_texture_file,
            api::set_export_metadata_file,
            api::set_export_metadata_paths_root,
            api::cancel_export_as,
            api::end_export_as,
        ])
        .setup(|tauri_app| {
            // Every 1s, update the list of files we are watching for changes
            std::thread::spawn(move || loop {
                texture_watcher.update_watched_files();
                template_watcher.update_watched_files();
                std::thread::sleep(Duration::from_millis(1_000));
            });

            // Tell the frontend when a texture file is updated
            let tauri_app_handle = tauri_app.handle();
            std::thread::spawn(move || loop {
                #[derive(Clone, serde::Serialize)]
                struct TextureEvent {
                    path: PathBuf,
                }
                if let Ok(Ok(events)) = texture_events_receiver.recv() {
                    for event in events {
                        tauri_app_handle
                            .emit_all("invalidate-texture", TextureEvent { path: event.path })
                            .ok();
                    }
                }
            });

            // Tell the frontend when a template file is updated
            let tauri_app_handle = tauri_app.handle();
            std::thread::spawn(move || loop {
                if let Ok(Ok(events)) = template_events_receiver.recv() {
                    #[derive(Clone, serde::Serialize)]
                    struct TemplateEvent {
                        path: PathBuf,
                    }
                    for event in events {
                        tauri_app_handle
                            .emit_all("invalidate-template", TemplateEvent { path: event.path })
                            .unwrap();
                    }
                }
            });

            Ok(())
        })
        .on_window_event(move |event| {
            if let WindowEvent::CloseRequested { api, .. } = event.event() {
                let mut app = app_state.0.lock().unwrap();
                app.request_exit();
                if !app.should_exit() {
                    api.prevent_close();
                    let new_state: dto::App = (&*app).into();
                    event
                        .window()
                        .app_handle()
                        .emit_all("force-refresh-state", new_state)
                        .unwrap();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
