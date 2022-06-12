use euclid::vec2;
use json_patch::Patch;
use std::path::PathBuf;
use std::time::Duration;

use crate::dto;
use crate::state::{App, AppState, Command, Document, DocumentError, SelectionInput};

impl AppState {
    pub fn mutate<F>(&self, operation: F) -> Patch
    where
        F: FnOnce(&mut App),
    {
        let mut app = self.0.lock().unwrap();

        let old_state: dto::App = (&*app).into();
        operation(&mut *app);
        let new_state: dto::App = (&*app).into();

        let old_json = serde_json::to_value(old_state);
        let new_json = serde_json::to_value(new_state);

        match (old_json, new_json) {
            (Ok(o), Ok(n)) => json_patch::diff(&o, &n),
            _ => {
                println!("Patch serialization error");
                Patch(Vec::new())
            }
        }
    }
}

#[tauri::command]
pub fn get_state(app_state: tauri::State<'_, AppState>) -> Result<dto::App, ()> {
    let app = app_state.0.lock().unwrap();
    Ok((&*app).into())
}

#[tauri::command]
pub fn new_document(app_state: tauri::State<'_, AppState>, path: PathBuf) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        app.new_document(path);
    }))
}

#[tauri::command]
pub async fn open_documents(
    app_state: tauri::State<'_, AppState>,
    paths: Vec<PathBuf>,
) -> Result<Patch, ()> {
    let mut documents: Vec<(PathBuf, Result<Document, DocumentError>)> = Vec::new();
    for path in &paths {
        let open_path = path.to_owned();
        documents.push((
            open_path.clone(),
            tauri::async_runtime::spawn_blocking(move || Document::open(&open_path))
                .await
                .unwrap(),
        ));
    }

    Ok(app_state.mutate(|app| {
        for document in documents {
            match document {
                (_, Ok(d)) => {
                    app.open_document(d);
                }
                (path, Err(e)) => {
                    app.show_error_message(format!(
                        "Could not open `{}`: {e}",
                        path.to_string_lossy()
                    ));
                }
            }
        }
    }))
}

#[tauri::command]
pub fn focus_document(app_state: tauri::State<'_, AppState>, path: PathBuf) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        app.focus_document(&path).ok();
    }))
}

#[tauri::command]
pub fn close_document(
    window: tauri::Window,
    app_state: tauri::State<'_, AppState>,
    path: PathBuf,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.document_mut(&path) {
            document.request_close();
        }
        app.advance_exit();
        if app.should_exit() {
            window.close().ok();
        }
    }))
}

#[tauri::command]
pub fn close_current_document(
    window: tauri::Window,
    app_state: tauri::State<'_, AppState>,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document.request_close();
        }
        app.advance_exit();
        if app.should_exit() {
            window.close().ok();
        }
    }))
}

#[tauri::command]
pub fn close_all_documents(
    window: tauri::Window,
    app_state: tauri::State<'_, AppState>,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        for document in app.documents_iter_mut() {
            document.request_close();
        }
        app.advance_exit();
        if app.should_exit() {
            window.close().ok();
        }
    }))
}

#[tauri::command]
pub fn request_exit(
    window: tauri::Window,
    app_state: tauri::State<'_, AppState>,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        app.request_exit();
        if app.should_exit() {
            window.close().ok();
        }
    }))
}

#[tauri::command]
pub fn cancel_exit(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        app.cancel_exit();
    }))
}

#[tauri::command]
pub fn close_without_saving(
    window: tauri::Window,
    app_state: tauri::State<'_, AppState>,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        let path = app.current_document().map(|d| d.path().to_owned());
        if let Some(path) = path {
            app.close_document(path);
            app.advance_exit();
            if app.should_exit() {
                window.close().ok();
            }
        }
    }))
}

#[tauri::command]
pub async fn save(
    window: tauri::Window,
    app_state: tauri::State<'_, AppState>,
) -> Result<Patch, ()> {
    let (sheet, destination, version) = {
        let app = app_state.0.lock().unwrap();
        match app.current_document() {
            Some(d) => (d.sheet().clone(), d.path().to_owned(), d.version()),
            _ => return Ok(Patch(Vec::new())),
        }
    };

    let write_destination = destination.clone();
    let result = tauri::async_runtime::spawn_blocking(move || sheet.write(&write_destination))
        .await
        .unwrap();

    Ok(app_state.mutate(|app| match result {
        Ok(_) => {
            if let Some(document) = app.document_mut(&destination) {
                document.mark_as_saved(version);
            }
            app.advance_exit();
            if app.should_exit() {
                window.close().ok();
            }
        }
        Err(e) => app.show_error_message(format!(
            "Could not save `{}`: {e}",
            destination.to_string_lossy()
        )),
    }))
}

