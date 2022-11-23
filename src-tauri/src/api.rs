use euclid::vec2;
use json_patch::Patch;
use std::path::PathBuf;
use std::time::Duration;
use tauri::ClipboardManager;

use crate::app::{App, AppState};
use crate::document::{Command, Document, DocumentError};
use crate::dto::{self, DiffStrategy, ToFileName};
use crate::export::export_sheet;
use crate::sheet;

impl AppState<'_> {
    pub fn mutate<F>(&self, diff_strategy: DiffStrategy, operation: F) -> Patch
    where
        F: FnOnce(&mut App),
    {
        let mut app = self.0.lock();

        let mut old_state: dto::App = (&*app).into();
        operation(&mut app);
        let mut new_state: dto::App = (&*app).into();

        old_state.trim_for_fast_diff(diff_strategy);
        new_state.trim_for_fast_diff(diff_strategy);

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
    let app = app_state.0.lock();
    Ok((&*app).into())
}

#[tauri::command]
pub fn show_error_message(
    app_state: tauri::State<'_, AppState>,
    title: String,
    summary: String,
    details: String,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        app.show_error_message(title, summary, details);
    }))
}

#[tauri::command]
pub fn acknowledge_error(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        app.acknowledge_error();
    }))
}

#[tauri::command]
pub fn new_document(app_state: tauri::State<'_, AppState>, path: PathBuf) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        app.new_document(path);
    }))
}

#[tauri::command]
pub async fn open_documents(
    app_state: tauri::State<'_, AppState<'_>>,
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

    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        for document in documents {
            match document {
                (_, Ok(d)) => {
                    app.open_document(d);
                }
                (path, Err(e)) => {
                    app.show_error_message(
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

#[tauri::command]
pub fn focus_document(app_state: tauri::State<'_, AppState>, path: PathBuf) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        app.focus_document(&path).ok();
    }))
}

#[tauri::command]
pub fn close_document(
    window: tauri::Window,
    app_state: tauri::State<'_, AppState>,
    path: PathBuf,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
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
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
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
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
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
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        app.request_exit();
        if app.should_exit() {
            window.close().ok();
        }
    }))
}

#[tauri::command]
pub fn cancel_exit(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        app.cancel_exit();
    }))
}

#[tauri::command]
pub fn close_without_saving(
    window: tauri::Window,
    app_state: tauri::State<'_, AppState>,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
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
    app_state: tauri::State<'_, AppState<'_>>,
) -> Result<Patch, ()> {
    let (sheet, destination, version) = {
        let app = app_state.0.lock();
        match app.current_document() {
            Some(d) => (d.sheet().clone(), d.path().to_owned(), d.version()),
            _ => return Ok(Patch(Vec::new())),
        }
    };

    let write_destination = destination.clone();
    let result = tauri::async_runtime::spawn_blocking(move || sheet.write(&write_destination))
        .await
        .unwrap();

    Ok(app_state.mutate(DiffStrategy::Full, |app| match result {
        Ok(_) => {
            if let Some(document) = app.document_mut(&destination) {
                document.mark_as_saved(version);
            }
            app.advance_exit();
            if app.should_exit() {
                window.close().ok();
            }
        }
        Err(e) => app.show_error_message(
            "Error".to_owned(),
            format!(
                "An error occured while trying to save `{}`",
                destination.to_file_name()
            ),
            e.to_string(),
        ),
    }))
}

#[tauri::command]
pub async fn save_as(
    app_state: tauri::State<'_, AppState<'_>>,
    new_path: PathBuf,
) -> Result<Patch, ()> {
    let (sheet, old_path, version) = {
        let app = app_state.0.lock();
        match app.current_document() {
            Some(d) => (d.sheet().clone(), d.path().to_owned(), d.version()),
            _ => return Ok(Patch(Vec::new())),
        }
    };

    let write_destination = new_path.clone();
    let result = tauri::async_runtime::spawn_blocking(move || sheet.write(&write_destination))
        .await
        .unwrap();

    Ok(app_state.mutate(DiffStrategy::Full, |app| match result {
        Ok(_) => {
            app.relocate_document(old_path, &new_path);
            if let Some(document) = app.document_mut(&new_path) {
                document.mark_as_saved(version);
            }
        }
        Err(e) => app.show_error_message(
            "Error".to_owned(),
            format!(
                "An error occured while trying to save `{}`",
                new_path.to_file_name()
            ),
            e.to_string(),
        ),
    }))
}

