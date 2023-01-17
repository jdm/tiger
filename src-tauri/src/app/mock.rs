use json_patch::Patch;
use std::{
    ops::Deref,
    path::{Path, PathBuf},
};

use crate::{
    api::Api,
    app::TigerApp,
    dto,
    features::{
        self,
        template_hot_reload::TemplateHotReloadInfo,
        texture_cache::{self, TextureCacheInfo},
        texture_hot_reload::TextureHotReloadInfo,
    },
    state::{self, State},
    utils::{
        handle,
        paths::{self, Paths},
    },
};

#[derive(Clone)]
pub struct TigerAppMock {
    paths: paths::Handle,
    state: state::Handle,
    texture_cache: texture_cache::Handle,
    client_state: handle::Handle<dto::State>,
    events: handle::Handle<Vec<(String, serde_json::Value)>>,
    clipboard: handle::Handle<Option<String>>,
    closed: handle::Handle<bool>,
    template_hot_reload_info: Option<TemplateHotReloadInfo>,
    texture_cache_info: Option<TextureCacheInfo>,
    texture_hot_reload_info: Option<TextureHotReloadInfo>,
}

pub struct TigerAppMockBuilder {
    paths: Paths,
}

impl TigerAppMockBuilder {
    pub fn new() -> Self {
        let paths = Paths::test_outputs();
        paths.remove_all();
        Self { paths }
    }

    pub fn paths(&self) -> &Paths {
        &self.paths
    }

    pub fn paths_mut(&mut self) -> &mut Paths {
        &mut self.paths
    }

    pub fn build(self) -> TigerAppMock {
        let mut app = TigerAppMock {
            paths: handle::Handle::new(self.paths),
            state: state::Handle::default(),
            texture_cache: texture_cache::Handle::default(),
            client_state: handle::Handle::new(State::default().to_dto(dto::StateTrim::Full)),
            events: handle::Handle::default(),
            clipboard: handle::Handle::default(),
            closed: handle::Handle::default(),
            template_hot_reload_info: None,
            texture_cache_info: None,
            texture_hot_reload_info: None,
        };

        features::texture_cache::init(app.clone());
        features::clipboard_analysis::init(app.clone());
        features::missing_textures::init(app.clone());
        features::onboarding::init(app.clone());
        features::recent_documents::init(app.clone());
        app.template_hot_reload_info = Some(features::template_hot_reload::init(app.clone()));
        app.texture_cache_info = Some(features::texture_cache::init(app.clone()));
        app.texture_hot_reload_info = Some(features::texture_hot_reload::init(app.clone()));
        app.replace_state();

        app
    }
}

impl TigerAppMock {
    pub fn new() -> Self {
        TigerAppMockBuilder::new().build()
    }

    pub fn client_state(&self) -> dto::State {
        self.client_state.lock().clone()
    }

    pub fn events(&self) -> Vec<(String, serde_json::Value)> {
        self.events.lock().clone()
    }

    pub fn texture_cache_info(&self) -> TextureCacheInfo {
        self.texture_cache_info.as_ref().unwrap().clone()
    }

    pub fn texture_hot_reload_info(&self) -> TextureHotReloadInfo {
        self.texture_hot_reload_info.as_ref().unwrap().clone()
    }

    pub fn template_hot_reload_info(&self) -> TemplateHotReloadInfo {
        self.template_hot_reload_info.as_ref().unwrap().clone()
    }

    pub fn is_closed(&self) -> bool {
        *self.closed.lock()
    }

    pub fn document(&self) -> dto::Document {
        let state = self.client_state();
        state
            .documents
            .into_iter()
            .find(|d| &d.path == state.current_document_path.as_ref().unwrap())
            .unwrap()
    }

    fn apply_patch(&self, patch: Patch) {
        let mut client_state = serde_json::to_value(self.client_state.lock().deref()).unwrap();
        json_patch::patch(&mut client_state, &patch).unwrap();
        *self.client_state.lock() = serde_json::from_value(client_state).unwrap();
    }
}

impl TigerApp for TigerAppMock {
    fn state(&self) -> state::Handle {
        self.state.clone()
    }

    fn texture_cache(&self) -> texture_cache::Handle {
        self.texture_cache.clone()
    }

    fn paths(&self) -> paths::Handle {
        self.paths.clone()
    }