#[tauri::command]
pub fn undo(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::Undo).ok();
        }
    }))
}

#[tauri::command]
pub fn redo(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::Redo).ok();
        }
    }))
}

#[tauri::command]
pub fn focus_content_tab(
    app_state: tauri::State<'_, AppState>,
    content_tab: dto::ContentTab,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::FocusContentTab(content_tab.into()))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn import_frames(
    app_state: tauri::State<'_, AppState>,
    paths: Vec<PathBuf>,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::ImportFrames(paths)).ok();
        }
    }))
}

#[tauri::command]
pub fn delete_selected_frames(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::DeleteSelectedFrames).ok();
        }
    }))
}

#[tauri::command]
pub fn clear_selection(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::ClearSelection).ok();
        }
    }))
}

#[tauri::command]
pub fn select_frame(
    app_state: tauri::State<'_, AppState>,
    path: PathBuf,
    shift: bool,
    ctrl: bool,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::AlterSelection(
                    SelectionInput::Frame(path),
                    shift,
                    ctrl,
                ))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn select_animation(
    app_state: tauri::State<'_, AppState>,
    name: String,
    shift: bool,
    ctrl: bool,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::AlterSelection(
                    SelectionInput::Animation(name),
                    shift,
                    ctrl,
                ))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn select_keyframe(
    app_state: tauri::State<'_, AppState>,
    direction: dto::Direction,
    index: usize,
    shift: bool,
    ctrl: bool,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::AlterSelection(
                    SelectionInput::Keyframe(direction.into(), index),
                    shift,
                    ctrl,
                ))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn select_hitbox(
    app_state: tauri::State<'_, AppState>,
    name: String,
    shift: bool,
    ctrl: bool,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::AlterSelection(
                    SelectionInput::Hitbox(name),
                    shift,
                    ctrl,
                ))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn pan(app_state: tauri::State<'_, AppState>, delta: (i32, i32)) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::Pan(vec2(delta.0 as f32, delta.1 as f32)))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn center_workbench(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::CenterWorkbench).ok();
        }
    }))
}

#[tauri::command]
pub fn zoom_in_workbench(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::ZoomInWorkbench).ok();
        }
    }))
}

#[tauri::command]
pub fn zoom_out_workbench(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::ZoomOutWorkbench).ok();
        }
    }))
}

#[tauri::command]
pub fn reset_workbench_zoom(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::ResetWorkbenchZoom).ok();
        }
    }))
}

#[tauri::command]
pub fn enable_sprite_darkening(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::EnableSpriteDarkening)
                .ok();
        }
    }))
}

#[tauri::command]
pub fn disable_sprite_darkening(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::DisableSpriteDarkening)
                .ok();
        }
    }))
}

#[tauri::command]
pub fn create_animation(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::CreateAnimation).ok();
        }
    }))
}

#[tauri::command]
pub fn edit_animation(app_state: tauri::State<'_, AppState>, name: String) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::EditAnimation(name)).ok();
        }
    }))
}

#[tauri::command]
pub fn rename_animation(
    app_state: tauri::State<'_, AppState>,
    old_name: String,
    new_name: String,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::RenameAnimation(old_name, new_name))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn delete_animation(app_state: tauri::State<'_, AppState>, name: String) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::DeleteAnimation(name))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn delete_selected_animations(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::DeleteSelectedAnimations)
                .ok();
        }
    }))
}

#[tauri::command]
pub fn tick(app_state: tauri::State<'_, AppState>, delta_time_millis: f64) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::Tick(Duration::from_nanos(
                    (delta_time_millis * 1_000_000.0) as u64,
                )))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn play(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::Play).ok();
        }
    }))
}

#[tauri::command]
pub fn pause(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::Pause).ok();
        }
    }))
}

#[tauri::command]
pub fn scrub_timeline(
    app_state: tauri::State<'_, AppState>,
    time_millis: u64,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::ScrubTimeline(Duration::from_millis(time_millis)))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn zoom_in_timeline(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::ZoomInTimeline).ok();
        }
    }))
}

#[tauri::command]
pub fn zoom_out_timeline(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::ZoomOutTimeline).ok();
        }
    }))
}

