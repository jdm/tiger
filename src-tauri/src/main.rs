#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use log::{error, LevelFilter};
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
use std::time::Duration;
use tauri::Manager;

use app::TigerApp;
use features::texture_cache;
use utils::paths;

mod api;
mod app;
mod document;
mod dto;
mod export;
mod features;
mod sheet;
mod state;
mod utils;

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
            app::tauri::acknowledge_error,
            app::tauri::cancel_exit,
            app::tauri::close_all_documents,
            app::tauri::close_current_document,
            app::tauri::close_document,
            app::tauri::focus_document,
            app::tauri::get_state,
            app::tauri::new_document,
            app::tauri::open_documents,
            app::tauri::request_exit,
            app::tauri::reveal_in_explorer,
            app::tauri::save_all,
            app::tauri::show_error_message,
            // Document
            app::tauri::apply_direction_preset,
            app::tauri::begin_drag_and_drop_frame,
            app::tauri::begin_drag_and_drop_keyframe,
            app::tauri::begin_drag_keyframe_duration,
            app::tauri::begin_export_as,
            app::tauri::begin_nudge_hitbox,
            app::tauri::begin_nudge_keyframe,
            app::tauri::begin_rename_animation,
            app::tauri::begin_rename_hitbox,
            app::tauri::begin_rename_selection,
            app::tauri::begin_resize_hitbox,
            app::tauri::browse_selection,
            app::tauri::browse_to_end,
            app::tauri::browse_to_start,
            app::tauri::cancel_export_as,
            app::tauri::cancel_rename,
            app::tauri::center_workbench,
            app::tauri::clear_selection,
            app::tauri::close_without_saving,
            app::tauri::copy,
            app::tauri::create_animation,
            app::tauri::create_hitbox,
            app::tauri::cut,
            app::tauri::delete_animation,
            app::tauri::delete_frame,
            app::tauri::delete_hitbox,
            app::tauri::delete_selected_animations,
            app::tauri::delete_selected_frames,
            app::tauri::delete_selected_hitboxes,
            app::tauri::delete_selected_keyframes,
            app::tauri::delete_selection,
            app::tauri::disable_sprite_darkening,
            app::tauri::drop_frame_on_timeline,
            app::tauri::drop_keyframe_on_timeline,
            app::tauri::edit_animation,
            app::tauri::enable_sprite_darkening,
            app::tauri::end_drag_and_drop_frame,
            app::tauri::end_drag_and_drop_keyframe,
            app::tauri::end_drag_keyframe_duration,
            app::tauri::end_export_as,
            app::tauri::end_nudge_hitbox,
            app::tauri::end_nudge_keyframe,
            app::tauri::end_rename_animation,
            app::tauri::end_rename_hitbox,
            app::tauri::end_resize_hitbox,
            app::tauri::export,
            app::tauri::filter_animations,
            app::tauri::filter_frames,
            app::tauri::hide_hitboxes,
            app::tauri::hide_origin,
            app::tauri::hide_sprite,
            app::tauri::import_frames,
            app::tauri::jump_to_animation_end,
            app::tauri::jump_to_animation_start,
            app::tauri::jump_to_next_frame,
            app::tauri::jump_to_previous_frame,
            app::tauri::lock_hitboxes,
            app::tauri::nudge_selection,
            app::tauri::pan_timeline,
            app::tauri::pan,
            app::tauri::paste,
            app::tauri::pause,
            app::tauri::play,
            app::tauri::redo,
            app::tauri::begin_relocate_frames,
            app::tauri::relocate_frame,
            app::tauri::end_relocate_frames,
            app::tauri::cancel_relocate_frames,
            app::tauri::reset_timeline_zoom,
            app::tauri::reset_workbench_zoom,
            app::tauri::save_as,
            app::tauri::save,
            app::tauri::scrub_timeline,
            app::tauri::select_all,
            app::tauri::select_animation,
            app::tauri::select_direction,
            app::tauri::select_frame,
            app::tauri::select_hitbox,
            app::tauri::select_keyframe,
            app::tauri::set_animation_looping,
            app::tauri::set_animations_list_offset,
            app::tauri::set_export_metadata_file,
            app::tauri::set_export_metadata_paths_root,
            app::tauri::set_export_template_file,
            app::tauri::set_export_atlas_image_file,
            app::tauri::set_frames_list_mode,
            app::tauri::set_frames_list_offset,
            app::tauri::set_hitbox_height,
            app::tauri::set_hitbox_position_x,
            app::tauri::set_hitbox_position_y,
            app::tauri::set_hitbox_width,
            app::tauri::set_hitboxes_list_offset,
            app::tauri::set_keyframe_duration,
            app::tauri::set_keyframe_offset_x,
            app::tauri::set_keyframe_offset_y,
            app::tauri::set_keyframe_snapping_base_duration,
            app::tauri::set_snap_keyframe_durations,
            app::tauri::set_snap_keyframes_to_multiples_of_duration,
            app::tauri::set_snap_keyframes_to_other_keyframes,
            app::tauri::set_timeline_offset,
            app::tauri::set_timeline_zoom_amount,
            app::tauri::set_workbench_zoom_factor,
            app::tauri::show_hitboxes,
            app::tauri::show_origin,
            app::tauri::show_sprite,
            app::tauri::tick,
            app::tauri::toggle_preserve_aspect_ratio,
            app::tauri::undo,
            app::tauri::unlock_hitboxes,
            app::tauri::update_drag_keyframe_duration,
            app::tauri::update_nudge_hitbox,
            app::tauri::update_nudge_keyframe,
            app::tauri::update_resize_hitbox,
            app::tauri::zoom_in_timeline_around,
            app::tauri::zoom_in_timeline,
            app::tauri::zoom_in_workbench_around,
            app::tauri::zoom_in_workbench,
            app::tauri::zoom_out_timeline_around,
            app::tauri::zoom_out_timeline,
            app::tauri::zoom_out_workbench_around,
            app::tauri::zoom_out_workbench,
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
