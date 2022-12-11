#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use log::{error, LevelFilter};
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
use tauri::Manager;

use api::Stateful;

mod api;
mod app;
mod document;
mod dto;
mod export;
mod features;
mod sheet;
mod utils;

fn main() {
    utils::paths::init();

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
            std::fs::File::create(utils::paths::log_file()).unwrap(),
        ),
    ])
    .unwrap();

    tauri::Builder::default()
        .manage(app::AppState(Default::default()))
        .manage(features::texture_cache::TextureCache::default())
        .setup(|tauri_app| {
            init_window_shadow(tauri_app);
            tauri_app
                .state::<features::texture_cache::TextureCache>()
                .init(tauri_app);
            features::recent_documents::init(tauri_app);
            features::template_hot_reload::init(tauri_app);
            features::texture_hot_reload::init(tauri_app);
            features::clipboard_analysis::init(tauri_app);
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
            api::pan,
            api::paste,
            api::pause,
            api::play,
            api::redo,
            api::reset_timeline_zoom,
            api::reset_workbench_zoom,
            api::save_as,
            api::save,
            api::scrub_timeline,
            api::select_animation,
            api::select_direction,
            api::select_frame,
            api::select_hitbox,
            api::select_keyframe,
            api::set_animation_looping,
            api::set_export_metadata_file,
            api::set_export_metadata_paths_root,
            api::set_export_template_file,
            api::set_export_texture_file,
            api::set_frames_list_mode,
            api::set_hitbox_height,
            api::set_hitbox_position_x,
            api::set_hitbox_position_y,
            api::set_hitbox_width,
            api::set_keyframe_duration,
            api::set_keyframe_offset_x,
            api::set_keyframe_offset_y,
            api::set_keyframe_snapping_base_duration,
            api::set_snap_keyframe_durations,
            api::set_snap_keyframes_to_multiples_of_duration,
            api::set_snap_keyframes_to_other_keyframes,
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
            api::zoom_in_timeline,
            api::zoom_in_workbench,
            api::zoom_in_workbench_around,
            api::zoom_out_timeline,
            api::zoom_out_workbench,
            api::zoom_out_workbench_around,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn handle_window_event(event: tauri::GlobalWindowEvent) {
    if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
        event
            .window()
            .app_handle()
            .patch_state(dto::AppTrim::Full, |app| {
                app.request_exit();
                if !app.should_exit() {
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