#[tauri::command]
pub fn reset_timeline_zoom(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::ResetTimelineZoom).ok();
        }
    }))
}

#[tauri::command]
pub fn set_animation_looping(
    app_state: tauri::State<'_, AppState>,
    is_looping: bool,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::SetAnimationLooping(is_looping))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn apply_direction_preset(
    app_state: tauri::State<'_, AppState>,
    preset: dto::DirectionPreset,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::ApplyDirectionPreset(preset.into()))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn select_direction(
    app_state: tauri::State<'_, AppState>,
    direction: dto::Direction,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::SelectDirection(direction.into()))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn begin_drag_and_drop_frame(
    app_state: tauri::State<'_, AppState>,
    frame: PathBuf,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::BeginDragAndDropFrame(frame))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn drop_frame_on_timeline(
    app_state: tauri::State<'_, AppState>,
    direction: dto::Direction,
    index: usize,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::DropFrameOnTimeline(direction.into(), index))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn end_drag_and_drop_frame(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::EndDragAndDropFrame).ok();
        }
    }))
}

#[tauri::command]
pub fn delete_selected_keyframes(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::DeleteSelectedKeyframes)
                .ok();
        }
    }))
}

#[tauri::command]
pub fn begin_drag_and_drop_keyframe(
    app_state: tauri::State<'_, AppState>,
    direction: dto::Direction,
    index: usize,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::BeginDragAndDropKeyframe(direction.into(), index))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn drop_keyframe_on_timeline(
    app_state: tauri::State<'_, AppState>,
    direction: dto::Direction,
    index: usize,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::DropKeyframeOnTimeline(direction.into(), index))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn end_drag_and_drop_keyframe(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::EndDragAndDropKeyframe)
                .ok();
        }
    }))
}

#[tauri::command]
pub fn begin_drag_keyframe_duration(
    app_state: tauri::State<'_, AppState>,
    direction: dto::Direction,
    index: usize,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::BeginDragKeyframeDuration(direction.into(), index))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn update_drag_keyframe_duration(
    app_state: tauri::State<'_, AppState>,
    delta_millis: i64,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::UpdateDragKeyframeDuration(delta_millis))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn end_drag_keyframe_duration(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::EndDragKeyframeDuration())
                .ok();
        }
    }))
}

#[tauri::command]
pub fn begin_nudge_keyframe(
    app_state: tauri::State<'_, AppState>,
    direction: dto::Direction,
    index: usize,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::BeginNudgeKeyframe(direction.into(), index))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn update_nudge_keyframe(
    app_state: tauri::State<'_, AppState>,
    displacement: (i32, i32),
    both_axis: bool,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::UpdateNudgeKeyframe(displacement.into(), both_axis))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn end_nudge_keyframe(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::EndNudgeKeyframe()).ok();
        }
    }))
}

#[tauri::command]
pub fn create_hitbox(
    app_state: tauri::State<'_, AppState>,
    position: Option<(i32, i32)>,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::CreateHitbox(position.map(|p| p.into())))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn rename_hitbox(
    app_state: tauri::State<'_, AppState>,
    old_name: String,
    new_name: String,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::RenameHitbox(old_name, new_name))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn delete_hitbox(app_state: tauri::State<'_, AppState>, name: String) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::DeleteHitbox(name)).ok();
        }
    }))
}

#[tauri::command]
pub fn hide_hitboxes(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::HideHitboxes).ok();
        }
    }))
}

#[tauri::command]
pub fn show_hitboxes(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::ShowHitboxes).ok();
        }
    }))
}

#[tauri::command]
pub fn begin_nudge_hitbox(
    app_state: tauri::State<'_, AppState>,
    name: String,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::BeginNudgeHitbox(name))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn update_nudge_hitbox(
    app_state: tauri::State<'_, AppState>,
    displacement: (i32, i32),
    both_axis: bool,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::UpdateNudgeHitbox(displacement.into(), both_axis))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn end_nudge_hitbox(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::EndNudgeHitbox).ok();
        }
    }))
}

#[tauri::command]
pub fn begin_resize_hitbox(
    app_state: tauri::State<'_, AppState>,
    name: String,
    axis: dto::ResizeAxis,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::BeginResizeHitbox(name, axis.into()))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn update_resize_hitbox(
    app_state: tauri::State<'_, AppState>,
    displacement: (i32, i32),
    preserve_aspect_ratio: bool,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
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
pub fn end_resize_hitbox(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(|app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::EndResizeHitbox).ok();
        }
    }))
}
