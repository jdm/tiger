use euclid::*;
use failure::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::export::*;
use crate::sheet::ExportFormat;
use crate::state::*;

const SHEET_FILE_EXTENSION: &str = "tiger";
const TEMPLATE_FILE_EXTENSION: &str = "liquid";
const IMAGE_IMPORT_FILE_EXTENSIONS: &str = "png;tga;bmp";
const IMAGE_EXPORT_FILE_EXTENSIONS: &str = "png";

#[derive(Fail, Debug)]
pub enum StateError {
    #[fail(display = "No document is open")]
    NoDocumentOpen,
    #[fail(display = "Requested document was not found")]
    DocumentNotFound,
    #[fail(display = "Sheet has no export settings")]
    NoExistingExportSettings,
}

// State preventing undo/redo while not default
// Reset when focusing different document
// TODO.important review places where we write to current_tab and clear transient state!
#[derive(Clone, Debug)]
struct TransientState {
    content_frame_being_dragged: Option<PathBuf>,
    item_being_renamed: Option<RenameItem>,
    rename_buffer: Option<String>,
    workbench_hitbox_being_dragged: Option<String>,
    workbench_hitbox_drag_initial_mouse_position: Vector2D<f32>,
    workbench_hitbox_drag_initial_offset: Vector2D<i32>,
    workbench_hitbox_being_scaled: Option<String>,
    workbench_hitbox_scale_axis: ResizeAxis,
    workbench_hitbox_scale_initial_mouse_position: Vector2D<f32>,
    workbench_hitbox_scale_initial_position: Vector2D<i32>,
    workbench_hitbox_scale_initial_size: Vector2D<u32>,
    workbench_animation_frame_being_dragged: Option<usize>,
    workbench_animation_frame_drag_initial_mouse_position: Vector2D<f32>,
    workbench_animation_frame_drag_initial_offset: Vector2D<i32>,
    timeline_frame_being_scaled: Option<usize>,
    timeline_frame_scale_initial_duration: u32,
    timeline_frame_scale_initial_clock: Duration,
    timeline_frame_being_dragged: Option<usize>,
    timeline_scrubbing: bool,
}

