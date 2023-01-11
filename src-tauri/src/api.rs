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
    fn begin_nudge_keyframe(&self, direction: dto::Direction, index: usize) -> Result<Patch, ()>;
    fn begin_relocate_frames(&self) -> Result<Patch, ()>;
    fn cancel_exit(&self) -> Result<Patch, ()>;
    fn cancel_relocate_frames(&self) -> Result<Patch, ()>;
    fn close_all_documents(&self) -> Result<Patch, ()>;
    fn close_current_document(&self) -> Result<Patch, ()>;
    fn close_document<P: AsRef<Path>>(&self, path: P) -> Result<Patch, ()>;
    fn close_without_saving(&self) -> Result<Patch, ()>;
    fn copy(&self) -> Result<Patch, ()>;
    fn create_animation(&self) -> Result<Patch, ()>;
    fn create_hitbox(&self, position: Option<(i32, i32)>) -> Result<Patch, ()>;
    fn cut(&self) -> Result<Patch, ()>;
    fn delete_frame<P: Into<PathBuf>>(&self, path: P) -> Result<Patch, ()>;
    fn delete_hitbox<S: Into<String>>(&self, name: S) -> Result<Patch, ()>;
    fn drop_frame_on_timeline(&self, direction: dto::Direction, index: usize) -> Result<Patch, ()>;
    fn drop_keyframe_on_timeline(
        &self,
        direction: dto::Direction,
        index: usize,
    ) -> Result<Patch, ()>;
    fn edit_animation<S: Into<String>>(&self, name: S) -> Result<Patch, ()>;
    fn end_drag_and_drop_frame(&self) -> Result<Patch, ()>;
    fn end_drag_and_drop_keyframe(&self) -> Result<Patch, ()>;
    fn end_drag_keyframe_duration(&self) -> Result<Patch, ()>;
    async fn end_export_as(&self) -> Result<Patch, ()>;
    fn end_nudge_keyframe(&self) -> Result<Patch, ()>;
    fn end_relocate_frames(&self) -> Result<Patch, ()>;
    async fn export(&self) -> Result<Patch, ()>;
    fn focus_document<P: AsRef<Path>>(&self, path: P) -> Result<Patch, ()>;
    fn import_frames<P: Into<PathBuf>>(&self, paths: Vec<P>) -> Result<Patch, ()>;
    fn new_document<P: Into<PathBuf>>(&self, path: P) -> Result<Patch, ()>;
    async fn open_documents<P: Into<PathBuf> + Send + Sync>(
        &self,
        paths: Vec<P>,
    ) -> Result<Patch, ()>;
    fn paste(&self) -> Result<Patch, ()>;
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
    fn select_animation<S: Into<String>>(
        &self,
        name: S,
        shift: bool,
        ctrl: bool,
    ) -> Result<Patch, ()>;
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
    fn set_export_metadata_file<P: Into<PathBuf>>(&self, file: P) -> Result<Patch, ()>;
    fn set_export_metadata_paths_root<P: Into<PathBuf>>(&self, file: P) -> Result<Patch, ()>;
    fn set_export_template_file<P: Into<PathBuf>>(&self, file: P) -> Result<Patch, ()>;
    fn set_export_texture_file<P: Into<PathBuf>>(&self, file: P) -> Result<Patch, ()>;
    fn set_hitbox_height(&self, height: u32) -> Result<Patch, ()>;
    fn set_hitbox_position_x(&self, x: i32) -> Result<Patch, ()>;
    fn set_hitbox_position_y(&self, y: i32) -> Result<Patch, ()>;
    fn set_hitbox_width(&self, width: u32) -> Result<Patch, ()>;
    fn set_keyframe_duration(&self, duration_millies: u64) -> Result<Patch, ()>;
    fn set_keyframe_offset_x(&self, x: i32) -> Result<Patch, ()>;
    fn set_keyframe_offset_y(&self, y: i32) -> Result<Patch, ()>;
    fn set_keyframe_snapping_base_duration(&self, duration_millis: u64) -> Result<Patch, ()>;
    fn set_snap_keyframe_durations(&self, snap: bool) -> Result<Patch, ()>;
    fn set_snap_keyframes_to_multiples_of_duration(&self, snap: bool) -> Result<Patch, ()>;
    fn set_snap_keyframes_to_other_keyframes(&self, snap: bool) -> Result<Patch, ()>;
    fn set_timeline_zoom_amount(&self, amount: f32) -> Result<Patch, ()>;
    fn set_workbench_zoom_factor(&self, zoom_factor: u32) -> Result<Patch, ()>;
    fn toggle_preserve_aspect_ratio(&self) -> Result<Patch, ()>;
    fn update_drag_keyframe_duration(&self, delta_millis: i64) -> Result<Patch, ()>;
    fn update_nudge_keyframe(&self, displacement: (i32, i32), both_axis: bool)
        -> Result<Patch, ()>;
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
impl<T: TigerApp + Sync> Api for T {
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