    fn patch_state<F: FnOnce(&mut State)>(&self, state_trim: dto::StateTrim, operation: F) {
        let patch = self.patch(state_trim, operation);
        self.apply_patch(patch);
    }

    fn replace_state(&self) {
        let state_handle = self.state();
        let state = state_handle.lock();
        *self.client_state.lock() = state.to_dto(dto::StateTrim::Full);
    }

    fn emit_all<S: serde::Serialize + Clone>(&self, event: &str, payload: S) {
        self.events
            .lock()
            .push((event.to_owned(), serde_json::to_value(payload).unwrap()));
    }

    fn read_clipboard(&self) -> Option<String> {
        self.clipboard.lock().clone()
    }

    fn write_clipboard<S: Into<String>>(&self, content: S) {
        *self.clipboard.lock() = Some(content.into())
    }

    fn close_window(&self) {
        *self.closed.lock() = true;
    }
}

#[allow(dead_code)]
impl TigerAppMock {
    pub fn acknowledge_error(&self) {
        self.apply_patch(Api::acknowledge_error(self).unwrap());
    }

    pub fn apply_direction_preset(&self, preset: dto::DirectionPreset) {
        self.apply_patch(Api::apply_direction_preset(self, preset).unwrap());
    }

    pub fn begin_drag_and_drop_frame<P: Into<PathBuf>>(&self, frame: P) {
        self.apply_patch(Api::begin_drag_and_drop_frame(self, frame).unwrap());
    }

    pub fn begin_drag_and_drop_keyframe(&self, direction: dto::Direction, index: usize) {
        self.apply_patch(Api::begin_drag_and_drop_keyframe(self, direction, index).unwrap());
    }

    pub fn begin_drag_keyframe_duration(&self, direction: dto::Direction, index: usize) {
        self.apply_patch(Api::begin_drag_keyframe_duration(self, direction, index).unwrap());
    }

    pub fn begin_export_as(&self) {
        self.apply_patch(Api::begin_export_as(self).unwrap());
    }

    pub fn begin_nudge_hitbox<S: Into<String>>(&self, name: S) {
        self.apply_patch(Api::begin_nudge_hitbox(self, name).unwrap());
    }

    pub fn begin_nudge_keyframe(&self, direction: dto::Direction, index: usize) {
        self.apply_patch(Api::begin_nudge_keyframe(self, direction, index).unwrap());
    }

    pub fn begin_relocate_frames(&self) {
        self.apply_patch(Api::begin_relocate_frames(self).unwrap());
    }

    pub fn begin_rename_animation<S: Into<String>>(&self, animation_name: S) {
        self.apply_patch(Api::begin_rename_animation(self, animation_name).unwrap());
    }

    pub fn begin_rename_hitbox<S: Into<String>>(&self, hitbox_name: S) {
        self.apply_patch(Api::begin_rename_hitbox(self, hitbox_name).unwrap());
    }

    pub fn begin_rename_selection(&self) {
        self.apply_patch(Api::begin_rename_selection(self).unwrap());
    }

    pub fn begin_resize_hitbox<S: Into<String>>(&self, name: S, axis: dto::ResizeAxis) {
        self.apply_patch(Api::begin_resize_hitbox(self, name, axis).unwrap());
    }

    pub fn browse_selection(&self, direction: dto::BrowseDirection, shift: bool) {
        self.apply_patch(Api::browse_selection(self, direction, shift).unwrap());
    }

    pub fn browse_to_end(&self, shift: bool) {
        self.apply_patch(Api::browse_to_end(self, shift).unwrap());
    }

    pub fn browse_to_start(&self, shift: bool) {
        self.apply_patch(Api::browse_to_start(self, shift).unwrap());
    }

    pub fn cancel_exit(&self) {
        self.apply_patch(Api::cancel_exit(self).unwrap());
    }

    pub fn cancel_export_as(&self) {
        self.apply_patch(Api::cancel_export_as(self).unwrap());
    }

    pub fn cancel_relocate_frames(&self) {
        self.apply_patch(Api::cancel_relocate_frames(self).unwrap());
    }

    pub fn cancel_rename(&self) {
        self.apply_patch(Api::cancel_rename(self).unwrap());
    }

    pub fn center_workbench(&self) {
        self.apply_patch(Api::center_workbench(self).unwrap());
    }

