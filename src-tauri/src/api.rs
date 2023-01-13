use async_trait::async_trait;
use json_patch::Patch;
use log::error;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::app::TigerApp;
use crate::document::{Command, Document, DocumentResult};
use crate::dto::{self, StateTrim, ToFileName};
use crate::export::export_sheet;
use crate::sheet::{Absolute, Sheet};
use crate::state::{self, State};

impl state::Handle {
    pub fn mutate<F>(&self, state_trim: StateTrim, operation: F) -> Patch
    where
        F: FnOnce(&mut State),
    {
        let mut state = self.lock();

        let old_state: dto::State = state.to_dto(state_trim);
        operation(&mut state);
        let new_state: dto::State = state.to_dto(state_trim);

        let old_json = serde_json::to_value(old_state);
        let new_json = serde_json::to_value(new_state);

        match (old_json, new_json) {
            (Ok(o), Ok(n)) => json_patch::diff(&o, &n),
            _ => {
                error!("App state serialization error");
                Patch(Vec::new())
            }
        }
    }
}

struct DocumentToSave {
    sheet: Sheet<Absolute>,
    source: PathBuf,
    destination: PathBuf,
    version: i32,
}

#[async_trait]
pub trait Api {
    fn acknowledge_error(&self) -> Result<Patch, ()>;
    fn apply_direction_preset(&self, preset: dto::DirectionPreset) -> Result<Patch, ()>;
    fn begin_drag_and_drop_frame<P: Into<PathBuf>>(&self, frame: P) -> Result<Patch, ()>;
    fn begin_drag_and_drop_keyframe(
        &self,
        direction: dto::Direction,
        index: usize,
    ) -> Result<Patch, ()>;
    fn begin_drag_keyframe_duration(
        &self,
        direction: dto::Direction,
        index: usize,
    ) -> Result<Patch, ()>;
    fn begin_export_as(&self) -> Result<Patch, ()>;
    fn begin_nudge_hitbox<S: Into<String>>(&self, name: S) -> Result<Patch, ()>;
    fn begin_nudge_keyframe(&self, direction: dto::Direction, index: usize) -> Result<Patch, ()>;
    fn begin_relocate_frames(&self) -> Result<Patch, ()>;
    fn begin_rename_animation<S: Into<String>>(&self, animation_name: S) -> Result<Patch, ()>;
    fn begin_rename_hitbox<S: Into<String>>(&self, hitbox_name: S) -> Result<Patch, ()>;
    fn begin_rename_selection(&self) -> Result<Patch, ()>;
    fn begin_resize_hitbox<S: Into<String>>(
        &self,
        name: S,
        axis: dto::ResizeAxis,
    ) -> Result<Patch, ()>;
    fn browse_selection(&self, direction: dto::BrowseDirection, shift: bool) -> Result<Patch, ()>;
    fn browse_to_end(&self, shift: bool) -> Result<Patch, ()>;
    fn browse_to_start(&self, shift: bool) -> Result<Patch, ()>;
    fn cancel_exit(&self) -> Result<Patch, ()>;
    fn cancel_export_as(&self) -> Result<Patch, ()>;
    fn cancel_relocate_frames(&self) -> Result<Patch, ()>;
    fn cancel_rename(&self) -> Result<Patch, ()>;
    fn center_workbench(&self) -> Result<Patch, ()>;
    fn clear_selection(&self) -> Result<Patch, ()>;
    fn close_all_documents(&self) -> Result<Patch, ()>;
    fn close_current_document(&self) -> Result<Patch, ()>;
    fn close_document<P: AsRef<Path>>(&self, path: P) -> Result<Patch, ()>;
    fn close_without_saving(&self) -> Result<Patch, ()>;
    fn copy(&self) -> Result<Patch, ()>;
    fn create_animation(&self) -> Result<Patch, ()>;
    fn create_hitbox(&self, position: Option<(i32, i32)>) -> Result<Patch, ()>;
    fn cut(&self) -> Result<Patch, ()>;
    fn delete_animation<S: Into<String>>(&self, name: S) -> Result<Patch, ()>;
    fn delete_frame<P: Into<PathBuf>>(&self, path: P) -> Result<Patch, ()>;
    fn delete_hitbox<S: Into<String>>(&self, name: S) -> Result<Patch, ()>;
    fn delete_selected_animations(&self) -> Result<Patch, ()>;
    fn delete_selected_frames(&self) -> Result<Patch, ()>;
    fn delete_selected_hitboxes(&self) -> Result<Patch, ()>;
    fn delete_selected_keyframes(&self) -> Result<Patch, ()>;
    fn delete_selection(&self) -> Result<Patch, ()>;
    fn disable_sprite_darkening(&self) -> Result<Patch, ()>;
    fn drop_frame_on_timeline(&self, direction: dto::Direction, index: usize) -> Result<Patch, ()>;
    fn drop_keyframe_on_timeline(
        &self,
        direction: dto::Direction,
        index: usize,
    ) -> Result<Patch, ()>;
    fn edit_animation<S: Into<String>>(&self, name: S) -> Result<Patch, ()>;
    fn enable_sprite_darkening(&self) -> Result<Patch, ()>;
    fn end_drag_and_drop_frame(&self) -> Result<Patch, ()>;
    fn end_drag_and_drop_keyframe(&self) -> Result<Patch, ()>;
    fn end_drag_keyframe_duration(&self) -> Result<Patch, ()>;
    async fn end_export_as(&self) -> Result<Patch, ()>;
    fn end_nudge_hitbox(&self) -> Result<Patch, ()>;
    fn end_nudge_keyframe(&self) -> Result<Patch, ()>;
    fn end_relocate_frames(&self) -> Result<Patch, ()>;
    fn end_rename_animation<S: Into<String>>(&self, new_name: S) -> Result<Patch, ()>;
    fn end_rename_hitbox<S: Into<String>>(&self, new_name: S) -> Result<Patch, ()>;
    fn end_resize_hitbox(&self) -> Result<Patch, ()>;
    async fn export(&self) -> Result<Patch, ()>;
    fn filter_animations<S: Into<String>>(&self, search_query: S) -> Result<Patch, ()>;
    fn filter_frames<S: Into<String>>(&self, search_query: S) -> Result<Patch, ()>;
    fn focus_document<P: AsRef<Path>>(&self, path: P) -> Result<Patch, ()>;
    fn get_state(&self) -> Result<dto::State, ()>;
    fn hide_hitboxes(&self) -> Result<Patch, ()>;
    fn hide_origin(&self) -> Result<Patch, ()>;
    fn hide_sprite(&self) -> Result<Patch, ()>;
    fn import_frames<P: Into<PathBuf>>(&self, paths: Vec<P>) -> Result<Patch, ()>;
    fn jump_to_animation_end(&self) -> Result<Patch, ()>;
    fn jump_to_animation_start(&self) -> Result<Patch, ()>;
    fn jump_to_next_frame(&self) -> Result<Patch, ()>;
    fn jump_to_previous_frame(&self) -> Result<Patch, ()>;
    fn lock_hitboxes(&self) -> Result<Patch, ()>;
    fn new_document<P: Into<PathBuf>>(&self, path: P) -> Result<Patch, ()>;
    fn nudge_selection(
        &self,
        direction: dto::NudgeDirection,
        large_nudge: bool,
    ) -> Result<Patch, ()>;
    async fn open_documents<P: Into<PathBuf> + Send + Sync>(
        &self,
        paths: Vec<P>,
    ) -> Result<Patch, ()>;
    fn pan(&self, delta: (f32, f32)) -> Result<Patch, ()>;
    fn pan_timeline(&self, delta: f32) -> Result<Patch, ()>;
    fn paste(&self) -> Result<Patch, ()>;
    fn pause(&self) -> Result<Patch, ()>;
    fn play(&self) -> Result<Patch, ()>;
    fn redo(&self) -> Result<Patch, ()>;
    fn relocate_frame<F: Into<PathBuf>, T: Into<PathBuf>>(
        &self,
        from: F,
        to: T,
    ) -> Result<Patch, ()>;
    fn request_exit(&self) -> Result<Patch, ()>;
    fn reset_timeline_zoom(&self) -> Result<Patch, ()>;
    fn reset_workbench_zoom(&self) -> Result<Patch, ()>;
    async fn save(&self) -> Result<Patch, ()>;
    async fn save_all(&self) -> Result<Patch, ()>;
    async fn save_as<P: Into<PathBuf> + Send + Sync>(&self, path: P) -> Result<Patch, ()>;
    fn scrub_timeline(&self, time_millis: u64) -> Result<Patch, ()>;
    fn select_all(&self) -> Result<Patch, ()>;
    fn select_animation<S: Into<String>>(
        &self,
        name: S,
        shift: bool,
        ctrl: bool,
    ) -> Result<Patch, ()>;
    fn select_direction(&self, direction: dto::Direction) -> Result<Patch, ()>;
    fn select_frame<P: Into<PathBuf>>(&self, path: P, shift: bool, ctrl: bool)
        -> Result<Patch, ()>;
    fn select_hitbox<S: Into<String>>(&self, name: S, shift: bool, ctrl: bool)
        -> Result<Patch, ()>;
    fn select_keyframe(
        &self,
        direction: dto::Direction,
        index: usize,
        shift: bool,
        ctrl: bool,
    ) -> Result<Patch, ()>;
    fn set_animation_looping(&self, is_looping: bool) -> Result<Patch, ()>;
    fn set_animations_list_offset(&self, offset: u32) -> Result<Patch, ()>;
    fn set_export_atlas_image_file<P: Into<PathBuf>>(&self, file: P) -> Result<Patch, ()>;
    fn set_export_metadata_file<P: Into<PathBuf>>(&self, file: P) -> Result<Patch, ()>;
    fn set_export_metadata_paths_root<P: Into<PathBuf>>(&self, file: P) -> Result<Patch, ()>;
    fn set_export_template_file<P: Into<PathBuf>>(&self, file: P) -> Result<Patch, ()>;
    fn set_frames_list_mode(&self, list_mode: dto::ListMode) -> Result<Patch, ()>;
    fn set_frames_list_offset(&self, offset: u32) -> Result<Patch, ()>;
    fn set_hitbox_height(&self, height: u32) -> Result<Patch, ()>;
    fn set_hitbox_position_x(&self, x: i32) -> Result<Patch, ()>;
    fn set_hitbox_position_y(&self, y: i32) -> Result<Patch, ()>;
    fn set_hitbox_width(&self, width: u32) -> Result<Patch, ()>;
    fn set_hitboxes_list_offset(&self, offset: u32) -> Result<Patch, ()>;
    fn set_keyframe_duration(&self, duration_millies: u64) -> Result<Patch, ()>;
    fn set_keyframe_offset_x(&self, x: i32) -> Result<Patch, ()>;
    fn set_keyframe_offset_y(&self, y: i32) -> Result<Patch, ()>;
    fn set_keyframe_snapping_base_duration(&self, duration_millis: u64) -> Result<Patch, ()>;
    fn set_snap_keyframe_durations(&self, snap: bool) -> Result<Patch, ()>;
    fn set_snap_keyframes_to_multiples_of_duration(&self, snap: bool) -> Result<Patch, ()>;
    fn set_snap_keyframes_to_other_keyframes(&self, snap: bool) -> Result<Patch, ()>;
    fn set_timeline_offset(&self, offset_millis: f32) -> Result<Patch, ()>;
    fn set_timeline_zoom_amount(&self, amount: f32) -> Result<Patch, ()>;
    fn set_workbench_zoom_factor(&self, zoom_factor: u32) -> Result<Patch, ()>;
    fn show_error_message<S: Into<String>, T: Into<String>, U: Into<String>>(
        &self,
        title: S,
        summary: T,
        details: U,
    ) -> Result<Patch, ()>;
    fn show_hitboxes(&self) -> Result<Patch, ()>;
    fn show_origin(&self) -> Result<Patch, ()>;
    fn show_sprite(&self) -> Result<Patch, ()>;
    fn tick(&self, delta_time_millis: f64) -> Result<Patch, ()>;
    fn toggle_preserve_aspect_ratio(&self) -> Result<Patch, ()>;
    fn undo(&self) -> Result<Patch, ()>;
    fn unlock_hitboxes(&self) -> Result<Patch, ()>;
    fn update_drag_keyframe_duration(&self, delta_millis: i64) -> Result<Patch, ()>;
    fn update_nudge_hitbox(&self, displacement: (i32, i32), both_axis: bool) -> Result<Patch, ()>;
    fn update_nudge_keyframe(&self, displacement: (i32, i32), both_axis: bool)
        -> Result<Patch, ()>;
    fn update_resize_hitbox(
        &self,
        displacement: (i32, i32),
        preserve_aspect_ratio: bool,
    ) -> Result<Patch, ()>;
    fn zoom_in_timeline(&self) -> Result<Patch, ()>;
    fn zoom_in_timeline_around(&self, fixed_point: f32) -> Result<Patch, ()>;
    fn zoom_in_workbench(&self) -> Result<Patch, ()>;
    fn zoom_in_workbench_around(&self, fixed_point: (f32, f32)) -> Result<Patch, ()>;
    fn zoom_out_timeline(&self) -> Result<Patch, ()>;
    fn zoom_out_timeline_around(&self, fixed_point: f32) -> Result<Patch, ()>;
    fn zoom_out_workbench(&self) -> Result<Patch, ()>;
    fn zoom_out_workbench_around(&self, fixed_point: (f32, f32)) -> Result<Patch, ()>;
}

