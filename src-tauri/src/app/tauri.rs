use std::path::{Path, PathBuf};

use json_patch::Patch;
use log::error;
use serde::Serialize;
use tauri::{ClipboardManager, Manager};

use crate::{
    api::Api,
    app::TigerApp,
    dto::{self, StateTrim},
    features::texture_cache,
    state::{self, State},
    utils::paths,
};

static EVENT_PATCH_STATE: &str = "patch-state";
static EVENT_REPLACE_STATE: &str = "replace-state";

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
        let patch = self.patch(state_trim, operation);
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

#[tauri::command]
pub fn get_state(app: tauri::AppHandle) -> Result<dto::State, ()> {
    app.get_state()
}

#[tauri::command]
pub fn show_error_message(
    app: tauri::AppHandle,
    title: String,
    summary: String,
    details: String,
) -> Result<Patch, ()> {
    app.show_error_message(title, summary, details)
}

#[tauri::command]
pub fn acknowledge_error(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.acknowledge_error()
}

#[tauri::command]
pub fn new_document(app: tauri::AppHandle, path: PathBuf) -> Result<Patch, ()> {
    app.new_document(path)
}

#[tauri::command]
pub async fn open_documents(app: tauri::AppHandle, paths: Vec<&Path>) -> Result<Patch, ()> {
    app.open_documents(paths).await
}

#[tauri::command]
pub fn focus_document(app: tauri::AppHandle, path: PathBuf) -> Result<Patch, ()> {
    app.focus_document(path)
}

#[tauri::command]
pub fn close_all_documents(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.close_all_documents()
}

#[tauri::command]
pub fn close_current_document(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.close_current_document()
}

#[tauri::command]
pub fn close_document(app: tauri::AppHandle, path: PathBuf) -> Result<Patch, ()> {
    app.close_document(path)
}

#[tauri::command]
pub fn request_exit(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.request_exit()
}

#[tauri::command]
pub fn cancel_exit(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.cancel_exit()
}

#[tauri::command]
pub fn reveal_in_explorer(path: PathBuf) {
    // For future improvements, see https://github.com/tauri-apps/tauri/issues/4062
    #[cfg(windows)]
    std::process::Command::new("explorer")
        .args(["/select,", path.to_string_lossy().as_ref()]) // The comma after select is not a typo
        .spawn()
        .unwrap();
}

#[tauri::command]
pub fn close_without_saving(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.close_without_saving()
}

#[tauri::command]
pub async fn save(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.save().await
}

#[tauri::command]
pub async fn save_all(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.save_all().await
}

#[tauri::command]
pub async fn save_as(app: tauri::AppHandle, new_path: PathBuf) -> Result<Patch, ()> {
    app.save_as(new_path).await
}

#[tauri::command]
pub fn undo(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.undo()
}

#[tauri::command]
pub fn redo(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.redo()
}

#[tauri::command]
pub fn copy(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.copy()
}

#[tauri::command]
pub fn cut(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.cut()
}

#[tauri::command]
pub fn paste(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.paste()
}

#[tauri::command]
pub fn set_frames_list_mode(app: tauri::AppHandle, list_mode: dto::ListMode) -> Result<Patch, ()> {
    app.set_frames_list_mode(list_mode)
}

#[tauri::command]
pub fn set_frames_list_offset(app: tauri::AppHandle, offset: u32) -> Result<Patch, ()> {
    app.set_frames_list_offset(offset)
}

#[tauri::command]
pub fn set_hitboxes_list_offset(app: tauri::AppHandle, offset: u32) -> Result<Patch, ()> {
    app.set_hitboxes_list_offset(offset)
}

#[tauri::command]
pub fn filter_frames(app: tauri::AppHandle, search_query: String) -> Result<Patch, ()> {
    app.filter_frames(search_query)
}

#[tauri::command]
pub fn filter_animations(app: tauri::AppHandle, search_query: String) -> Result<Patch, ()> {
    app.filter_animations(search_query)
}

#[tauri::command]
pub fn set_animations_list_offset(app: tauri::AppHandle, offset: u32) -> Result<Patch, ()> {
    app.set_animations_list_offset(offset)
}