impl TransientState {
    fn new() -> TransientState {
        TransientState {
            content_frame_being_dragged: None,
            item_being_renamed: None,
            rename_buffer: None,
            workbench_hitbox_being_dragged: None,
            workbench_hitbox_drag_initial_mouse_position: vec2(0.0, 0.0),
            workbench_hitbox_drag_initial_offset: vec2(0, 0),
            workbench_hitbox_being_scaled: None,
            workbench_hitbox_scale_axis: ResizeAxis::N,
            workbench_hitbox_scale_initial_mouse_position: vec2(0.0, 0.0),
            workbench_hitbox_scale_initial_position: vec2(0, 0),
            workbench_hitbox_scale_initial_size: vec2(0, 0),
            workbench_animation_frame_being_dragged: None,
            workbench_animation_frame_drag_initial_mouse_position: vec2(0.0, 0.0),
            workbench_animation_frame_drag_initial_offset: vec2(0, 0),
            timeline_frame_being_scaled: None,
            timeline_frame_scale_initial_duration: 0,
            timeline_frame_scale_initial_clock: Default::default(),
            timeline_frame_being_dragged: None,
            timeline_scrubbing: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tab {
    source: PathBuf,
    history: Vec<(Option<Command>, Document)>,
    current_history_position: usize,
    timeline_clock: Duration,
    timeline_playing: bool,
}

impl Tab {
    fn new<T: AsRef<Path>>(path: T) -> Tab {
        Tab {
            source: path.as_ref().to_path_buf(),
            history: vec![(None, Document::new())],
            current_history_position: 0,
            timeline_clock: Default::default(),
            timeline_playing: false,
        }
    }

    fn open<T: AsRef<Path>>(path: T) -> Result<Tab, Error> {
        Ok(Tab {
            source: path.as_ref().to_path_buf(),
            history: vec![(None, Document::open(path)?)],
            current_history_position: 0,
            timeline_clock: Default::default(),
            timeline_playing: false,
        })
    }

    pub fn get_source(&self) -> &Path {
        &self.source
    }

    fn get_current_document(&self) -> &Document {
        &self.history[self.current_history_position].1
    }
}

#[derive(Clone, Debug)]
pub struct AppState {
    tabs: Vec<Tab>,
    transient_state: TransientState,
    current_tab: Option<PathBuf>,
    clock: Duration,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            tabs: vec![],
            transient_state: TransientState::new(),
            current_tab: None,
            clock: Duration::new(0, 0),
        }
    }

    pub fn tick(&mut self, delta: Duration) {
        self.clock += delta;
        if let Some(tab) = self.get_current_tab_mut() {
            tab.get_current_document().tick(
                delta,
                &mut tab.timeline_clock,
                &mut tab.timeline_playing,
            );
        }
    }

    pub fn get_clock(&self) -> Duration {
        self.clock
    }

    fn is_opened<T: AsRef<Path>>(&self, path: T) -> bool {
        self.tabs.iter().any(|t| t.source == path.as_ref())
    }

    pub fn get_current(&self) -> Option<(&Tab, &Document)> {
        if let Some(current_path) = &self.current_tab {
            if let Some(tab) = self.tabs.iter().find(|d| &d.source == current_path) {
                return Some((tab, tab.get_current_document()));
            }
        }
        None
    }

    pub fn get_current_tab(&self) -> Option<&Tab> {
        if let Some(current_path) = &self.current_tab {
            self.tabs.iter().find(|d| &d.source == current_path)
        } else {
            None
        }
    }

    pub fn get_current_tab_mut(&self) -> Option<&mut Tab> {
        if let Some(current_path) = &self.current_tab {
            self.tabs.iter_mut().find(|d| &d.source == current_path)
        } else {
            None
        }
    }

    pub fn get_current_document(&self) -> Option<&Document> {
        if let Some(current_path) = &self.current_tab {
            self.tabs
                .iter()
                .find(|d| &d.source == current_path)
                .and_then(|t| Some(t.get_current_document()))
        } else {
            None
        }
    }

    fn get_document<T: AsRef<Path>>(&mut self, path: T) -> Option<&Document> {
        self.tabs
            .iter()
            .find(|d| d.source == path.as_ref())
            .and_then(|t| Some(t.get_current_document()))
    }

    fn get_tab<T: AsRef<Path>>(&mut self, path: T) -> Option<&Tab> {
        self.tabs.iter().find(|d| d.source == path.as_ref())
    }

    fn get_tab_mut<T: AsRef<Path>>(&mut self, path: T) -> Option<&mut Tab> {
        self.tabs.iter_mut().find(|d| d.source == path.as_ref())
    }

    fn end_new_document<T: AsRef<Path>>(&mut self, path: T) -> Result<(), Error> {
        match self.get_tab_mut(&path) {
            Some(d) => *d = Tab::new(path),
            None => {
                let tab = Tab::new(path);
                self.add_tab(tab);
            }
        }
        self.current_tab = Some(path.as_ref().to_path_buf());
        Ok(())
    }

    fn end_open_document<T: AsRef<Path>>(&mut self, path: T) -> Result<(), Error> {
        if self.get_tab(&path).is_none() {
            let tab = Tab::open(path)?;
            self.add_tab(tab);
        }
        self.current_tab = Some(path.as_ref().to_path_buf());
        Ok(())
    }

    fn relocate_document<T: AsRef<Path>, U: AsRef<Path>>(
        &mut self,
        from: T,
        to: U,
    ) -> Result<(), Error> {
        for tab in &mut self.tabs {
            if &tab.source == from.as_ref() {
                tab.source = to.as_ref().to_path_buf();
                if Some(from.as_ref().to_path_buf()) == self.current_tab {
                    self.current_tab = Some(to.as_ref().to_path_buf());
                }
                return Ok(());
            }
        }
        Err(StateError::DocumentNotFound.into())
    }

    fn add_tab(&mut self, added_tab: Tab) {
        assert!(!self.is_opened(&added_tab.source));
        self.tabs.push(added_tab);
    }

    fn close_current_document(&mut self) -> Result<(), Error> {
        let tab = self.get_current_tab().ok_or(StateError::NoDocumentOpen)?;
        let index = self
            .tabs
            .iter()
            .position(|d| d as *const Tab == tab as *const Tab)
            .ok_or(StateError::DocumentNotFound)?;
        self.tabs.remove(index);
        self.current_tab = if self.tabs.is_empty() {
            None
        } else {
            Some(
                self.tabs[std::cmp::min(index, self.tabs.len() - 1)]
                    .source
                    .clone(),
            )
        };
        Ok(())
    }

    fn close_all_documents(&mut self) {
        self.tabs.clear();
        self.current_tab = None;
    }

    fn save_all_documents(&mut self) -> Result<(), Error> {
        for tab in &mut self.tabs {
            tab.get_current_document().save(tab.get_source())?;
        }
        Ok(())
    }

    pub fn get_timeline_zoom_factor(&self) -> Result<f32, Error> {
        let document = self
            .get_current_document()
            .ok_or(StateError::NoDocumentOpen)?;
        Ok(document.get_timeline_zoom_factor())
    }

    pub fn tabs_iter(&self) -> impl Iterator<Item = &Tab> {
        self.tabs.iter()
    }

    pub fn documents_iter(&self) -> impl Iterator<Item = &Document> {
        self.tabs.iter().map(|t| t.get_current_document())
    }

    pub fn process_sync_command(&mut self, command: &SyncCommand) -> Result<(), Error> {
        let old_document = self.get_current_document();
        let mut document = old_document.cloned();

        match command {
            SyncCommand::EndNewDocument(p) => self.end_new_document(p)?,
            SyncCommand::EndOpenDocument(p) => self.end_open_document(p)?,
            SyncCommand::RelocateDocument(from, to) => self.relocate_document(from, to)?,
            SyncCommand::FocusDocument(p) => {
                if self.is_opened(&p) {
                    self.current_tab = Some(p.clone());
                }
            }
            SyncCommand::CloseCurrentDocument => self.close_current_document()?,
            SyncCommand::CloseAllDocuments => self.close_all_documents(),
            SyncCommand::SaveAllDocuments => self.save_all_documents()?,
            SyncCommand::BeginExportAs => document
                .ok_or(StateError::NoDocumentOpen)?
                .begin_export_as(),
            SyncCommand::CancelExportAs => document
                .ok_or(StateError::NoDocumentOpen)?
                .cancel_export_as(),
            SyncCommand::EndSetExportTextureDestination(p, d) => self
                .get_document(p)
                .cloned()
                .ok_or(StateError::DocumentNotFound)?
                .end_set_export_texture_destination(d)?,
            SyncCommand::EndSetExportMetadataDestination(p, d) => self
                .get_document(p)
                .cloned()
                .ok_or(StateError::DocumentNotFound)?
                .end_set_export_metadata_destination(d)?,
            SyncCommand::EndSetExportMetadataPathsRoot(p, d) => self
                .get_document(p)
                .cloned()
                .ok_or(StateError::DocumentNotFound)?
                .end_set_export_metadata_paths_root(d)?,
            SyncCommand::EndSetExportFormat(p, f) => self
                .get_document(p)
                .cloned()
                .ok_or(StateError::DocumentNotFound)?
                .end_set_export_format(f.clone())?,
            SyncCommand::EndExportAs => document
                .ok_or(StateError::NoDocumentOpen)?
                .end_export_as()?,
            SyncCommand::SwitchToContentTab(tab) => document
                .ok_or(StateError::NoDocumentOpen)?
                .switch_to_content_tab(*tab),
            SyncCommand::SelectFrame(p) => document
                .ok_or(StateError::NoDocumentOpen)?
                .select_frame(&p)?,
            SyncCommand::SelectAnimation(a) => document
                .ok_or(StateError::NoDocumentOpen)?
                .select_animation(&a)?,
            SyncCommand::SelectHitbox(h) => document
                .ok_or(StateError::NoDocumentOpen)?
                .select_hitbox(&h)?,
            SyncCommand::SelectAnimationFrame(af) => document
                .ok_or(StateError::NoDocumentOpen)?
                .select_animation_frame(*af)?,
            SyncCommand::SelectPrevious => document
                .ok_or(StateError::NoDocumentOpen)?
                .select_previous()?,
            SyncCommand::SelectNext => document.ok_or(StateError::NoDocumentOpen)?.select_next()?,
            SyncCommand::EditFrame(p) => {
                document.ok_or(StateError::NoDocumentOpen)?.edit_frame(&p)?
            }
            SyncCommand::EditAnimation(a) => document
                .ok_or(StateError::NoDocumentOpen)?
                .edit_animation(&a)?,
            SyncCommand::CreateAnimation => document
                .ok_or(StateError::NoDocumentOpen)?
                .create_animation()?,
            SyncCommand::BeginFrameDrag(f) => document
                .ok_or(StateError::NoDocumentOpen)?
                .begin_frame_drag(f)?,
            SyncCommand::EndFrameDrag => {
                document.ok_or(StateError::NoDocumentOpen)?.end_frame_drag()
            }
            SyncCommand::InsertAnimationFrameBefore(f, n) => document
                .ok_or(StateError::NoDocumentOpen)?
                .insert_animation_frame_before(f, *n)?,
            SyncCommand::ReorderAnimationFrame(a, b) => document
                .ok_or(StateError::NoDocumentOpen)?
                .reorder_animation_frame(*a, *b)?,
            SyncCommand::BeginAnimationFrameDurationDrag(a) => document
                .ok_or(StateError::NoDocumentOpen)?
                .begin_animation_frame_duration_drag(*a)?,
            SyncCommand::UpdateAnimationFrameDurationDrag(d) => document
                .ok_or(StateError::NoDocumentOpen)?
                .update_animation_frame_duration_drag(*d)?,
            SyncCommand::EndAnimationFrameDurationDrag => document
                .ok_or(StateError::NoDocumentOpen)?
                .end_animation_frame_duration_drag(),
            SyncCommand::BeginAnimationFrameDrag(a) => document
                .ok_or(StateError::NoDocumentOpen)?
                .begin_animation_frame_drag(*a)?,
            SyncCommand::EndAnimationFrameDrag => document
                .ok_or(StateError::NoDocumentOpen)?
                .end_animation_frame_drag(),
            SyncCommand::BeginAnimationFrameOffsetDrag(a, m) => document
                .ok_or(StateError::NoDocumentOpen)?
                .begin_animation_frame_offset_drag(*a, *m)?,
            SyncCommand::UpdateAnimationFrameOffsetDrag(o, b) => document
                .ok_or(StateError::NoDocumentOpen)?
                .update_animation_frame_offset_drag(*o, *b)?,
            SyncCommand::EndAnimationFrameOffsetDrag => document
                .ok_or(StateError::NoDocumentOpen)?
                .end_animation_frame_offset_drag(),
            SyncCommand::WorkbenchZoomIn => document
                .ok_or(StateError::NoDocumentOpen)?
                .workbench_zoom_in(),
            SyncCommand::WorkbenchZoomOut => document
                .ok_or(StateError::NoDocumentOpen)?
                .workbench_zoom_out(),
            SyncCommand::WorkbenchResetZoom => document
                .ok_or(StateError::NoDocumentOpen)?
                .workbench_reset_zoom(),
            SyncCommand::Pan(delta) => document.ok_or(StateError::NoDocumentOpen)?.pan(*delta),
            SyncCommand::CreateHitbox(p) => document
                .ok_or(StateError::NoDocumentOpen)?
                .create_hitbox(*p)?,
            SyncCommand::BeginHitboxScale(h, a, p) => document
                .ok_or(StateError::NoDocumentOpen)?
                .begin_hitbox_scale(&h, *a, *p)?,
            SyncCommand::UpdateHitboxScale(p) => document
                .ok_or(StateError::NoDocumentOpen)?
                .update_hitbox_scale(*p)?,
            SyncCommand::EndHitboxScale => document
                .ok_or(StateError::NoDocumentOpen)?
                .end_hitbox_scale(),
            SyncCommand::BeginHitboxDrag(a, m) => document
                .ok_or(StateError::NoDocumentOpen)?
                .begin_hitbox_drag(&a, *m)?,
            SyncCommand::UpdateHitboxDrag(o, b) => document
                .ok_or(StateError::NoDocumentOpen)?
                .update_hitbox_drag(*o, *b)?,
            SyncCommand::EndHitboxDrag => document
                .ok_or(StateError::NoDocumentOpen)?
                .end_hitbox_drag(),
            SyncCommand::TogglePlayback => document
                .ok_or(StateError::NoDocumentOpen)?
                .toggle_playback()?,
            SyncCommand::SnapToPreviousFrame => document
                .ok_or(StateError::NoDocumentOpen)?
                .snap_to_previous_frame()?,
            SyncCommand::SnapToNextFrame => document
                .ok_or(StateError::NoDocumentOpen)?
                .snap_to_next_frame()?,
            SyncCommand::ToggleLooping => document
                .ok_or(StateError::NoDocumentOpen)?
                .toggle_looping()?,
            SyncCommand::TimelineZoomIn => document
                .ok_or(StateError::NoDocumentOpen)?
                .timeline_zoom_in(),
            SyncCommand::TimelineZoomOut => document
                .ok_or(StateError::NoDocumentOpen)?
                .timeline_zoom_out(),
            SyncCommand::TimelineResetZoom => document
                .ok_or(StateError::NoDocumentOpen)?
                .timeline_reset_zoom(),
            SyncCommand::BeginScrub => document
                .ok_or(StateError::NoDocumentOpen)?
                .begin_timeline_scrub(),
            SyncCommand::UpdateScrub(t) => document
                .ok_or(StateError::NoDocumentOpen)?
                .update_timeline_scrub(*t)?,
            SyncCommand::EndScrub => document
                .ok_or(StateError::NoDocumentOpen)?
                .end_timeline_scrub(),
            SyncCommand::NudgeSelection(d, l) => document
                .ok_or(StateError::NoDocumentOpen)?
                .nudge_selection(d, *l)?,
            SyncCommand::DeleteSelection => document
                .ok_or(StateError::NoDocumentOpen)?
                .delete_selection(),
            SyncCommand::BeginRenameSelection => document
                .ok_or(StateError::NoDocumentOpen)?
                .begin_rename_selection()?,
            SyncCommand::UpdateRenameSelection(n) => document
                .ok_or(StateError::NoDocumentOpen)?
                .update_rename_selection(n),
            SyncCommand::EndRenameSelection => document
                .ok_or(StateError::NoDocumentOpen)?
                .end_rename_selection()?,
        };

        if document.as_ref() != old_document {
            // TODO push undo state
        }

        Ok(())
    }
}

fn begin_new_document() -> Result<CommandBuffer, Error> {
    let mut command_buffer = CommandBuffer::new();
    if let nfd::Response::Okay(path_string) =
        nfd::open_save_dialog(Some(SHEET_FILE_EXTENSION), None)?
    {
        let mut path = std::path::PathBuf::from(path_string);
        path.set_extension(SHEET_FILE_EXTENSION);
        command_buffer.end_new_document(path);
    };
    Ok(command_buffer)
}

fn begin_open_document() -> Result<CommandBuffer, Error> {
    let mut buffer = CommandBuffer::new();
    match nfd::open_file_multiple_dialog(Some(SHEET_FILE_EXTENSION), None)? {
        nfd::Response::Okay(path_string) => {
            let path = std::path::PathBuf::from(path_string);
            buffer.end_open_document(path);
        }
        nfd::Response::OkayMultiple(path_strings) => {
            for path_string in path_strings {
                let path = std::path::PathBuf::from(path_string);
                buffer.end_open_document(path);
            }
        }
        _ => (),
    };
    Ok(buffer)
}

fn save_as<T: AsRef<Path>>(source: T, document: &Document) -> Result<CommandBuffer, Error> {
    let mut buffer = CommandBuffer::new();
    if let nfd::Response::Okay(path_string) =
        nfd::open_save_dialog(Some(SHEET_FILE_EXTENSION), None)?
    {
        let mut new_path = std::path::PathBuf::from(path_string);
        new_path.set_extension(SHEET_FILE_EXTENSION);
        buffer.relocate_document(source, new_path);
        buffer.save(new_path, document);
    };
    Ok(buffer)
}

fn begin_import<T: AsRef<Path>>(into: T) -> Result<CommandBuffer, Error> {
    let mut buffer = CommandBuffer::new();
    match nfd::open_file_multiple_dialog(Some(IMAGE_IMPORT_FILE_EXTENSIONS), None)? {
        nfd::Response::Okay(path_string) => {
            let path = std::path::PathBuf::from(path_string);
            buffer.end_import(into, path);
        }
        nfd::Response::OkayMultiple(path_strings) => {
            for path_string in &path_strings {
                let path = std::path::PathBuf::from(path_string);
                buffer.end_import(into, path);
            }
        }
        _ => (),
    };
    Ok(buffer)
}

fn begin_set_export_texture_destination<T: AsRef<Path>>(
    document_path: T,
) -> Result<CommandBuffer, Error> {
    let mut buffer = CommandBuffer::new();
    if let nfd::Response::Okay(path_string) =
        nfd::open_save_dialog(Some(IMAGE_EXPORT_FILE_EXTENSIONS), None)?
    {
        let texture_destination = std::path::PathBuf::from(path_string);
        buffer.end_set_export_texture_destination(document_path, texture_destination);
    };
    Ok(buffer)
}

fn begin_set_export_metadata_destination<T: AsRef<Path>>(
    document_path: T,
) -> Result<CommandBuffer, Error> {
    let mut buffer = CommandBuffer::new();
    if let nfd::Response::Okay(path_string) = nfd::open_save_dialog(None, None)? {
        let metadata_destination = std::path::PathBuf::from(path_string);
        buffer.end_set_export_metadata_destination(document_path, metadata_destination);
    };
    Ok(buffer)
}

fn begin_set_export_metadata_paths_root<T: AsRef<Path>>(
    document_path: T,
) -> Result<CommandBuffer, Error> {
    let mut buffer = CommandBuffer::new();
    if let nfd::Response::Okay(path_string) = nfd::open_pick_folder(None)? {
        let metadata_paths_root = std::path::PathBuf::from(path_string);
        buffer.end_set_export_metadata_paths_root(document_path, metadata_paths_root);
    }
    Ok(buffer)
}

fn begin_set_export_format<T: AsRef<Path>>(document_path: T) -> Result<CommandBuffer, Error> {
    let mut buffer = CommandBuffer::new();
    if let nfd::Response::Okay(path_string) =
        nfd::open_file_dialog(Some(TEMPLATE_FILE_EXTENSION), None)?
    {
        let format = ExportFormat::Template(std::path::PathBuf::from(path_string));
        buffer.end_set_export_format(document_path, format);
    };
    Ok(buffer)
}

fn export(document: &Document) -> Result<(), Error> {
    let export_settings = document
        .get_sheet()
        .get_export_settings()
        .as_ref()
        .ok_or(StateError::NoExistingExportSettings)?;

    // TODO texture export performance is awful
    let packed_sheet = pack_sheet(document.get_sheet())?;
    let exported_data = export_sheet(
        document.get_sheet(),
        &export_settings,
        &packed_sheet.get_layout(),
    )?;

    {
        let mut file = File::create(&export_settings.metadata_destination)?;
        file.write_all(&exported_data.into_bytes())?;
    }
    {
        let mut file = File::create(&export_settings.texture_destination)?;
        packed_sheet.get_texture().write_to(&mut file, image::PNG)?;
    }

    Ok(())
}

pub fn process_async_command(command: &AsyncCommand) -> Result<CommandBuffer, Error> {
    let no_commands = CommandBuffer::new();
    match command {
        AsyncCommand::BeginNewDocument => begin_new_document(),
        AsyncCommand::BeginOpenDocument => begin_open_document(),
        AsyncCommand::Save(p, d) => d.save(p).and(Ok(no_commands)),
        AsyncCommand::SaveAs(p, d) => save_as(p, d),
        AsyncCommand::BeginSetExportTextureDestination(p) => {
            begin_set_export_texture_destination(p)
        }
        AsyncCommand::BeginSetExportMetadataDestination(p) => {
            begin_set_export_metadata_destination(p)
        }
        AsyncCommand::BeginSetExportMetadataPathsRoot(p) => begin_set_export_metadata_paths_root(p),
        AsyncCommand::BeginSetExportFormat(p) => begin_set_export_format(p),
        AsyncCommand::BeginImport(p) => begin_import(p),
        AsyncCommand::Export(d) => export(d).and(Ok(no_commands)),
    }
}