    pub fn clear_selection(&self) {
        self.apply_patch(Api::clear_selection(self).unwrap());
    }

    pub fn close_all_documents(&self) {
        self.apply_patch(Api::close_all_documents(self).unwrap());
    }

    pub fn close_current_document(&self) {
        self.apply_patch(Api::close_current_document(self).unwrap());
    }

    pub fn close_document<P: AsRef<Path>>(&self, path: P) {
        self.apply_patch(Api::close_document(self, path).unwrap());
    }

    pub fn close_without_saving(&self) {
        self.apply_patch(Api::close_without_saving(self).unwrap());
    }

    pub fn copy(&self) {
        self.apply_patch(Api::copy(self).unwrap());
    }

    pub fn create_animation(&self) {
        self.apply_patch(Api::create_animation(self).unwrap());
    }

    pub fn create_hitbox(&self) {
        self.apply_patch(Api::create_hitbox(self).unwrap());
    }

    pub fn cut(&self) {
        self.apply_patch(Api::cut(self).unwrap());
    }

    pub fn delete_animation<S: Into<String>>(&self, name: S) {
        self.apply_patch(Api::delete_animation(self, name).unwrap());
    }

    pub fn delete_frame<P: Into<PathBuf>>(&self, path: P) {
        self.apply_patch(Api::delete_frame(self, path).unwrap());
    }

    pub fn delete_hitbox<S: Into<String>>(&self, name: S) {
        self.apply_patch(Api::delete_hitbox(self, name).unwrap());
    }

    pub fn delete_selected_animations(&self) {
        self.apply_patch(Api::delete_selected_animations(self).unwrap());
    }

    pub fn delete_selected_frames(&self) {
        self.apply_patch(Api::delete_selected_frames(self).unwrap());
    }

    pub fn delete_selected_hitboxes(&self) {
        self.apply_patch(Api::delete_selected_hitboxes(self).unwrap());
    }

    pub fn delete_selected_keyframes(&self) {
        self.apply_patch(Api::delete_selected_keyframes(self).unwrap());
    }

    pub fn delete_selection(&self) {
        self.apply_patch(Api::delete_selection(self).unwrap());
    }

    pub fn disable_sprite_darkening(&self) {
        self.apply_patch(Api::disable_sprite_darkening(self).unwrap());
    }

    pub fn drop_frame_on_timeline(&self, direction: dto::Direction, index: usize) {
        self.apply_patch(Api::drop_frame_on_timeline(self, direction, index).unwrap());
    }

    pub fn drop_keyframe_on_timeline(&self, direction: dto::Direction, index: usize) {
        self.apply_patch(Api::drop_keyframe_on_timeline(self, direction, index).unwrap());
    }

    pub fn edit_animation<S: Into<String>>(&self, name: S) {
        self.apply_patch(Api::edit_animation(self, name).unwrap());
    }

    pub fn enable_sprite_darkening(&self) {
        self.apply_patch(Api::enable_sprite_darkening(self).unwrap());
    }

    pub fn end_drag_and_drop_frame(&self) {
        self.apply_patch(Api::end_drag_and_drop_frame(self).unwrap());
    }

    pub fn end_drag_and_drop_keyframe(&self) {
        self.apply_patch(Api::end_drag_and_drop_keyframe(self).unwrap());
    }

    pub fn end_drag_keyframe_duration(&self) {
        self.apply_patch(Api::end_drag_keyframe_duration(self).unwrap());
    }

    pub async fn end_export_as(&self) {
        self.apply_patch(Api::end_export_as(self).await.unwrap());
    }

    pub fn end_nudge_hitbox(&self) {
        self.apply_patch(Api::end_nudge_hitbox(self).unwrap());
    }

    pub fn end_nudge_keyframe(&self) {
        self.apply_patch(Api::end_nudge_keyframe(self).unwrap());
    }

    pub fn end_relocate_frames(&self) {
        self.apply_patch(Api::end_relocate_frames(self).unwrap());
    }

    pub fn end_rename_animation<S: Into<String>>(&self, new_name: S) {
        self.apply_patch(Api::end_rename_animation(self, new_name).unwrap());
    }

    pub fn end_rename_hitbox<S: Into<String>>(&self, new_name: S) {
        self.apply_patch(Api::end_rename_hitbox(self, new_name).unwrap());
    }