#[tauri::command]
pub fn import_frames(app: tauri::AppHandle, paths: Vec<PathBuf>) -> Result<Patch, ()> {
    app.import_frames(paths)
}

#[tauri::command]
pub fn begin_relocate_frames(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.begin_relocate_frames()
}

#[tauri::command]
pub fn relocate_frame(app: tauri::AppHandle, from: PathBuf, to: PathBuf) -> Result<Patch, ()> {
    app.relocate_frame(from, to)
}

#[tauri::command]
pub fn cancel_relocate_frames(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.cancel_relocate_frames()
}

#[tauri::command]
pub fn end_relocate_frames(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.end_relocate_frames()
}

#[tauri::command]
pub fn delete_frame(app: tauri::AppHandle, path: PathBuf) -> Result<Patch, ()> {
    app.delete_frame(path)
}

#[tauri::command]
pub fn delete_selected_frames(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.delete_selected_frames()
}

#[tauri::command]
pub fn delete_selection(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.delete_selection()
}

#[tauri::command]
pub fn nudge_selection(
    app: tauri::AppHandle,
    direction: dto::NudgeDirection,
    large_nudge: bool,
) -> Result<Patch, ()> {
    app.nudge_selection(direction, large_nudge)
}

#[tauri::command]
pub fn browse_selection(
    app: tauri::AppHandle,
    direction: dto::BrowseDirection,
    shift: bool,
) -> Result<Patch, ()> {
    app.browse_selection(direction, shift)
}

#[tauri::command]
pub fn browse_to_end(app: tauri::AppHandle, shift: bool) -> Result<Patch, ()> {
    app.browse_to_end(shift)
}

#[tauri::command]
pub fn browse_to_start(app: tauri::AppHandle, shift: bool) -> Result<Patch, ()> {
    app.browse_to_start(shift)
}

#[tauri::command]
pub fn clear_selection(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.clear_selection()
}

#[tauri::command]
pub fn select_all(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.select_all()
}

#[tauri::command]
pub fn select_animation(
    app: tauri::AppHandle,
    name: &str,
    shift: bool,
    ctrl: bool,
) -> Result<Patch, ()> {
    app.select_animation(name, shift, ctrl)
}

#[tauri::command]
pub fn select_frame(
    app: tauri::AppHandle,
    path: PathBuf,
    shift: bool,
    ctrl: bool,
) -> Result<Patch, ()> {
    app.select_frame(path, shift, ctrl)
}

#[tauri::command]
pub fn select_hitbox(
    app: tauri::AppHandle,
    name: &str,
    shift: bool,
    ctrl: bool,
) -> Result<Patch, ()> {
    app.select_hitbox(name, shift, ctrl)
}

#[tauri::command]
pub fn select_keyframe(
    app: tauri::AppHandle,
    direction: dto::Direction,
    index: usize,
    shift: bool,
    ctrl: bool,
) -> Result<Patch, ()> {
    app.select_keyframe(direction, index, shift, ctrl)
}

#[tauri::command]
pub fn pan(app: tauri::AppHandle, delta: (f32, f32)) -> Result<Patch, ()> {
    app.pan(delta)
}

#[tauri::command]
pub fn center_workbench(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.center_workbench()
}

#[tauri::command]
pub fn zoom_in_workbench(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.zoom_in_workbench()
}

#[tauri::command]
pub fn zoom_out_workbench(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.zoom_out_workbench()
}

#[tauri::command]
pub fn zoom_in_workbench_around(
    app: tauri::AppHandle,
    fixed_point: (f32, f32),
) -> Result<Patch, ()> {
    app.zoom_in_workbench_around(fixed_point)
}

#[tauri::command]
pub fn zoom_out_workbench_around(
    app: tauri::AppHandle,
    fixed_point: (f32, f32),
) -> Result<Patch, ()> {
    app.zoom_out_workbench_around(fixed_point)
}

#[tauri::command]
pub fn set_workbench_zoom_factor(app: tauri::AppHandle, zoom_factor: u32) -> Result<Patch, ()> {
    app.set_workbench_zoom_factor(zoom_factor)
}

#[tauri::command]
pub fn reset_workbench_zoom(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.reset_workbench_zoom()
}