#[tauri::command]
pub async fn save_all(app_state: tauri::State<'_, AppState<'_>>) -> Result<Patch, ()> {
    struct DocumentToSave {
        sheet: sheet::Sheet,
        destination: PathBuf,
        version: i32,
    }

    let documents_to_save: Vec<DocumentToSave> = {
        let app = app_state.0.lock();
        app.documents_iter()
            .map(|d| DocumentToSave {
                sheet: d.sheet().clone(),
                destination: d.path().to_owned(),
                version: d.version(),
            })
            .collect()
    };

    let mut work = Vec::new();
    for document in &documents_to_save {
        let sheet_cloned = document.sheet.clone();
        let destination_cloned = document.destination.clone();
        work.push(tauri::async_runtime::spawn_blocking(move || {
            sheet_cloned.write(&destination_cloned)
        }));
    }
    let results = futures::future::join_all(work)
        .await
        .into_iter()
        .map(|r| r.unwrap());

    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        for (document_to_save, result) in documents_to_save.iter().zip(results) {
            match result {
                Ok(_) => {
                    if let Some(document) = app.document_mut(&document_to_save.destination) {
                        document.mark_as_saved(document_to_save.version);
                    }
                }
                Err(e) => app.show_error_message(
                    "Error".to_owned(),
                    format!(
                        "An error occured while trying to save `{}`",
                        document_to_save.destination.to_file_name()
                    ),
                    e.to_string(),
                ),
            }
        }
    }))
}

#[tauri::command]
pub fn undo(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::Undo).ok();
        }
    }))
}

#[tauri::command]
pub fn redo(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::Redo).ok();
        }
    }))
}

#[tauri::command]
pub fn cut(
    tauri_app: tauri::AppHandle,
    app_state: tauri::State<'_, AppState>,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(data) = app.current_document().and_then(|d| d.copy()) {
            if let Ok(serialized) = serde_json::to_string(&data) {
                let mut clipboard = tauri_app.clipboard_manager();
                clipboard.write_text(serialized).ok();
            }
        }
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::DeleteSelection).ok();
        }
    }))
}

#[tauri::command]
pub fn copy(
    tauri_app: tauri::AppHandle,
    app_state: tauri::State<'_, AppState>,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(data) = app.current_document().and_then(|d| d.copy()) {
            if let Ok(serialized) = serde_json::to_string(&data) {
                let mut clipboard = tauri_app.clipboard_manager();
                clipboard.write_text(serialized).ok();
            }
        }
    }))
}

#[tauri::command]
pub fn paste(
    tauri_app: tauri::AppHandle,
    app_state: tauri::State<'_, AppState>,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        let clipboard = tauri_app.clipboard_manager();
        if let Ok(Some(serialized)) = clipboard.read_text() {
            if let Ok(data) = serde_json::from_str(&serialized) {
                if let Some(document) = app.current_document_mut() {
                    document.process_command(Command::Paste(data)).ok();
                }
            }
        }
    }))
}

#[tauri::command]
pub fn set_frames_list_mode(
    app_state: tauri::State<'_, AppState>,
    list_mode: dto::ListMode,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::SetFramesListMode(list_mode.into()))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn filter_frames(
    app_state: tauri::State<'_, AppState>,
    search_query: String,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::FilterFrames(search_query))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn filter_animations(
    app_state: tauri::State<'_, AppState>,
    search_query: String,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::FilterAnimations(search_query))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn import_frames(
    app_state: tauri::State<'_, AppState>,
    paths: Vec<PathBuf>,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::ImportFrames(paths)).ok();
        }
    }))
}

#[tauri::command]
pub fn delete_frame(app_state: tauri::State<'_, AppState>, path: PathBuf) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::DeleteFrame(path)).ok();
        }
    }))
}

#[tauri::command]
pub fn delete_selected_frames(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::DeleteSelectedFrames).ok();
        }
    }))
}

#[tauri::command]
pub fn delete_selection(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::DeleteSelection).ok();
        }
    }))
}

#[tauri::command]
pub fn nudge_selection(
    app_state: tauri::State<'_, AppState>,
    direction: dto::NudgeDirection,
    large_nudge: bool,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::NudgeSelection(direction.into(), large_nudge))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn browse_selection(
    app_state: tauri::State<'_, AppState>,
    direction: dto::BrowseDirection,
    shift: bool,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::BrowseSelection(direction.into(), shift))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn clear_selection(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
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
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::SelectFrame(path, shift, ctrl))
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
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::SelectAnimation(name, shift, ctrl))
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
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
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