    pub fn end_resize_hitbox(&self) {
        self.apply_patch(Api::end_resize_hitbox(self).unwrap());
    }

    pub async fn export(&self) {
        self.apply_patch(Api::export(self).await.unwrap());
    }

    pub fn filter_animations<S: Into<String>>(&self, search_query: S) {
        self.apply_patch(Api::filter_animations(self, search_query).unwrap());
    }

    pub fn filter_frames<S: Into<String>>(&self, search_query: S) {
        self.apply_patch(Api::filter_frames(self, search_query).unwrap());
    }

    pub fn focus_document<P: AsRef<Path>>(&self, path: P) {
        self.apply_patch(Api::focus_document(self, path).unwrap());
    }

    pub fn focus_next_document(&self) {
        self.apply_patch(Api::focus_next_document(self).unwrap());
    }

    pub fn focus_previous_document(&self) {
        self.apply_patch(Api::focus_previous_document(self).unwrap());
    }

    pub fn hide_hitboxes(&self) {
        self.apply_patch(Api::hide_hitboxes(self).unwrap());
    }

    pub fn hide_origin(&self) {
        self.apply_patch(Api::hide_origin(self).unwrap());
    }

    pub fn hide_sprite(&self) {
        self.apply_patch(Api::hide_sprite(self).unwrap());
    }

    pub fn import_frames<P: Into<PathBuf>>(&self, paths: Vec<P>) {
        self.apply_patch(Api::import_frames(self, paths).unwrap());
    }

    pub fn jump_to_animation_end(&self) {
        self.apply_patch(Api::jump_to_animation_end(self).unwrap());
    }

    pub fn jump_to_animation_start(&self) {
        self.apply_patch(Api::jump_to_animation_start(self).unwrap());
    }

    pub fn jump_to_next_frame(&self) {
        self.apply_patch(Api::jump_to_next_frame(self).unwrap());
    }

    pub fn jump_to_previous_frame(&self) {
        self.apply_patch(Api::jump_to_previous_frame(self).unwrap());
    }

    pub fn lock_hitboxes(&self) {
        self.apply_patch(Api::lock_hitboxes(self).unwrap());
    }

    pub fn new_document<P: Into<PathBuf>>(&self, path: P) {
        self.apply_patch(Api::new_document(self, path).unwrap());
    }

    pub fn nudge_selection(&self, direction: dto::NudgeDirection, large_nudge: bool) {
        self.apply_patch(Api::nudge_selection(self, direction, large_nudge).unwrap());
    }

    pub async fn open_documents<P: Into<PathBuf> + Send + Sync>(&self, paths: Vec<P>) {
        self.apply_patch(Api::open_documents(self, paths).await.unwrap());
    }

    pub fn pan(&self, delta: (f32, f32)) {
        self.apply_patch(Api::pan(self, delta).unwrap());
    }

    pub fn pan_timeline(&self, delta: f32) {
        self.apply_patch(Api::pan_timeline(self, delta).unwrap());
    }

    pub fn paste(&self) {
        self.apply_patch(Api::paste(self).unwrap());
    }

    pub fn pause(&self) {
        self.apply_patch(Api::pause(self).unwrap());
    }

    pub fn play(&self) {
        self.apply_patch(Api::play(self).unwrap());
    }

    pub fn redo(&self) {
        self.apply_patch(Api::redo(self).unwrap());
    }

    pub fn relocate_frame<P: Into<PathBuf>, Q: Into<PathBuf>>(&self, from: P, to: Q) {
        self.apply_patch(Api::relocate_frame(self, from, to).unwrap());
    }

    pub fn request_exit(&self) {
        self.apply_patch(Api::request_exit(self).unwrap());
    }

    pub fn reset_timeline_zoom(&self) {
        self.apply_patch(Api::reset_timeline_zoom(self).unwrap());
    }

    pub fn reset_workbench_zoom(&self) {
        self.apply_patch(Api::reset_workbench_zoom(self).unwrap());
    }

    pub async fn save(&self) {
        self.apply_patch(Api::save(self).await.unwrap());
    }

    pub async fn save_all(&self) {
        self.apply_patch(Api::save_all(self).await.unwrap());
    }

    pub async fn save_as<P: Into<PathBuf> + Send + Sync>(&self, new_path: P) {
        self.apply_patch(Api::save_as(self, new_path).await.unwrap());
    }