#[tauri::command]
pub fn enable_sprite_darkening(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.enable_sprite_darkening()
}

#[tauri::command]
pub fn disable_sprite_darkening(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.disable_sprite_darkening()
}

#[tauri::command]
pub fn hide_sprite(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.hide_sprite()
}

#[tauri::command]
pub fn show_sprite(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.show_sprite()
}

#[tauri::command]
pub fn hide_hitboxes(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.hide_hitboxes()
}

#[tauri::command]
pub fn show_hitboxes(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.show_hitboxes()
}

#[tauri::command]
pub fn hide_origin(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.hide_origin()
}

#[tauri::command]
pub fn show_origin(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.show_origin()
}

#[tauri::command]
pub fn create_animation(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.create_animation()
}

#[tauri::command]
pub fn edit_animation(app: tauri::AppHandle, name: &str) -> Result<Patch, ()> {
    app.edit_animation(name)
}

#[tauri::command]
pub fn begin_rename_animation(app: tauri::AppHandle, animation_name: String) -> Result<Patch, ()> {
    app.begin_rename_animation(animation_name)
}

#[tauri::command]
pub fn begin_rename_hitbox(app: tauri::AppHandle, hitbox_name: String) -> Result<Patch, ()> {
    app.begin_rename_hitbox(hitbox_name)
}

#[tauri::command]
pub fn begin_rename_selection(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.begin_rename_selection()
}

#[tauri::command]
pub fn cancel_rename(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.cancel_rename()
}

#[tauri::command]
pub fn end_rename_animation(app: tauri::AppHandle, new_name: String) -> Result<Patch, ()> {
    app.end_rename_animation(new_name)
}

#[tauri::command]
pub fn end_rename_hitbox(app: tauri::AppHandle, new_name: String) -> Result<Patch, ()> {
    app.end_rename_hitbox(new_name)
}

#[tauri::command]
pub fn delete_animation(app: tauri::AppHandle, name: String) -> Result<Patch, ()> {
    app.delete_animation(name)
}

#[tauri::command]
pub fn delete_selected_animations(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.delete_selected_animations()
}

#[tauri::command]
pub fn tick(app: tauri::AppHandle, delta_time_millis: f64) -> Result<Patch, ()> {
    app.tick(delta_time_millis)
}

#[tauri::command]
pub fn play(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.play()
}

#[tauri::command]
pub fn pause(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.pause()
}

#[tauri::command]
pub fn scrub_timeline(app: tauri::AppHandle, time_millis: u64) -> Result<Patch, ()> {
    app.scrub_timeline(time_millis)
}

#[tauri::command]
pub fn jump_to_animation_start(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.jump_to_animation_start()
}

#[tauri::command]
pub fn jump_to_animation_end(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.jump_to_animation_end()
}

#[tauri::command]
pub fn jump_to_previous_frame(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.jump_to_previous_frame()
}

#[tauri::command]
pub fn jump_to_next_frame(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.jump_to_next_frame()
}

#[tauri::command]
pub fn set_snap_keyframe_durations(app: tauri::AppHandle, snap: bool) -> Result<Patch, ()> {
    app.set_snap_keyframe_durations(snap)
}

#[tauri::command]
pub fn set_snap_keyframes_to_other_keyframes(
    app: tauri::AppHandle,
    snap: bool,
) -> Result<Patch, ()> {
    app.set_snap_keyframes_to_other_keyframes(snap)
}

#[tauri::command]
pub fn set_snap_keyframes_to_multiples_of_duration(
    app: tauri::AppHandle,
    snap: bool,
) -> Result<Patch, ()> {
    app.set_snap_keyframes_to_multiples_of_duration(snap)
}

#[tauri::command]
pub fn set_keyframe_snapping_base_duration(
    app: tauri::AppHandle,
    duration_millis: u64,
) -> Result<Patch, ()> {
    app.set_keyframe_snapping_base_duration(duration_millis)
}

#[tauri::command]
pub fn zoom_in_timeline(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.zoom_in_timeline()
}

#[tauri::command]
pub fn zoom_out_timeline(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.zoom_out_timeline()
}

#[tauri::command]
pub fn zoom_in_timeline_around(app: tauri::AppHandle, fixed_point: f32) -> Result<Patch, ()> {
    app.zoom_in_timeline_around(fixed_point)
}

