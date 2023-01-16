use async_trait::async_trait;
use json_patch::Patch;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::app::TigerApp;
use crate::document::{Command, Document, DocumentResult};
use crate::dto::{self, StateTrim, ToFileName};
use crate::export::export_sheet;
use crate::sheet::{Absolute, Sheet};

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
        Ok(self.patch(StateTrim::Full, |state| {
            state.acknowledge_error();
        }))
    }

    fn apply_direction_preset(&self, preset: dto::DirectionPreset) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::ApplyDirectionPreset(preset.into()))
                    .ok();
            }
        }))
    }

    fn begin_drag_and_drop_frame<P: Into<PathBuf>>(&self, frame: P) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
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
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::BeginDragAndDropKeyframe {
                        direction: direction.into(),
                        index,
                    })
                    .ok();
            }
        }))
    }

    fn begin_drag_keyframe_duration(
        &self,
        direction: dto::Direction,
        index: usize,
    ) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::BeginDragKeyframeDuration {
                        direction: direction.into(),
                        index,
                    })
                    .ok();
            }
        }))
    }

    fn begin_export_as(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::BeginExportAs).ok();
            }
        }))
    }

    fn begin_nudge_hitbox<S: Into<String>>(&self, name: S) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::BeginNudgeHitbox(name.into()))
                    .ok();
            }
        }))
    }

    fn begin_nudge_keyframe(&self, direction: dto::Direction, index: usize) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::BeginNudgeKeyframe {
                        direction: direction.into(),
                        index,
                    })
                    .ok();
            }
        }))
    }

    fn begin_relocate_frames(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::BeginRelocateFrames).ok();
            }
        }))
    }

    fn begin_rename_animation<S: Into<String>>(&self, animation_name: S) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::BeginRenameAnimation(animation_name.into()))
                    .ok();
            }
        }))
    }

    fn begin_rename_hitbox<S: Into<String>>(&self, hitbox_name: S) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::BeginRenameHitbox(hitbox_name.into()))
                    .ok();
            }
        }))
    }

    fn begin_rename_selection(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
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
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::BeginResizeHitbox {
                        name: name.into(),
                        axis: axis.into(),
                    })
                    .ok();
            }
        }))
    }

    fn browse_selection(&self, direction: dto::BrowseDirection, shift: bool) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::BrowseSelection {
                        direction: direction.into(),
                        shift,
                    })
                    .ok();
            }
        }))
    }

    fn browse_to_end(&self, shift: bool) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::BrowseToEnd(shift)).ok();
            }
        }))
    }

    fn browse_to_start(&self, shift: bool) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::BrowseToStart(shift)).ok();
            }
        }))
    }

    fn cancel_exit(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            state.cancel_exit();
        }))
    }

    fn cancel_export_as(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::CancelExportAs).ok();
            }
        }))
    }

    fn cancel_relocate_frames(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::CancelRelocateFrames).ok();
            }
        }))
    }

    fn cancel_rename(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::CancelRename).ok();
            }
        }))
    }

    fn center_workbench(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::CenterWorkbench).ok();
            }
        }))
    }

    fn clear_selection(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::ClearSelection).ok();
            }
        }))
    }

    fn close_all_documents(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
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
        Ok(self.patch(StateTrim::Full, |state| {
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
        Ok(self.patch(StateTrim::Full, |state| {
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
        Ok(self.patch(StateTrim::Full, |state| {
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
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(data) = state.current_document().and_then(|d| d.copy()) {
                if let Ok(serialized) = serde_json::to_string(&data) {
                    self.write_clipboard(serialized);
                    state.set_clipboard_manifest(Some(data.manifest()));
                }
            }
        }))
    }

    fn create_animation(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::CreateAnimation).ok();
            }
        }))
    }

    fn create_hitbox(&self, position: Option<(i32, i32)>) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::CreateHitbox(position.map(|p| p.into())))
                    .ok();
            }
        }))
    }

    fn cut(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
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
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::DeleteAnimation(name.into()))
                    .ok();
            }
        }))
    }

    fn delete_frame<P: Into<PathBuf>>(&self, path: P) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::DeleteFrame(path.into()))
                    .ok();
            }
        }))
    }

    fn delete_hitbox<S: Into<String>>(&self, name: S) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::DeleteHitbox(name.into()))
                    .ok();
            }
        }))
    }

    fn delete_selected_animations(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::DeleteSelectedAnimations)
                    .ok();
            }
        }))
    }

    fn delete_selected_frames(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::DeleteSelectedFrames).ok();
            }
        }))
    }

    fn delete_selected_hitboxes(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::DeleteSelectedHitboxes)
                    .ok();
            }
        }))
    }

    fn delete_selected_keyframes(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::DeleteSelectedKeyframes)
                    .ok();
            }
        }))
    }

    fn delete_selection(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::DeleteSelection).ok();
            }
        }))
    }

    fn disable_sprite_darkening(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::DisableSpriteDarkening)
                    .ok();
            }
        }))
    }

    fn drop_frame_on_timeline(&self, direction: dto::Direction, index: usize) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::DropFrameOnTimeline {
                        direction: direction.into(),
                        index,
                    })
                    .ok();
            }
        }))
    }

    fn drop_keyframe_on_timeline(
        &self,
        direction: dto::Direction,
        index: usize,
    ) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::DropKeyframeOnTimeline {
                        direction: direction.into(),
                        index,
                    })
                    .ok();
            }
        }))
    }

    fn edit_animation<S: Into<String>>(&self, name: S) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::EditAnimation(name.into()))
                    .ok();
            }
        }))
    }

    fn enable_sprite_darkening(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::EnableSpriteDarkening)
                    .ok();
            }
        }))
    }

    fn end_drag_and_drop_frame(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::EndDragAndDropFrame).ok();
            }
        }))
    }

    fn end_drag_and_drop_keyframe(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::EndDragAndDropKeyframe)
                    .ok();
            }
        }))
    }

    fn end_drag_keyframe_duration(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::EndDragKeyframeDuration)
                    .ok();
            }
        }))
    }

    async fn end_export_as(&self) -> Result<Patch, ()> {
        let mut patch = self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::EndExportAs).ok();
            }
        });

        let mut additional_patch = export_document(self).await;
        patch.0.append(&mut additional_patch.0);
        Ok(patch)
    }

    fn end_nudge_hitbox(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::EndNudgeHitbox).ok();
            }
        }))
    }

    fn end_nudge_keyframe(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::EndNudgeKeyframe).ok();
            }
        }))
    }

    fn end_relocate_frames(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::EndRelocateFrames).ok();
            }
        }))
    }

    fn end_rename_animation<S: Into<String>>(&self, new_name: S) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::EndRenameAnimation(new_name.into()))
                    .ok();
            }
        }))
    }

    fn end_rename_hitbox<S: Into<String>>(&self, new_name: S) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::EndRenameHitbox(new_name.into()))
                    .ok();
            }
        }))
    }

    fn end_resize_hitbox(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::EndResizeHitbox).ok();
            }
        }))
    }

    async fn export(&self) -> Result<Patch, ()> {
        let has_export_settings = {
            let state_handle = self.state();
            let state = state_handle.lock();
            state
                .current_document()
                .map(|d| d.sheet().export_settings().is_some())
                .unwrap_or_default()
        };
        if has_export_settings {
            Ok(export_document(self).await)
        } else {
            self.begin_export_as()
        }
    }

    fn filter_animations<S: Into<String>>(&self, search_query: S) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::FilterAnimations(search_query.into()))
                    .ok();
            }
        }))
    }

    fn filter_frames<S: Into<String>>(&self, search_query: S) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::FilterFrames(search_query.into()))
                    .ok();
            }
        }))
    }

    fn focus_document<P: AsRef<Path>>(&self, path: P) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            state.focus_document(path.as_ref()).ok();
        }))
    }

    fn get_state(&self) -> Result<dto::State, ()> {
        Ok(self.state().lock().to_dto(StateTrim::Full))
    }

    fn hide_hitboxes(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::HideHitboxes).ok();
            }
        }))
    }

    fn hide_origin(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::HideOrigin).ok();
            }
        }))
    }

    fn hide_sprite(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::HideSprite).ok();
            }
        }))
    }

    fn import_frames<P: Into<PathBuf>>(&self, paths: Vec<P>) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
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
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::JumpToAnimationEnd).ok();
            }
        }))
    }

    fn jump_to_animation_start(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::JumpToAnimationStart).ok();
            }
        }))
    }

    fn jump_to_next_frame(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::JumpToNextFrame).ok();
            }
        }))
    }

    fn jump_to_previous_frame(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::JumpToPreviousFrame).ok();
            }
        }))
    }

    fn lock_hitboxes(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::LockHitboxes).ok();
            }
        }))
    }

    fn new_document<P: Into<PathBuf>>(&self, path: P) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            state.new_document(path.into());
        }))
    }

    fn nudge_selection(
        &self,
        direction: dto::NudgeDirection,
        large_nudge: bool,
    ) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::NudgeSelection {
                        direction: direction.into(),
                        large_nudge,
                    })
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

        Ok(self.patch(StateTrim::Full, |state| {
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
        Ok(self.patch(StateTrim::OnlyWorkbench, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::Pan(delta.into())).ok();
            }
        }))
    }

    fn pan_timeline(&self, delta: f32) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::OnlyWorkbench, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::PanTimeline(delta)).ok();
            }
        }))
    }

    fn paste(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
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
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::Pause).ok();
            }
        }))
    }

    fn play(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::Play).ok();
            }
        }))
    }

    fn redo(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
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
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::RelocateFrame {
                        from: from.into(),
                        to: to.into(),
                    })
                    .ok();
            }
        }))
    }

    fn request_exit(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            state.request_exit();
            if state.should_exit() {
                self.close_window();
            }
        }))
    }

    fn reset_timeline_zoom(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::ResetTimelineZoom).ok();
            }
        }))
    }

    fn reset_workbench_zoom(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
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
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::ScrubTimeline(Duration::from_millis(time_millis)))
                    .ok();
            }
        }))
    }

    fn select_all(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
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
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SelectAnimation {
                        name: name.into(),
                        shift,
                        ctrl,
                    })
                    .ok();
            }
        }))
    }

    fn select_direction(&self, direction: dto::Direction) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
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
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SelectFrame {
                        path: path.into(),
                        shift,
                        ctrl,
                    })
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
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SelectHitbox {
                        name: name.into(),
                        shift,
                        ctrl,
                    })
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
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SelectKeyframe {
                        direction: direction.into(),
                        index,
                        shift,
                        ctrl,
                    })
                    .ok();
            }
        }))
    }

    fn set_animation_looping(&self, is_looping: bool) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetAnimationLooping(is_looping))
                    .ok();
            }
        }))
    }

    fn set_animations_list_offset(&self, offset: u32) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::OnlyWorkbench, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetAnimationsListOffset(offset))
                    .ok();
            }
        }))
    }

    fn set_export_atlas_image_file<P: Into<PathBuf>>(&self, file: P) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetExportAtlasImageFile(file.into()))
                    .ok();
            }
        }))
    }

    fn set_export_metadata_file<P: Into<PathBuf>>(&self, file: P) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetExportMetadataFile(file.into()))
                    .ok();
            }
        }))
    }

    fn set_export_metadata_paths_root<P: Into<PathBuf>>(&self, directory: P) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetExportMetadataPathsRoot(directory.into()))
                    .ok();
            }
        }))
    }

    fn set_export_template_file<P: Into<PathBuf>>(&self, path: P) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetExportTemplateFile(path.into()))
                    .ok();
            }
        }))
    }

    fn set_frames_list_mode(&self, list_mode: dto::ListMode) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetFramesListMode(list_mode.into()))
                    .ok();
            }
        }))
    }

    fn set_frames_list_offset(&self, offset: u32) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::OnlyWorkbench, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetFramesListOffset(offset))
                    .ok();
            }
        }))
    }

    fn set_hitbox_height(&self, height: u32) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetHitboxHeight(height))
                    .ok();
            }
        }))
    }

    fn set_hitbox_position_x(&self, x: i32) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetHitboxPositionX(x))
                    .ok();
            }
        }))
    }

    fn set_hitbox_position_y(&self, y: i32) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetHitboxPositionY(y))
                    .ok();
            }
        }))
    }

    fn set_hitbox_width(&self, width: u32) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetHitboxWidth(width))
                    .ok();
            }
        }))
    }

    fn set_hitboxes_list_offset(&self, offset: u32) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::OnlyWorkbench, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetHitboxesListOffset(offset))
                    .ok();
            }
        }))
    }

    fn set_keyframe_duration(&self, duration_millis: u64) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
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
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetKeyframeOffsetX(x))
                    .ok();
            }
        }))
    }

    fn set_keyframe_offset_y(&self, y: i32) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetKeyframeOffsetY(y))
                    .ok();
            }
        }))
    }

    fn set_keyframe_snapping_base_duration(&self, duration_millis: u64) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
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
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetSnapKeyframeDurations(snap))
                    .ok();
            }
        }))
    }

    fn set_snap_keyframes_to_multiples_of_duration(&self, snap: bool) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetSnapKeyframeToMultiplesOfDuration(snap))
                    .ok();
            }
        }))
    }

    fn set_snap_keyframes_to_other_keyframes(&self, snap: bool) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetSnapKeyframeToOtherKeyframes(snap))
                    .ok();
            }
        }))
    }

    fn set_timeline_offset(&self, offset_millis: f32) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::OnlyWorkbench, |state| {
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
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetTimelineZoomAmount(amount))
                    .ok();
            }
        }))
    }

    fn set_workbench_zoom_factor(&self, zoom_factor: u32) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
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
        Ok(self.patch(StateTrim::Full, |state| {
            state.show_error_message(title.into(), summary.into(), details.into());
        }))
    }

    fn show_hitboxes(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::ShowHitboxes).ok();
            }
        }))
    }

    fn show_origin(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::ShowOrigin).ok();
            }
        }))
    }

    fn show_sprite(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::ShowSprite).ok();
            }
        }))
    }

    fn tick(&self, delta_time_millis: f64) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::OnlyCurrentDocument, |state| {
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
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::TogglePreserveAspectRatio)
                    .ok();
            }
        }))
    }

    fn undo(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::Undo).ok();
            }
        }))
    }

    fn unlock_hitboxes(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::UnlockHitboxes).ok();
            }
        }))
    }

    fn update_drag_keyframe_duration(&self, delta_millis: i64) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::OnlyWorkbench, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::UpdateDragKeyframeDuration(delta_millis))
                    .ok();
            }
        }))
    }

    fn update_nudge_hitbox(&self, displacement: (i32, i32), both_axis: bool) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::OnlyWorkbench, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::UpdateNudgeHitbox {
                        displacement: displacement.into(),
                        both_axis,
                    })
                    .ok();
            }
        }))
    }

    fn update_nudge_keyframe(
        &self,
        displacement: (i32, i32),
        both_axis: bool,
    ) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::OnlyWorkbench, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::UpdateNudgeKeyframe {
                        displacement: displacement.into(),
                        both_axis,
                    })
                    .ok();
            }
        }))
    }

    fn update_resize_hitbox(
        &self,
        displacement: (i32, i32),
        preserve_aspect_ratio: bool,
    ) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::OnlyWorkbench, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::UpdateResizeHitbox {
                        displacement: displacement.into(),
                        preserve_aspect_ratio,
                    })
                    .ok();
            }
        }))
    }

    fn zoom_in_timeline(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::ZoomInTimeline).ok();
            }
        }))
    }

    fn zoom_in_timeline_around(&self, fixed_point: f32) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
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
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::ZoomInWorkbench).ok();
            }
        }))
    }

    fn zoom_in_workbench_around(&self, fixed_point: (f32, f32)) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::ZoomInWorkbenchAround(fixed_point.into()))
                    .ok();
            }
        }))
    }

    fn zoom_out_timeline(&self) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::ZoomOutTimeline).ok();
            }
        }))
    }

    fn zoom_out_timeline_around(&self, fixed_point: f32) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
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
        Ok(self.patch(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::ZoomOutWorkbench).ok();
            }
        }))
    }

    fn zoom_out_workbench_around(&self, fixed_point: (f32, f32)) -> Result<Patch, ()> {
        Ok(self.patch(StateTrim::Full, |state| {
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

    Ok(app.patch(StateTrim::Full, |state| {
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

async fn export_document<A: TigerApp>(app: &A) -> Patch {
    let (sheet, document_name) = {
        let state_handle = app.state();
        let state = state_handle.lock();
        match state.current_document() {
            Some(d) => (d.sheet().clone(), d.path().to_file_name()),
            _ => return Patch(Vec::new()),
        }
    };

    let result = tauri::async_runtime::spawn_blocking({
        let texture_cache = app.texture_cache();
        move || export_sheet(&sheet, texture_cache)
    })
    .await
    .unwrap();

    app.patch(StateTrim::Full, |state| {
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
    })
}