    fn cancel_exit(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            state.cancel_exit();
        }))
    }

    fn cancel_relocate_frames(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document.process_command(Command::CancelRelocateFrames).ok();
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

    fn focus_document<P: AsRef<Path>>(&self, path: P) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            state.focus_document(path.as_ref()).ok();
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

    fn new_document<P: Into<PathBuf>>(&self, path: P) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            state.new_document(path.into());
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

    fn set_export_texture_file<P: Into<PathBuf>>(&self, file: P) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::SetExportTextureFile(file.into()))
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

    fn toggle_preserve_aspect_ratio(&self) -> Result<Patch, ()> {
        Ok(self.state().mutate(StateTrim::Full, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::TogglePreserveAspectRatio)
                    .ok();
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
pub fn get_state(state_handle: tauri::State<'_, state::Handle>) -> Result<dto::State, ()> {
    let state = state_handle.lock();
    Ok(state.to_dto(StateTrim::Full))
}

#[tauri::command]
pub fn show_error_message(
    state_handle: tauri::State<'_, state::Handle>,
    title: String,
    summary: String,
    details: String,
) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        state.show_error_message(title, summary, details);
    }))
}

#[tauri::command]
pub fn acknowledge_error(state_handle: tauri::State<'_, state::Handle>) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        state.acknowledge_error();
    }))
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
pub fn undo(state_handle: tauri::State<'_, state::Handle>) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document.process_command(Command::Undo).ok();
        }
    }))
}

#[tauri::command]
pub fn redo(state_handle: tauri::State<'_, state::Handle>) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document.process_command(Command::Redo).ok();
        }
    }))
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
pub fn set_frames_list_mode(
    state_handle: tauri::State<'_, state::Handle>,
    list_mode: dto::ListMode,
) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document
                .process_command(Command::SetFramesListMode(list_mode.into()))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn set_frames_list_offset(
    state_handle: tauri::State<'_, state::Handle>,
    offset: u32,
) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document
                .process_command(Command::SetFramesListOffset(offset))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn set_hitboxes_list_offset(
    state_handle: tauri::State<'_, state::Handle>,
    offset: u32,
) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document
                .process_command(Command::SetHitboxesListOffset(offset))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn filter_frames(
    state_handle: tauri::State<'_, state::Handle>,
    search_query: String,
) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document
                .process_command(Command::FilterFrames(search_query))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn filter_animations(
    state_handle: tauri::State<'_, state::Handle>,
    search_query: String,
) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document
                .process_command(Command::FilterAnimations(search_query))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn set_animations_list_offset(
    state_handle: tauri::State<'_, state::Handle>,
    offset: u32,
) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document
                .process_command(Command::SetAnimationsListOffset(offset))
                .ok();
        }
    }))
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
pub fn delete_selected_frames(state_handle: tauri::State<'_, state::Handle>) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document.process_command(Command::DeleteSelectedFrames).ok();
        }
    }))
}

#[tauri::command]
pub fn delete_selection(state_handle: tauri::State<'_, state::Handle>) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document.process_command(Command::DeleteSelection).ok();
        }
    }))
}

#[tauri::command]
pub fn nudge_selection(
    state_handle: tauri::State<'_, state::Handle>,
    direction: dto::NudgeDirection,
    large_nudge: bool,
) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document
                .process_command(Command::NudgeSelection(direction.into(), large_nudge))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn browse_selection(
    state_handle: tauri::State<'_, state::Handle>,
    direction: dto::BrowseDirection,
    shift: bool,
) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document
                .process_command(Command::BrowseSelection(direction.into(), shift))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn browse_to_end(
    state_handle: tauri::State<'_, state::Handle>,
    shift: bool,
) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document.process_command(Command::BrowseToEnd(shift)).ok();
        }
    }))
}

#[tauri::command]
pub fn browse_to_start(
    state_handle: tauri::State<'_, state::Handle>,
    shift: bool,
) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document.process_command(Command::BrowseToStart(shift)).ok();
        }
    }))
}

#[tauri::command]
pub fn clear_selection(state_handle: tauri::State<'_, state::Handle>) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document.process_command(Command::ClearSelection).ok();
        }
    }))
}