#[tauri::command]
pub fn zoom_out_timeline_around(app: tauri::AppHandle, fixed_point: f32) -> Result<Patch, ()> {
    app.zoom_out_timeline_around(fixed_point)
}

#[tauri::command]
pub fn set_timeline_zoom_amount(app: tauri::AppHandle, amount: f32) -> Result<Patch, ()> {
    app.set_timeline_zoom_amount(amount)
}

#[tauri::command]
pub fn reset_timeline_zoom(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.reset_timeline_zoom()
}

#[tauri::command]
pub fn set_timeline_offset(app: tauri::AppHandle, offset_millis: f32) -> Result<Patch, ()> {
    app.set_timeline_offset(offset_millis)
}

#[tauri::command]
pub fn pan_timeline(app: tauri::AppHandle, delta: f32) -> Result<Patch, ()> {
    app.pan_timeline(delta)
}

#[tauri::command]
pub fn set_animation_looping(app: tauri::AppHandle, is_looping: bool) -> Result<Patch, ()> {
    app.set_animation_looping(is_looping)
}

#[tauri::command]
pub fn apply_direction_preset(
    app: tauri::AppHandle,
    preset: dto::DirectionPreset,
) -> Result<Patch, ()> {
    app.apply_direction_preset(preset)
}

#[tauri::command]
pub fn select_direction(app: tauri::AppHandle, direction: dto::Direction) -> Result<Patch, ()> {
    app.select_direction(direction)
}

#[tauri::command]
pub fn begin_drag_and_drop_frame(app: tauri::AppHandle, frame: PathBuf) -> Result<Patch, ()> {
    app.begin_drag_and_drop_frame(frame)
}

#[tauri::command]
pub fn drop_frame_on_timeline(
    app: tauri::AppHandle,
    direction: dto::Direction,
    index: usize,
) -> Result<Patch, ()> {
    app.drop_frame_on_timeline(direction, index)
}

#[tauri::command]
pub fn end_drag_and_drop_frame(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.end_drag_and_drop_frame()
}

#[tauri::command]
pub fn delete_selected_keyframes(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.delete_selected_keyframes()
}

#[tauri::command]
pub fn set_keyframe_duration(app: tauri::AppHandle, duration_millis: u64) -> Result<Patch, ()> {
    app.set_keyframe_duration(duration_millis)
}

#[tauri::command]
pub fn set_keyframe_offset_x(app: tauri::AppHandle, x: i32) -> Result<Patch, ()> {
    app.set_keyframe_offset_x(x)
}

#[tauri::command]
pub fn set_keyframe_offset_y(app: tauri::AppHandle, y: i32) -> Result<Patch, ()> {
    app.set_keyframe_offset_x(y)
}

#[tauri::command]
pub fn begin_drag_and_drop_keyframe(
    app: tauri::AppHandle,
    direction: dto::Direction,
    index: usize,
) -> Result<Patch, ()> {
    app.begin_drag_and_drop_keyframe(direction, index)
}

#[tauri::command]
pub fn drop_keyframe_on_timeline(
    app: tauri::AppHandle,
    direction: dto::Direction,
    index: usize,
) -> Result<Patch, ()> {
    app.drop_keyframe_on_timeline(direction, index)
}

#[tauri::command]
pub fn end_drag_and_drop_keyframe(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.end_drag_and_drop_keyframe()
}

#[tauri::command]
pub fn begin_drag_keyframe_duration(
    app: tauri::AppHandle,
    direction: dto::Direction,
    index: usize,
) -> Result<Patch, ()> {
    app.begin_drag_keyframe_duration(direction, index)
}

#[tauri::command]
pub fn update_drag_keyframe_duration(
    app: tauri::AppHandle,
    delta_millis: i64,
) -> Result<Patch, ()> {
    app.update_drag_keyframe_duration(delta_millis)
}

#[tauri::command]
pub fn end_drag_keyframe_duration(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.end_drag_keyframe_duration()
}

#[tauri::command]
pub fn begin_nudge_keyframe(
    app: tauri::AppHandle,
    direction: dto::Direction,
    index: usize,
) -> Result<Patch, ()> {
    app.begin_nudge_keyframe(direction, index)
}