#[tauri::command]
pub fn select_hitbox(
    app_state: tauri::State<'_, AppState>,
    name: String,
    shift: bool,
    ctrl: bool,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::SelectHitbox(name, shift, ctrl))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn pan(app_state: tauri::State<'_, AppState>, delta: (i32, i32)) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::OnlyWorkbench, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::Pan(vec2(delta.0 as f32, delta.1 as f32)))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn center_workbench(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::CenterWorkbench).ok();
        }
    }))
}

#[tauri::command]
pub fn zoom_in_workbench(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::ZoomInWorkbench).ok();
        }
    }))
}

#[tauri::command]
pub fn zoom_out_workbench(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::ZoomOutWorkbench).ok();
        }
    }))
}

#[tauri::command]
pub fn set_workbench_zoom_factor(
    app_state: tauri::State<'_, AppState>,
    zoom_factor: u32,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::SetWorkbenchZoomFactor(zoom_factor))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn reset_workbench_zoom(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::ResetWorkbenchZoom).ok();
        }
    }))
}

#[tauri::command]
pub fn enable_sprite_darkening(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::EnableSpriteDarkening)
                .ok();
        }
    }))
}

#[tauri::command]
pub fn disable_sprite_darkening(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::DisableSpriteDarkening)
                .ok();
        }
    }))
}

#[tauri::command]
pub fn hide_sprite(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::HideSprite).ok();
        }
    }))
}

#[tauri::command]
pub fn show_sprite(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::ShowSprite).ok();
        }
    }))
}

#[tauri::command]
pub fn hide_hitboxes(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::HideHitboxes).ok();
        }
    }))
}

#[tauri::command]
pub fn show_hitboxes(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::ShowHitboxes).ok();
        }
    }))
}

#[tauri::command]
pub fn hide_origin(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::HideOrigin).ok();
        }
    }))
}

#[tauri::command]
pub fn show_origin(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::ShowOrigin).ok();
        }
    }))
}

#[tauri::command]
pub fn create_animation(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::CreateAnimation).ok();
        }
    }))
}

#[tauri::command]
pub fn edit_animation(app_state: tauri::State<'_, AppState>, name: String) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
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
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::RenameAnimation(old_name, new_name))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn delete_animation(app_state: tauri::State<'_, AppState>, name: String) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::DeleteAnimation(name))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn delete_selected_animations(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::DeleteSelectedAnimations)
                .ok();
        }
    }))
}

#[tauri::command]
pub fn tick(app_state: tauri::State<'_, AppState>, delta_time_millis: f64) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
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
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::Play).ok();
        }
    }))
}

#[tauri::command]
pub fn pause(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
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
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::ScrubTimeline(Duration::from_millis(time_millis)))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn jump_to_animation_start(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::JumpToAnimationStart).ok();
        }
    }))
}

#[tauri::command]
pub fn jump_to_animation_end(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::JumpToAnimationEnd).ok();
        }
    }))
}

#[tauri::command]
pub fn jump_to_previous_frame(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::JumpToPreviousFrame).ok();
        }
    }))
}

#[tauri::command]
pub fn jump_to_next_frame(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::JumpToNextFrame).ok();
        }
    }))
}

#[tauri::command]
pub fn zoom_in_timeline(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::ZoomInTimeline).ok();
        }
    }))
}

#[tauri::command]
pub fn zoom_out_timeline(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::ZoomOutTimeline).ok();
        }
    }))
}

#[tauri::command]
pub fn set_timeline_zoom_amount(
    app_state: tauri::State<'_, AppState>,
    amount: f32,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::SetTimelineZoomAmount(amount))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn reset_timeline_zoom(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
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
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
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
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
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
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
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
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
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
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::DropFrameOnTimeline(direction.into(), index))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn end_drag_and_drop_frame(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::EndDragAndDropFrame).ok();
        }
    }))
}

#[tauri::command]
pub fn delete_selected_keyframes(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::DeleteSelectedKeyframes)
                .ok();
        }
    }))
}

#[tauri::command]
pub fn set_keyframe_duration(
    app_state: tauri::State<'_, AppState>,
    duration_millis: u64,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::SetKeyframeDuration(Duration::from_millis(
                    duration_millis,
                )))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn set_keyframe_offset_x(app_state: tauri::State<'_, AppState>, x: i32) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::SetKeyframeOffsetX(x))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn set_keyframe_offset_y(app_state: tauri::State<'_, AppState>, y: i32) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::SetKeyframeOffsetY(y))
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
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
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
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::DropKeyframeOnTimeline(direction.into(), index))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn end_drag_and_drop_keyframe(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
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
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
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
    Ok(app_state.mutate(DiffStrategy::OnlyWorkbench, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::UpdateDragKeyframeDuration(delta_millis))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn end_drag_keyframe_duration(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
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
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
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
    Ok(app_state.mutate(DiffStrategy::OnlyWorkbench, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::UpdateNudgeKeyframe(displacement.into(), both_axis))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn end_nudge_keyframe(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
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
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
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
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::RenameHitbox(old_name, new_name))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn delete_hitbox(app_state: tauri::State<'_, AppState>, name: String) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::DeleteHitbox(name)).ok();
        }
    }))
}