#[tauri::command]
pub fn select_all(state_handle: tauri::State<'_, state::Handle>) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document.process_command(Command::SelectAll).ok();
        }
    }))
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
pub fn pan(state_handle: tauri::State<'_, state::Handle>, delta: (f32, f32)) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::OnlyWorkbench, |state| {
        if let Some(document) = state.current_document_mut() {
            document.process_command(Command::Pan(delta.into())).ok();
        }
    }))
}

#[tauri::command]
pub fn center_workbench(state_handle: tauri::State<'_, state::Handle>) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document.process_command(Command::CenterWorkbench).ok();
        }
    }))
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
pub fn enable_sprite_darkening(state_handle: tauri::State<'_, state::Handle>) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document
                .process_command(Command::EnableSpriteDarkening)
                .ok();
        }
    }))
}

#[tauri::command]
pub fn disable_sprite_darkening(
    state_handle: tauri::State<'_, state::Handle>,
) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document
                .process_command(Command::DisableSpriteDarkening)
                .ok();
        }
    }))
}

#[tauri::command]
pub fn hide_sprite(state_handle: tauri::State<'_, state::Handle>) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document.process_command(Command::HideSprite).ok();
        }
    }))
}

#[tauri::command]
pub fn show_sprite(state_handle: tauri::State<'_, state::Handle>) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document.process_command(Command::ShowSprite).ok();
        }
    }))
}

#[tauri::command]
pub fn hide_hitboxes(state_handle: tauri::State<'_, state::Handle>) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document.process_command(Command::HideHitboxes).ok();
        }
    }))
}

#[tauri::command]
pub fn show_hitboxes(state_handle: tauri::State<'_, state::Handle>) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document.process_command(Command::ShowHitboxes).ok();
        }
    }))
}

#[tauri::command]
pub fn hide_origin(state_handle: tauri::State<'_, state::Handle>) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document.process_command(Command::HideOrigin).ok();
        }
    }))
}

#[tauri::command]
pub fn show_origin(state_handle: tauri::State<'_, state::Handle>) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document.process_command(Command::ShowOrigin).ok();
        }
    }))
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
pub fn begin_rename_animation(
    state_handle: tauri::State<'_, state::Handle>,
    animation_name: String,
) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document
                .process_command(Command::BeginRenameAnimation(animation_name))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn begin_rename_hitbox(
    state_handle: tauri::State<'_, state::Handle>,
    hitbox_name: String,
) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document
                .process_command(Command::BeginRenameHitbox(hitbox_name))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn begin_rename_selection(state_handle: tauri::State<'_, state::Handle>) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document.process_command(Command::BeginRenameSelection).ok();
        }
    }))
}

#[tauri::command]
pub fn cancel_rename(state_handle: tauri::State<'_, state::Handle>) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document.process_command(Command::CancelRename).ok();
        }
    }))
}

#[tauri::command]
pub fn end_rename_animation(
    state_handle: tauri::State<'_, state::Handle>,
    new_name: String,
) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document
                .process_command(Command::EndRenameAnimation(new_name))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn end_rename_hitbox(
    state_handle: tauri::State<'_, state::Handle>,
    new_name: String,
) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document
                .process_command(Command::EndRenameHitbox(new_name))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn delete_animation(
    state_handle: tauri::State<'_, state::Handle>,
    name: String,
) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document
                .process_command(Command::DeleteAnimation(name))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn delete_selected_animations(
    state_handle: tauri::State<'_, state::Handle>,
) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document
                .process_command(Command::DeleteSelectedAnimations)
                .ok();
        }
    }))
}

#[tauri::command]
pub fn tick(
    state_handle: tauri::State<'_, state::Handle>,
    delta_time_millis: f64,
) -> Result<Patch, ()> {
    Ok(
        state_handle.mutate(StateTrim::OnlyCurrentDocument, |state| {
            if let Some(document) = state.current_document_mut() {
                document
                    .process_command(Command::Tick(Duration::from_nanos(
                        (delta_time_millis * 1_000_000.0) as u64,
                    )))
                    .ok();
            }
        }),
    )
}

#[tauri::command]
pub fn play(state_handle: tauri::State<'_, state::Handle>) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document.process_command(Command::Play).ok();
        }
    }))
}

#[tauri::command]
pub fn pause(state_handle: tauri::State<'_, state::Handle>) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document.process_command(Command::Pause).ok();
        }
    }))
}

#[tauri::command]
pub fn scrub_timeline(
    state_handle: tauri::State<'_, state::Handle>,
    time_millis: u64,
) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document
                .process_command(Command::ScrubTimeline(Duration::from_millis(time_millis)))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn jump_to_animation_start(state_handle: tauri::State<'_, state::Handle>) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document.process_command(Command::JumpToAnimationStart).ok();
        }
    }))
}