#[tauri::command]
pub fn update_nudge_keyframe(
    app: tauri::AppHandle,
    displacement: (i32, i32),
    both_axis: bool,
) -> Result<Patch, ()> {
    app.update_nudge_keyframe(displacement, both_axis)
}

#[tauri::command]
pub fn end_nudge_keyframe(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.end_nudge_keyframe()
}

#[tauri::command]
pub fn create_hitbox(app: tauri::AppHandle, position: Option<(i32, i32)>) -> Result<Patch, ()> {
    app.create_hitbox(position)
}

#[tauri::command]
pub fn delete_hitbox(app: tauri::AppHandle, name: String) -> Result<Patch, ()> {
    app.delete_hitbox(name)
}

#[tauri::command]
pub fn delete_selected_hitboxes(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.delete_selected_hitboxes()
}

#[tauri::command]
pub fn lock_hitboxes(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.lock_hitboxes()
}

#[tauri::command]
pub fn unlock_hitboxes(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.unlock_hitboxes()
}

#[tauri::command]
pub fn set_hitbox_height(app: tauri::AppHandle, height: u32) -> Result<Patch, ()> {
    app.set_hitbox_height(height)
}

#[tauri::command]
pub fn set_hitbox_width(app: tauri::AppHandle, width: u32) -> Result<Patch, ()> {
    app.set_hitbox_width(width)
}

#[tauri::command]
pub fn set_hitbox_position_x(app: tauri::AppHandle, x: i32) -> Result<Patch, ()> {
    app.set_hitbox_position_x(x)
}

#[tauri::command]
pub fn set_hitbox_position_y(app: tauri::AppHandle, y: i32) -> Result<Patch, ()> {
    app.set_hitbox_position_y(y)
}

#[tauri::command]
pub fn toggle_preserve_aspect_ratio(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.toggle_preserve_aspect_ratio()
}

#[tauri::command]
pub fn begin_nudge_hitbox(app: tauri::AppHandle, name: String) -> Result<Patch, ()> {
    app.begin_nudge_hitbox(name)
}

#[tauri::command]
pub fn update_nudge_hitbox(
    app: tauri::AppHandle,
    displacement: (i32, i32),
    both_axis: bool,
) -> Result<Patch, ()> {
    app.update_nudge_hitbox(displacement, both_axis)
}

#[tauri::command]
pub fn end_nudge_hitbox(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.end_nudge_hitbox()
}

#[tauri::command]
pub fn begin_resize_hitbox(
    app: tauri::AppHandle,
    name: String,
    axis: dto::ResizeAxis,
) -> Result<Patch, ()> {
    app.begin_resize_hitbox(name, axis)
}

#[tauri::command]
pub fn update_resize_hitbox(
    app: tauri::AppHandle,
    displacement: (i32, i32),
    preserve_aspect_ratio: bool,
) -> Result<Patch, ()> {
    app.update_resize_hitbox(displacement, preserve_aspect_ratio)
}

#[tauri::command]
pub fn end_resize_hitbox(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.end_resize_hitbox()
}

#[tauri::command]
pub async fn export(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.export().await
}

#[tauri::command]
pub fn begin_export_as(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.begin_export_as()
}

#[tauri::command]
pub fn set_export_template_file(app: tauri::AppHandle, file: PathBuf) -> Result<Patch, ()> {
    app.set_export_template_file(file)
}

#[tauri::command]
pub fn set_export_atlas_image_file(app: tauri::AppHandle, file: PathBuf) -> Result<Patch, ()> {
    app.set_export_atlas_image_file(file)
}

#[tauri::command]
pub fn set_export_metadata_file(app: tauri::AppHandle, file: PathBuf) -> Result<Patch, ()> {
    app.set_export_metadata_file(file)
}

#[tauri::command]
pub fn set_export_metadata_paths_root(
    app: tauri::AppHandle,
    directory: PathBuf,
) -> Result<Patch, ()> {
    app.set_export_metadata_paths_root(directory)
}

#[tauri::command]
pub fn cancel_export_as(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.cancel_export_as()
}

#[tauri::command]
pub async fn end_export_as(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.end_export_as().await
}