    pub fn scrub_timeline(&self, time_millis: u64) {
        self.apply_patch(Api::scrub_timeline(self, time_millis).unwrap());
    }

    pub fn select_all(&self) {
        self.apply_patch(Api::select_all(self).unwrap());
    }

    pub fn select_animation<S: Into<String>>(&self, name: S, shift: bool, ctrl: bool) {
        self.apply_patch(Api::select_animation(self, name, shift, ctrl).unwrap());
    }

    pub fn select_direction(&self, direction: dto::Direction) {
        self.apply_patch(Api::select_direction(self, direction).unwrap());
    }

    pub fn select_frame<P: Into<PathBuf>>(&self, path: P, shift: bool, ctrl: bool) {
        self.apply_patch(Api::select_frame(self, path, shift, ctrl).unwrap());
    }

    pub fn select_hitbox<S: Into<String>>(&self, name: S, shift: bool, ctrl: bool) {
        self.apply_patch(Api::select_hitbox(self, name, shift, ctrl).unwrap());
    }

    pub fn select_keyframe(
        &self,
        direction: dto::Direction,
        index: usize,
        shift: bool,
        ctrl: bool,
    ) {
        self.apply_patch(Api::select_keyframe(self, direction, index, shift, ctrl).unwrap());
    }

    pub fn set_animation_looping(&self, is_looping: bool) {
        self.apply_patch(Api::set_animation_looping(self, is_looping).unwrap());
    }

    pub fn set_animations_list_offset(&self, offset: u32) {
        self.apply_patch(Api::set_animations_list_offset(self, offset).unwrap());
    }

    pub fn set_export_atlas_image_file<P: Into<PathBuf>>(&self, path: P) {
        self.apply_patch(Api::set_export_atlas_image_file(self, path).unwrap());
    }

    pub fn set_export_metadata_file<P: Into<PathBuf>>(&self, path: P) {
        self.apply_patch(Api::set_export_metadata_file(self, path).unwrap());
    }

    pub fn set_export_metadata_paths_root<P: Into<PathBuf>>(&self, path: P) {
        self.apply_patch(Api::set_export_metadata_paths_root(self, path).unwrap());
    }

    pub fn set_export_template_file<P: Into<PathBuf>>(&self, path: P) {
        self.apply_patch(Api::set_export_template_file(self, path).unwrap());
    }

    pub fn set_frames_list_mode(&self, list_mode: dto::ListMode) {
        self.apply_patch(Api::set_frames_list_mode(self, list_mode).unwrap());
    }

    pub fn set_frames_list_offset(&self, offset: u32) {
        self.apply_patch(Api::set_frames_list_offset(self, offset).unwrap());
    }

    pub fn set_hitbox_height(&self, height: u32) {
        self.apply_patch(Api::set_hitbox_height(self, height).unwrap());
    }

    pub fn set_hitbox_position_x(&self, x: i32) {
        self.apply_patch(Api::set_hitbox_position_x(self, x).unwrap());
    }

    pub fn set_hitbox_position_y(&self, y: i32) {
        self.apply_patch(Api::set_hitbox_position_y(self, y).unwrap());
    }

    pub fn set_hitbox_width(&self, width: u32) {
        self.apply_patch(Api::set_hitbox_width(self, width).unwrap());
    }

    pub fn set_hitboxes_list_offset(&self, offset: u32) {
        self.apply_patch(Api::set_hitboxes_list_offset(self, offset).unwrap());
    }

    pub fn set_keyframe_duration(&self, duration_millis: u64) {
        self.apply_patch(Api::set_keyframe_duration(self, duration_millis).unwrap());
    }

    pub fn set_keyframe_offset_x(&self, x: i32) {
        self.apply_patch(Api::set_keyframe_offset_x(self, x).unwrap());
    }

    pub fn set_keyframe_offset_y(&self, y: i32) {
        self.apply_patch(Api::set_keyframe_offset_y(self, y).unwrap());
    }

    pub fn set_keyframe_snapping_base_duration(&self, duration_millis: u64) {
        self.apply_patch(Api::set_keyframe_snapping_base_duration(self, duration_millis).unwrap());
    }

    pub fn set_snap_keyframe_durations(&self, snap: bool) {
        self.apply_patch(Api::set_snap_keyframe_durations(self, snap).unwrap());
    }