#[tauri::command]
pub fn delete_selected_hitboxes(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::DeleteSelectedHitboxes)
                .ok();
        }
    }))
}

#[tauri::command]
pub fn lock_hitboxes(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::LockHitboxes).ok();
        }
    }))
}

#[tauri::command]
pub fn unlock_hitboxes(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::UnlockHitboxes).ok();
        }
    }))
}

#[tauri::command]
pub fn set_hitbox_position_x(app_state: tauri::State<'_, AppState>, x: i32) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::SetHitboxPositionX(x))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn set_hitbox_position_y(app_state: tauri::State<'_, AppState>, y: i32) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::SetHitboxPositionY(y))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn set_hitbox_width(app_state: tauri::State<'_, AppState>, width: u32) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::SetHitboxWidth(width))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn set_hitbox_height(app_state: tauri::State<'_, AppState>, height: u32) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::SetHitboxHeight(height))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn toggle_preserve_aspect_ratio(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::TogglePreserveAspectRatio)
                .ok();
        }
    }))
}

#[tauri::command]
pub fn begin_nudge_hitbox(
    app_state: tauri::State<'_, AppState>,
    name: String,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
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
    Ok(app_state.mutate(DiffStrategy::OnlyWorkbench, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::UpdateNudgeHitbox(displacement.into(), both_axis))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn end_nudge_hitbox(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
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
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
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
    Ok(app_state.mutate(DiffStrategy::OnlyWorkbench, |app| {
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
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::EndResizeHitbox).ok();
        }
    }))
}

#[tauri::command]
pub async fn export(app_state: tauri::State<'_, AppState<'_>>) -> Result<Patch, ()> {
    let (sheet, document_name) = {
        let app = app_state.0.lock();
        match app.current_document() {
            Some(d) => (d.sheet().clone(), d.path().to_file_name()),
            _ => return Ok(Patch(Vec::new())),
        }
    };

    match tauri::async_runtime::spawn_blocking(move || export_sheet(&sheet))
        .await
        .unwrap()
    {
        Ok(_) => Ok(Patch(Vec::new())),
        Err(e) => Ok(app_state.mutate(DiffStrategy::Full, |app| {
            app.show_error_message(
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

#[tauri::command]
pub fn begin_export_as(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::BeginExportAs).ok();
        }
    }))
}

#[tauri::command]
pub fn set_export_template_file(
    app_state: tauri::State<'_, AppState>,
    file: PathBuf,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::SetExportTemplateFile(file))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn set_export_texture_file(
    app_state: tauri::State<'_, AppState>,
    file: PathBuf,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::SetExportTextureFile(file))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn set_export_metadata_file(
    app_state: tauri::State<'_, AppState>,
    file: PathBuf,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::SetExportMetadataFile(file))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn set_export_metadata_paths_root(
    app_state: tauri::State<'_, AppState>,
    directory: PathBuf,
) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document
                .process_command(Command::SetExportMetadataPathsRoot(directory))
                .ok();
        }
    }))
}

#[tauri::command]
pub fn cancel_export_as(app_state: tauri::State<'_, AppState>) -> Result<Patch, ()> {
    Ok(app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::CancelExportAs).ok();
        }
    }))
}

#[tauri::command]
pub async fn end_export_as(app_state: tauri::State<'_, AppState<'_>>) -> Result<Patch, ()> {
    let mut patch = app_state.mutate(DiffStrategy::Full, |app| {
        if let Some(document) = app.current_document_mut() {
            document.process_command(Command::EndExportAs).ok();
        }
    });

    let (sheet, document_name) = {
        let app = app_state.0.lock();
        match app.current_document() {
            Some(d) => (d.sheet().clone(), d.path().to_file_name()),
            _ => return Ok(patch),
        }
    };

    let result = tauri::async_runtime::spawn_blocking(move || export_sheet(&sheet))
        .await
        .unwrap();

    let mut additional_patch = app_state.mutate(DiffStrategy::Full, |app| {
        if let Err(e) = result {
            app.show_error_message(
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