#[async_trait]
impl<A: TigerApp + Sync> Api for A {
    fn acknowledge_error(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            state.acknowledge_error();
        }))
    }

    fn apply_direction_preset(&self, preset: dto::DirectionPreset) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::ApplyDirectionPreset(preset.into()))
                    .ok();
            }
        }))
    }

    fn begin_drag_and_drop_frame<P: Into<PathBuf>>(&self, frame: P) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::BeginDragAndDropFrame(frame.into()))
                    .ok();
            }
        }))
    }

    fn begin_drag_and_drop_keyframe(
        &self,
        direction: dto::Direction,
        index: usize,
    ) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::BeginDragAndDropKeyframe(direction.into(), index))
                    .ok();
            }
        }))
    }

    fn begin_drag_keyframe_duration(
        &self,
        direction: dto::Direction,
        index: usize,
    ) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::BeginDragKeyframeDuration(direction.into(), index))
                    .ok();
            }
        }))
    }

    fn begin_export_as(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::BeginExportAs).ok();
            }
        }))
    }

    fn begin_nudge_hitbox<S: Into<String>>(&self, name: S) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::BeginNudgeHitbox(name.into()))
                    .ok();
            }
        }))
    }

    fn begin_nudge_keyframe(&self, direction: dto::Direction, index: usize) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::BeginNudgeKeyframe(direction.into(), index))
                    .ok();
            }
        }))
    }

    fn begin_relocate_frames(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::BeginRelocateFrames).ok();
            }
        }))
    }

    fn begin_rename_animation<S: Into<String>>(&self, animation_name: S) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::BeginRenameAnimation(animation_name.into()))
                    .ok();
            }
        }))
    }

    fn begin_rename_hitbox<S: Into<String>>(&self, hitbox_name: S) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::BeginRenameHitbox(hitbox_name.into()))
                    .ok();
            }
        }))
    }

    fn begin_rename_selection(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::BeginRenameSelection).ok();
            }
        }))
    }

    fn begin_resize_hitbox<S: Into<String>>(
        &self,
        name: S,
        axis: dto::ResizeAxis,
    ) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::BeginResizeHitbox(name.into(), axis.into()))
                    .ok();
            }
        }))
    }

    fn browse_selection(&self, direction: dto::BrowseDirection, shift: bool) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::BrowseSelection(direction.into(), shift))
                    .ok();
            }
        }))
    }

    fn browse_to_end(&self, shift: bool) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::BrowseToEnd(shift)).ok();
            }
        }))
    }

    fn browse_to_start(&self, shift: bool) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::BrowseToStart(shift)).ok();
            }
        }))
    }

    fn cancel_exit(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            state.cancel_exit();
        }))
    }

    fn cancel_export_as(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::CancelExportAs).ok();
            }
        }))
    }

    fn cancel_relocate_frames(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::CancelRelocateFrames).ok();
            }
        }))
    }

    fn cancel_rename(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::CancelRename).ok();
            }
        }))
    }

    fn center_workbench(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::CenterWorkbench).ok();
            }
        }))
    }

    fn clear_selection(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::ClearSelection).ok();
            }
        }))
    }

    fn close_all_documents(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            for document in state.documents_iter_mut() {
                document.request_close();
            }
            state.advance_exit();
            if state.should_exit() {
                self.close_window();
            }
        }))
    }

    fn close_current_document(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.request_close();
            }
            state.advance_exit();
            if state.should_exit() {
                self.close_window();
            }
        }))
    }

    fn close_document<P: AsRef<Path>>(&self, path: P) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.document_mut(path.as_ref()) {
                document.request_close();
            }
            state.advance_exit();
            if state.should_exit() {
                self.close_window();
            }
        }))
    }

    fn close_without_saving(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            let path = state.current_document().map(|d| d.path().to_owned());
            if let Some(path) = path {
                state.close_document(path);
                state.advance_exit();
                if state.should_exit() {
                    self.close_window();
                }
            }
        }))
    }

    fn copy(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(data) = state.current_document().and_then(|d| d.copy()) {
                if let Ok(serialized) = serde_json::to_string(&data) {
                    self.write_clipboard(serialized);
                    state.set_clipboard_manifest(Some(data.manifest()));
                }
            }
        }))
    }

    fn create_animation(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::CreateAnimation).ok();
            }
        }))
    }

    fn create_hitbox(&self, position: Option<(i32, i32)>) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::CreateHitbox(position.map(|p| p.into())))
                    .ok();
            }
        }))
    }

    fn cut(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(data) = state.current_document().and_then(|d| d.copy()) {
                if let Ok(serialized) = serde_json::to_string(&data) {
                    self.write_clipboard(serialized);
                    state.set_clipboard_manifest(Some(data.manifest()));
                }
            }
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::DeleteSelection).ok();
            }
        }))
    }

    fn delete_animation<S: Into<String>>(&self, name: S) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::DeleteAnimation(name.into()))
                    .ok();
            }
        }))
    }

    fn delete_frame<P: Into<PathBuf>>(&self, path: P) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::DeleteFrame(path.into()))
                    .ok();
            }
        }))
    }

    fn delete_hitbox<S: Into<String>>(&self, name: S) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::DeleteHitbox(name.into()))
                    .ok();
            }
        }))
    }

    fn delete_selected_animations(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::DeleteSelectedAnimations)
                    .ok();
            }
        }))
    }

    fn delete_selected_frames(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::DeleteSelectedFrames).ok();
            }
        }))
    }

    fn delete_selected_hitboxes(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::DeleteSelectedHitboxes)
                    .ok();
            }
        }))
    }

    fn delete_selected_keyframes(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::DeleteSelectedKeyframes)
                    .ok();
            }
        }))
    }

    fn delete_selection(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::DeleteSelection).ok();
            }
        }))
    }

    fn disable_sprite_darkening(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::DisableSpriteDarkening)
                    .ok();
            }
        }))
    }

    fn drop_frame_on_timeline(&self, direction: dto::Direction, index: usize) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::DropFrameOnTimeline(direction.into(), index))
                    .ok();
            }
        }))
    }

    fn drop_keyframe_on_timeline(
        &self,
        direction: dto::Direction,
        index: usize,
    ) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::DropKeyframeOnTimeline(direction.into(), index))
                    .ok();
            }
        }))
    }

    fn edit_animation<S: Into<String>>(&self, name: S) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::EditAnimation(name.into()))
                    .ok();
            }
        }))
    }

    fn enable_sprite_darkening(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::EnableSpriteDarkening)
                    .ok();
            }
        }))
    }

    fn end_drag_and_drop_frame(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::EndDragAndDropFrame).ok();
            }
        }))
    }

    fn end_drag_and_drop_keyframe(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::EndDragAndDropKeyframe)
                    .ok();
            }
        }))
    }

    fn end_drag_keyframe_duration(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::EndDragKeyframeDuration)
                    .ok();
            }
        }))
    }

    async fn end_export_as(&self) -> Result<Patch, ()> {
        let mut patch = self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::EndExportAs).ok();
            }
        });

        let (sheet, document_name) = {
            let state_handle = self.state();
            let state = state_handle.lock();
            match state.current_document() {
                Some(d) => (d.sheet().clone(), d.path().to_file_name()),
                _ => return Ok(patch),
            }
        };

        let result = tauri::async_runtime::spawn_blocking({
            let texture_cache = self.texture_cache();
            move || export_sheet(&sheet, texture_cache)
        })
        .await
        .unwrap();

        let mut additional_patch = self.state().mutate(StateTrim::Full, |state| {
            if let Err(e) = result {
                state.show_error_message(
                    "Export Error".to_owned(),
                    format!(
                        "An error occured while trying to export `{}`",
                        document_name.to_file_name(),
                    ),
                    e.to_string(),
                );
            }
        });

        patch.0.append(&mut additional_patch.0);
        Ok(patch)
    }

    fn end_nudge_hitbox(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::EndNudgeHitbox).ok();
            }
        }))
    }

    fn end_nudge_keyframe(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::EndNudgeKeyframe).ok();
            }
        }))
    }

    fn end_relocate_frames(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::EndRelocateFrames).ok();
            }
        }))
    }

    fn end_rename_animation<S: Into<String>>(&self, new_name: S) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::EndRenameAnimation(new_name.into()))
                    .ok();
            }
        }))
    }

    fn end_rename_hitbox<S: Into<String>>(&self, new_name: S) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::EndRenameHitbox(new_name.into()))
                    .ok();
            }
        }))
    }

    fn end_resize_hitbox(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::EndResizeHitbox).ok();
            }
        }))
    }

    async fn export(&self) -> Result<Patch, ()> {
        let (sheet, document_name) = {
            let state_handle = self.state();
            let state = state_handle.lock();
            match state.current_document() {
                Some(d) => (d.sheet().clone(), d.path().to_file_name()),
                _ => return Ok(Patch(Vec::new())),
            }
        };

        match tauri::async_runtime::spawn_blocking({
            let texture_cache = self.texture_cache();
            move || export_sheet(&sheet, texture_cache)
        })
        .await
        .unwrap()
        {
            Ok(_) => Ok(Patch(Vec::new())),
            Err(e) => Ok(self.state().mutate(StateTrim::Full, |state| {
                state.show_error_message(
                    "Export Error".to_owned(),
                    format!(
                        "An error occured while trying to export `{}`",
                        document_name.to_file_name(),
                    ),
                    e.to_string(),
                )
            })),
        }
    }

    fn filter_animations<S: Into<String>>(&self, search_query: S) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::FilterAnimations(search_query.into()))
                    .ok();
            }
        }))
    }

    fn filter_frames<S: Into<String>>(&self, search_query: S) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::FilterFrames(search_query.into()))
                    .ok();
            }
        }))
    }

    fn focus_document<P: AsRef<Path>>(&self, path: P) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            state.focus_document(path.as_ref()).ok();
        }))
    }

    fn get_state(&self) -> Result<dto::State, ()> {
        Ok(self.state().lock().to_dto(StateTrim::Full))
    }

    fn hide_hitboxes(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::HideHitboxes).ok();
            }
        }))
    }

    fn hide_origin(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::HideOrigin).ok();
            }
        }))
    }

    fn hide_sprite(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::HideSprite).ok();
            }
        }))
    }

    fn import_frames<P: Into<PathBuf>>(&self, paths: Vec<P>) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::ImportFrames(
                        paths.into_iter().map(|p| p.into()).collect(),
                    ))
                    .ok();
            }
        }))
    }

    fn jump_to_animation_end(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::JumpToAnimationEnd).ok();
            }
        }))
    }

    fn jump_to_animation_start(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::JumpToAnimationStart).ok();
            }
        }))
    }

    fn jump_to_next_frame(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::JumpToNextFrame).ok();
            }
        }))
    }

    fn jump_to_previous_frame(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::JumpToPreviousFrame).ok();
            }
        }))
    }

    fn lock_hitboxes(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::LockHitboxes).ok();
            }
        }))
    }

    fn new_document<P: Into<PathBuf>>(&self, path: P) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            state.new_document(path.into());
        }))
    }

    fn nudge_selection(
        &self,
        direction: dto::NudgeDirection,
        large_nudge: bool,
    ) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::NudgeSelection(direction.into(), large_nudge))
                    .ok();
            }
        }))
    }

    async fn open_documents<P: Into<PathBuf> + Send + Sync>(
        &self,
        paths: Vec<P>,
    ) -> Result<Patch, ()> {
        let mut documents: Vec<(PathBuf, DocumentResult<Document>)> = Vec::new();
        for path in paths {
            let open_path: PathBuf = path.into();
            documents.push((
                open_path.clone(),
                tauri::async_runtime::spawn_blocking(move || Document::open(open_path))
                    .await
                    .unwrap(),
            ));
        }

        Ok(self.state().mutate(StateTrim::Full, |state| {
            for document in documents {
                match document {
                    (_, Ok(d)) => {
                        state.open_document(d);
                    }
                    (path, Err(e)) => {
                        state.show_error_message(
                            "Error".to_owned(),
                            format!(
                                "An error occured while trying to open `{}`",
                                path.to_file_name()
                            ),
                            e.to_string(),
                        );
                    }
                }
            }
        }))
    }

    fn pan(&self, delta: (f32, f32)) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::OnlyWorkbench, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::Pan(delta.into())).ok();
            }
        }))
    }

    fn pan_timeline(&self, delta: f32) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::OnlyWorkbench, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::PanTimeline(delta)).ok();
            }
        }))
    }

    fn paste(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(serialized) = self.read_clipboard() {
                if let Ok(data) = serde_json::from_str(&serialized) {
                    if let Some(document) = state.current_document_mut() {
                        document.process_command(Command::Paste(data)).ok();
                    }
                }
            }
        }))
    }

    fn pause(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::Pause).ok();
            }
        }))
    }

    fn play(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::Play).ok();
            }
        }))
    }

    fn redo(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::Redo).ok();
            }
        }))
    }

    fn relocate_frame<P: Into<PathBuf>, Q: Into<PathBuf>>(
        &self,
        from: P,
        to: Q,
    ) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::RelocateFrame(from.into(), to.into()))
                    .ok();
            }
        }))
    }

    fn request_exit(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            state.request_exit();
            if state.should_exit() {
                self.close_window();
            }
        }))
    }

    fn reset_timeline_zoom(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::ResetTimelineZoom).ok();
            }
        }))
    }

    fn reset_workbench_zoom(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::ResetWorkbenchZoom).ok();
            }
        }))
    }

    async fn save(&self) -> Result<Patch, ()> {
        let documents_to_save: Vec<DocumentToSave> = {
            let state_handle = self.state();
            let state = state_handle.lock();
            let Some(document) = state.current_document() else {
                return Ok(Patch(Vec::new()))
            };
            vec![DocumentToSave {
                sheet: document.sheet().clone(),
                source: document.path().to_owned(),
                destination: document.path().to_owned(),
                version: document.version(),
            }]
        };
        save_documents(self, documents_to_save).await
    }

    async fn save_all(&self) -> Result<Patch, ()> {
        let documents_to_save: Vec<DocumentToSave> = {
            let state_handle = self.state();
            let state = state_handle.lock();
            state
                .documents_iter()
                .map(|d| DocumentToSave {
                    sheet: d.sheet().clone(),
                    source: d.path().to_owned(),
                    destination: d.path().to_owned(),
                    version: d.version(),
                })
                .collect()
        };
        save_documents(self, documents_to_save).await
    }

    async fn save_as<P: Into<PathBuf> + Send + Sync>(&self, new_path: P) -> Result<Patch, ()> {
        let documents_to_save: Vec<DocumentToSave> = {
            let state_handle = self.state();
            let state = state_handle.lock();
            let Some(document) = state.current_document() else {
                return Ok(Patch(Vec::new()))
            };
            vec![DocumentToSave {
                sheet: document.sheet().clone(),
                source: document.path().to_owned(),
                destination: new_path.into(),
                version: document.version(),
            }]
        };
        save_documents(self, documents_to_save).await
    }

    fn scrub_timeline(&self, time_millis: u64) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::ScrubTimeline(Duration::from_millis(time_millis)))
                    .ok();
            }
        }))
    }

    fn select_all(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::SelectAll).ok();
            }
        }))
    }

    fn select_animation<S: Into<String>>(
        &self,
        name: S,
        shift: bool,
        ctrl: bool,
    ) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SelectAnimation(name.into(), shift, ctrl))
                    .ok();
            }
        }))
    }

    fn select_direction(&self, direction: dto::Direction) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SelectDirection(direction.into()))
                    .ok();
            }
        }))
    }

    fn select_frame<P: Into<PathBuf>>(
        &self,
        path: P,
        shift: bool,
        ctrl: bool,
    ) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SelectFrame(path.into(), shift, ctrl))
                    .ok();
            }
        }))
    }

    fn select_hitbox<S: Into<String>>(
        &self,
        name: S,
        shift: bool,
        ctrl: bool,
    ) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SelectHitbox(name.into(), shift, ctrl))
                    .ok();
            }
        }))
    }

    fn select_keyframe(
        &self,
        direction: dto::Direction,
        index: usize,
        shift: bool,
        ctrl: bool,
    ) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SelectKeyframe(
                        direction.into(),
                        index,
                        shift,
                        ctrl,
                    ))
                    .ok();
            }
        }))
    }

    fn set_animation_looping(&self, is_looping: bool) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetAnimationLooping(is_looping))
                    .ok();
            }
        }))
    }

    fn set_animations_list_offset(&self, offset: u32) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetAnimationsListOffset(offset))
                    .ok();
            }
        }))
    }

    fn set_export_atlas_image_file<P: Into<PathBuf>>(&self, file: P) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetExportAtlasImageFile(file.into()))
                    .ok();
            }
        }))
    }

    fn set_export_metadata_file<P: Into<PathBuf>>(&self, file: P) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetExportMetadataFile(file.into()))
                    .ok();
            }
        }))
    }

    fn set_export_metadata_paths_root<P: Into<PathBuf>>(&self, directory: P) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetExportMetadataPathsRoot(directory.into()))
                    .ok();
            }
        }))
    }

    fn set_export_template_file<P: Into<PathBuf>>(&self, path: P) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetExportTemplateFile(path.into()))
                    .ok();
            }
        }))
    }

    fn set_frames_list_mode(&self, list_mode: dto::ListMode) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetFramesListMode(list_mode.into()))
                    .ok();
            }
        }))
    }

    fn set_frames_list_offset(&self, offset: u32) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetFramesListOffset(offset))
                    .ok();
            }
        }))
    }

    fn set_hitbox_height(&self, height: u32) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetHitboxHeight(height))
                    .ok();
            }
        }))
    }

    fn set_hitbox_position_x(&self, x: i32) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetHitboxPositionX(x))
                    .ok();
            }
        }))
    }

    fn set_hitbox_position_y(&self, y: i32) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetHitboxPositionY(y))
                    .ok();
            }
        }))
    }

    fn set_hitbox_width(&self, width: u32) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetHitboxWidth(width))
                    .ok();
            }
        }))
    }

    fn set_hitboxes_list_offset(&self, offset: u32) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetHitboxesListOffset(offset))
                    .ok();
            }
        }))
    }

    fn set_keyframe_duration(&self, duration_millis: u64) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetKeyframeDuration(Duration::from_millis(
                        duration_millis,
                    )))
                    .ok();
            }
        }))
    }

    fn set_keyframe_offset_x(&self, x: i32) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetKeyframeOffsetX(x))
                    .ok();
            }
        }))
    }

    fn set_keyframe_offset_y(&self, y: i32) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetKeyframeOffsetY(y))
                    .ok();
            }
        }))
    }

    fn set_keyframe_snapping_base_duration(&self, duration_millis: u64) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetKeyframeSnappingBaseDuration(
                        Duration::from_millis(duration_millis),
                    ))
                    .ok();
            }
        }))
    }

    fn set_snap_keyframe_durations(&self, snap: bool) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetSnapKeyframeDurations(snap))
                    .ok();
            }
        }))
    }

    fn set_snap_keyframes_to_multiples_of_duration(&self, snap: bool) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetSnapKeyframeToMultiplesOfDuration(snap))
                    .ok();
            }
        }))
    }

    fn set_snap_keyframes_to_other_keyframes(&self, snap: bool) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetSnapKeyframeToOtherKeyframes(snap))
                    .ok();
            }
        }))
    }

    fn set_timeline_offset(&self, offset_millis: f32) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::OnlyWorkbench, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetTimelineOffset(Duration::from_secs_f32(
                        offset_millis.max(0.0) / 1_000.0,
                    )))
                    .ok();
            }
        }))
    }

    fn set_timeline_zoom_amount(&self, amount: f32) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetTimelineZoomAmount(amount))
                    .ok();
            }
        }))
    }

    fn set_workbench_zoom_factor(&self, zoom_factor: u32) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetWorkbenchZoomFactor(zoom_factor))
                    .ok();
            }
        }))
    }

    fn show_error_message<S: Into<String>, T: Into<String>, U: Into<String>>(
        &self,
        title: S,
        summary: T,
        details: U,
    ) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            state.show_error_message(title.into(), summary.into(), details.into());
        }))
    }

    fn show_hitboxes(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::ShowHitboxes).ok();
            }
        }))
    }

    fn show_origin(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::ShowOrigin).ok();
            }
        }))
    }

    fn show_sprite(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::ShowSprite).ok();
            }
        }))
    }

    fn tick(&self, delta_time_millis: f64) -> Result<Patch, ()> {
        Ok(self
            .state()
            .mutate(StateTrim::OnlyCurrentDocument, |state| {
                if let Some(document) = state.current_document_mut() {
                    document
                        .process_command(Command::Tick(Duration::from_nanos(
                            (delta_time_millis * 1_000_000.0) as u64,
                        )))
                        .ok();
                }
            }))
    }

    fn toggle_preserve_aspect_ratio(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::TogglePreserveAspectRatio)
                    .ok();
            }
        }))
    }

    fn undo(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::Undo).ok();
            }
        }))
    }

    fn unlock_hitboxes(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::UnlockHitboxes).ok();
            }
        }))
    }

    fn update_drag_keyframe_duration(&self, delta_millis: i64) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::OnlyWorkbench, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::UpdateDragKeyframeDuration(delta_millis))
                    .ok();
            }
        }))
    }

    fn update_nudge_hitbox(&self, displacement: (i32, i32), both_axis: bool) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::OnlyWorkbench, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::UpdateNudgeHitbox(displacement.into(), both_axis))
                    .ok();
            }
        }))
    }

    fn update_nudge_keyframe(
        &self,
        displacement: (i32, i32),
        both_axis: bool,
    ) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::OnlyWorkbench, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::UpdateNudgeKeyframe(displacement.into(), both_axis))
                    .ok();
            }
        }))
    }

    fn update_resize_hitbox(
        &self,
        displacement: (i32, i32),
        preserve_aspect_ratio: bool,
    ) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::OnlyWorkbench, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::UpdateResizeHitbox(
                        displacement.into(),
                        preserve_aspect_ratio,
                    ))
                    .ok();
            }
        }))
    }

    fn zoom_in_timeline(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::ZoomInTimeline).ok();
            }
        }))
    }

    fn zoom_in_timeline_around(&self, fixed_point: f32) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::ZoomInTimelineAround(Duration::from_secs_f32(
                        fixed_point.max(0.0) / 1_000.0,
                    )))
                    .ok();
            }
        }))
    }

    fn zoom_in_workbench(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::ZoomInWorkbench).ok();
            }
        }))
    }

    fn zoom_in_workbench_around(&self, fixed_point: (f32, f32)) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::ZoomInWorkbenchAround(fixed_point.into()))
                    .ok();
            }
        }))
    }

    fn zoom_out_timeline(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::ZoomOutTimeline).ok();
            }
        }))
    }

    fn zoom_out_timeline_around(&self, fixed_point: f32) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::ZoomOutTimelineAround(Duration::from_secs_f32(
                        fixed_point.max(0.0) / 1_000.0,
                    )))
                    .ok();
            }
        }))
    }

    fn zoom_out_workbench(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::ZoomOutWorkbench).ok();
            }
        }))
    }

    fn zoom_out_workbench_around(&self, fixed_point: (f32, f32)) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::ZoomOutWorkbenchAround(fixed_point.into()))
                    .ok();
            }
        }))
    }
}

async fn save_documents<A: TigerApp>(
    app: &A,
    mut documents: Vec<DocumentToSave>,
) -> Result<Patch, ()> {
    let mut work = Vec::new();
    for document in &mut documents {
        let sheet = std::mem::take(&mut document.sheet);
        let write_destination = document.destination.clone();
        work.push(tauri::async_runtime::spawn_blocking(move || {
            sheet.write(&write_destination)
        }));
    }
    let results = futures::future::join_all(work)
        .await
        .into_iter()
        .map(|r| r.unwrap());

    Ok(app.state().mutate(StateTrim::Full, |state| {
        for (document, result) in documents.iter().zip(results) {
            match result {
                Ok(_) => {
                    state.relocate_document(&document.source, &document.destination);
                    if let Some(d) = state.document_mut(&document.destination) {
                        d.mark_as_saved(document.version);
                    }
                }
                Err(e) => state.show_error_message(
                    "Error".to_owned(),
                    format!(
                        "An error occured while trying to save `{}`",
                        document.destination.to_file_name()
                    ),
                    e.to_string(),
                ),
            }
        }

        state.advance_exit();
        if state.should_exit() {
            app.close_window();
        }
    }))
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