#[tauri::command]
pub fn jump_to_animation_end(state_handle: tauri::State<'_, state::Handle>) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document.process_command(Command::JumpToAnimationEnd).ok();
        }
    }))
}

#[tauri::command]
pub fn jump_to_previous_frame(state_handle: tauri::State<'_, state::Handle>) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document.process_command(Command::JumpToPreviousFrame).ok();
        }
    }))
}

#[tauri::command]
pub fn jump_to_next_frame(state_handle: tauri::State<'_, state::Handle>) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document.process_command(Command::JumpToNextFrame).ok();
        }
    }))
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
pub fn set_timeline_offset(
    state_handle: tauri::State<'_, state::Handle>,
    offset_millis: f32,
) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::OnlyWorkbench, |state| {
        if let Some(document) = state.current_document_mut() {
            document
                .process_command(Command::SetTimelineOffset(Duration::from_secs_f32(
                    offset_millis.max(0.0) / 1_000.0,
                )))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn pan_timeline(
    state_handle: tauri::State<'_, state::Handle>,
    delta: f32,
) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::OnlyWorkbench, |state| {
        if let Some(document) = state.current_document_mut() {
            document.process_command(Command::PanTimeline(delta)).ok();
        }
    }))
}

#[tauri::command]
pub fn set_animation_looping(
    state_handle: tauri::State<'_, state::Handle>,
    is_looping: bool,
) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document
                .process_command(Command::SetAnimationLooping(is_looping))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn apply_direction_preset(
    state_handle: tauri::State<'_, state::Handle>,
    preset: dto::DirectionPreset,
) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document
                .process_command(Command::ApplyDirectionPreset(preset.into()))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn select_direction(
    state_handle: tauri::State<'_, state::Handle>,
    direction: dto::Direction,
) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document
                .process_command(Command::SelectDirection(direction.into()))
                .ok();
        }
    }))
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
pub fn delete_selected_keyframes(
    state_handle: tauri::State<'_, state::Handle>,
) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document
                .process_command(Command::DeleteSelectedKeyframes)
                .ok();
        }
    }))
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
pub fn delete_selected_hitboxes(
    state_handle: tauri::State<'_, state::Handle>,
) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document
                .process_command(Command::DeleteSelectedHitboxes)
                .ok();
        }
    }))
}

#[tauri::command]
pub fn lock_hitboxes(state_handle: tauri::State<'_, state::Handle>) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document.process_command(Command::LockHitboxes).ok();
        }
    }))
}

#[tauri::command]
pub fn unlock_hitboxes(state_handle: tauri::State<'_, state::Handle>) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document.process_command(Command::UnlockHitboxes).ok();
        }
    }))
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
pub fn begin_nudge_hitbox(
    state_handle: tauri::State<'_, state::Handle>,
    name: String,
) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document
                .process_command(Command::BeginNudgeHitbox(name))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn update_nudge_hitbox(
    state_handle: tauri::State<'_, state::Handle>,
    displacement: (i32, i32),
    both_axis: bool,
) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::OnlyWorkbench, |state| {
        if let Some(document) = state.current_document_mut() {
            document
                .process_command(Command::UpdateNudgeHitbox(displacement.into(), both_axis))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn end_nudge_hitbox(state_handle: tauri::State<'_, state::Handle>) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document.process_command(Command::EndNudgeHitbox).ok();
        }
    }))
}

#[tauri::command]
pub fn begin_resize_hitbox(
    state_handle: tauri::State<'_, state::Handle>,
    name: String,
    axis: dto::ResizeAxis,
) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document
                .process_command(Command::BeginResizeHitbox(name, axis.into()))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn update_resize_hitbox(
    state_handle: tauri::State<'_, state::Handle>,
    displacement: (i32, i32),
    preserve_aspect_ratio: bool,
) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::OnlyWorkbench, |state| {
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

#[tauri::command]
pub fn end_resize_hitbox(state_handle: tauri::State<'_, state::Handle>) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document.process_command(Command::EndResizeHitbox).ok();
        }
    }))
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
pub fn set_export_texture_file(app: tauri::AppHandle, file: PathBuf) -> Result<Patch, ()> {
    app.set_export_texture_file(file)
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
pub fn cancel_export_as(state_handle: tauri::State<'_, state::Handle>) -> Result<Patch, ()> {
    Ok(state_handle.mutate(StateTrim::Full, |state| {
        if let Some(document) = state.current_document_mut() {
            document.process_command(Command::CancelExportAs).ok();
        }
    }))
}

#[tauri::command]
pub async fn end_export_as(app: tauri::AppHandle) -> Result<Patch, ()> {
    app.end_export_as().await
}