    pub fn set_snap_keyframes_to_multiples_of_duration(&self, snap: bool) {
        self.apply_patch(Api::set_snap_keyframes_to_multiples_of_duration(self, snap).unwrap());
    }

    pub fn set_snap_keyframes_to_other_keyframes(&self, snap: bool) {
        self.apply_patch(Api::set_snap_keyframes_to_other_keyframes(self, snap).unwrap());
    }

    pub fn set_timeline_offset(&self, offset_millis: f32) {
        self.apply_patch(Api::set_timeline_offset(self, offset_millis).unwrap());
    }

    pub fn set_timeline_zoom_amount(&self, amount: f32) {
        self.apply_patch(Api::set_timeline_zoom_amount(self, amount).unwrap());
    }

    pub fn set_workbench_zoom_factor(&self, zoom_factor: u32) {
        self.apply_patch(Api::set_workbench_zoom_factor(self, zoom_factor).unwrap());
    }

    pub fn show_error_message<S: Into<String>, T: Into<String>, U: Into<String>>(
        &self,
        title: S,
        summary: T,
        details: U,
    ) {
        self.apply_patch(Api::show_error_message(self, title, summary, details).unwrap());
    }

    pub fn show_hitboxes(&self) {
        self.apply_patch(Api::show_hitboxes(self).unwrap());
    }

    pub fn show_origin(&self) {
        self.apply_patch(Api::show_origin(self).unwrap());
    }

    pub fn show_sprite(&self) {
        self.apply_patch(Api::show_sprite(self).unwrap());
    }

    pub fn tick(&self, delta_time_millis: f64) {
        self.apply_patch(Api::tick(self, delta_time_millis).unwrap());
    }

    pub fn toggle_preserve_aspect_ratio(&self) {
        self.apply_patch(Api::toggle_preserve_aspect_ratio(self).unwrap());
    }

    pub fn undo(&self) {
        self.apply_patch(Api::undo(self).unwrap());
    }

    pub fn unlock_hitboxes(&self) {
        self.apply_patch(Api::unlock_hitboxes(self).unwrap());
    }

    pub fn update_drag_keyframe_duration(&self, duration_millis: i64) {
        self.apply_patch(Api::update_drag_keyframe_duration(self, duration_millis).unwrap());
    }

    pub fn update_nudge_hitbox(&self, displacement: (i32, i32), both_axis: bool) {
        self.apply_patch(Api::update_nudge_hitbox(self, displacement, both_axis).unwrap());
    }

    pub fn update_nudge_keyframe(&self, displacement: (i32, i32), both_axis: bool) {
        self.apply_patch(Api::update_nudge_keyframe(self, displacement, both_axis).unwrap());
    }

    pub fn update_resize_hitbox(&self, displacement: (i32, i32), preserve_aspect_ratio: bool) {
        self.apply_patch(
            Api::update_resize_hitbox(self, displacement, preserve_aspect_ratio).unwrap(),
        );
    }

    pub fn zoom_in_timeline(&self) {
        self.apply_patch(Api::zoom_in_timeline(self).unwrap());
    }

    pub fn zoom_in_timeline_around(&self, fixed_point: f32) {
        self.apply_patch(Api::zoom_in_timeline_around(self, fixed_point).unwrap());
    }

    pub fn zoom_in_workbench(&self) {
        self.apply_patch(Api::zoom_in_workbench(self).unwrap());
    }

    pub fn zoom_in_workbench_around(&self, fixed_point: (f32, f32)) {
        self.apply_patch(Api::zoom_in_workbench_around(self, fixed_point).unwrap());
    }

    pub fn zoom_out_timeline(&self) {
        self.apply_patch(Api::zoom_out_timeline(self).unwrap());
    }

    pub fn zoom_out_timeline_around(&self, fixed_point: f32) {
        self.apply_patch(Api::zoom_out_timeline_around(self, fixed_point).unwrap());
    }

    pub fn zoom_out_workbench(&self) {
        self.apply_patch(Api::zoom_out_workbench(self).unwrap());
    }

    pub fn zoom_out_workbench_around(&self, fixed_point: (f32, f32)) {
        self.apply_patch(Api::zoom_out_workbench_around(self, fixed_point).unwrap());
    }
}
