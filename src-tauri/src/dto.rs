use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::path::PathBuf;
use uuid::Uuid;

use crate::app;
use crate::document::{self};
use crate::sheet::{self, Paths};

// Typescript: @/stores/app

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct App {
    documents: Vec<Document>,
    current_document_path: Option<PathBuf>,
    recent_document_paths: Vec<RecentDocument>,
    clipboard_manifest: Option<ClipboardManifest>,
    is_release_build: bool,
    error: Option<UserFacingError>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RecentDocument {
    path: PathBuf,
    name: String,
}

#[derive(Clone, Serialize)]
pub enum ClipboardManifest {
    Animations,
    Keyframes,
    Hitboxes,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserFacingError {
    key: String,
    title: String,
    summary: String,
    details: String,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    animation_being_renamed: Option<String>,
    animations_filter: String,
    animations_list_offset: u32,
    current_animation_name: Option<String>,
    current_keyframe_index: Option<usize>,
    current_sequence_direction: Option<Direction>,
    darken_sprites: bool,
    export_settings_being_edited: Option<ExportSettings>,
    export_settings_validation: Option<ExportSettingsValidation>,
    frames_being_relocated: Option<HashMap<PathBuf, PathBuf>>,
    frames_being_dragged: HashSet<PathBuf>,
    frames_filter: String,
    frames_list_mode: ListMode,
    frames_list_offset: u32,
    has_unsaved_changes: bool,
    hide_hitboxes: bool,
    hide_origin: bool,
    hide_sprite: bool,
    hitbox_being_renamed: Option<String>,
    hitboxes_being_nudged: HashSet<String>,
    hitboxes_being_resized: HashSet<String>,
    hitboxes_list_offset: u32,
    is_dragging_keyframe_duration: bool,
    keyframe_snapping_base_duration_millis: u64,
    keyframes_being_dragged: HashSet<(Direction, usize)>,
    last_interacted_animation: Option<String>,
    last_interacted_frame: Option<PathBuf>,
    last_interacted_hitbox: Option<String>,
    lock_hitboxes: bool,
    name: String,
    path: PathBuf,
    preserve_aspect_ratio: bool,
    redo_effect: Option<String>,
    sheet: Sheet,
    snap_keyframe_durations: bool,
    snap_keyframes_to_multiples_of_duration: bool,
    snap_keyframes_to_other_keyframes: bool,
    timeline_clock_millis: u64,
    timeline_is_playing: bool,
    timeline_offset_millis: f32,
    timeline_zoom_amount: f32,
    timeline_zoom_factor: f32,
    undo_effect: Option<String>,
    was_close_requested: bool,
    workbench_offset: (f32, f32),
    workbench_zoom: f32,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Sheet {
    frames: Vec<Frame>,
    animations: Vec<Animation>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    path: PathBuf,
    name: String,
    selected: bool,
    filtered_out: bool,
    missing_on_disk: bool,
}

#[derive(Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Animation {
    name: String,
    selected: bool,
    filtered_out: bool,
    sequences: HashMap<Direction, Sequence>,
    direction_preset: Option<DirectionPreset>,
    is_looping: bool,
    key: Uuid,
}

#[derive(Clone, Copy, Deserialize, Eq, PartialEq, Hash, Serialize)]
pub enum Direction {
    East,
    NorthEast,
    North,
    NorthWest,
    West,
    SouthWest,
    South,
    SouthEast,
}

#[derive(Clone, Copy, Deserialize, Eq, PartialEq, Serialize)]
pub enum DirectionPreset {
    FourDirections,
    EightDirections,
    LeftRight,
    UpDown,
    Isometric,
    FixedAngle,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Sequence {
    keyframes: Vec<Keyframe>,
    duration_millis: Option<u64>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Keyframe {
    frame: PathBuf,
    name: String,
    selected: bool,
    start_time_millis: u64,
    duration_millis: u64,
    offset: (i32, i32),
    hitboxes: Vec<Hitbox>,
    key: Uuid,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Hitbox {
    name: String,
    selected: bool,
    top_left: (i32, i32),
    size: (u32, u32),
    key: Uuid,
}

#[derive(Clone, Deserialize, Serialize)]
pub enum ListMode {
    Linear,
    Grid4xN,
}

#[derive(Clone, Deserialize)]
pub enum ResizeAxis {
    N,
    S,
    W,
    E,
    NW,
    NE,
    SE,
    SW,
}

#[derive(Clone, Deserialize)]
pub enum NudgeDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Deserialize)]
pub enum BrowseDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportSettings {
    template_file: PathBuf,
    texture_file: PathBuf,
    metadata_file: PathBuf,
    metadata_paths_root: PathBuf,
}

#[derive(Clone, Serialize)]
pub enum ExportSettingsError {
    ExpectedAbsolutePath,
    ExpectedDirectory,
    ExpectedFile,
    FileNotFound,
    #[serde(rename = "templateError")]
    TemplateError(String),
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportSettingsValidation {
    valid_settings: bool,
    template_file_error: Option<ExportSettingsError>,
    texture_file_error: Option<ExportSettingsError>,
    metadata_file_error: Option<ExportSettingsError>,
    metadata_paths_root_error: Option<ExportSettingsError>,
}

#[derive(Clone, Copy)]
pub enum AppTrim {
    Full,
    OnlyCurrentDocument,
    OnlyWorkbench,
    NoDocuments,
}

pub enum DocumentTrim {
    Full,
    OnlyWorkbench,
    Empty,
}

enum SheetTrim {
    Full,
    OnlyAnimation(String),
    Empty,
}

pub trait ToFileStem {
    fn to_file_stem(&self) -> String;
}

impl<T: AsRef<Path>> ToFileStem for T {
    fn to_file_stem(&self) -> String {
        self.as_ref()
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| "??".to_owned())
    }
}

pub trait ToFileName {
    fn to_file_name(&self) -> String;
}

impl<T: AsRef<Path>> ToFileName for T {
    fn to_file_name(&self) -> String {
        self.as_ref()
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| "??".to_owned())
    }
}

impl app::App {
    pub fn to_dto(&self, trim: AppTrim) -> App {
        App {
            documents: self
                .documents_iter()
                .map(|document| {
                    let is_current = |path| Some(path) == self.current_document().map(|d| d.path());
                    let doc_trim = match (trim, document.path()) {
                        (AppTrim::Full, _) => DocumentTrim::Full,
                        (AppTrim::OnlyCurrentDocument, p) if is_current(p) => DocumentTrim::Full,
                        (AppTrim::OnlyWorkbench, p) if is_current(p) => DocumentTrim::OnlyWorkbench,
                        (AppTrim::OnlyCurrentDocument, _) => DocumentTrim::Empty,
                        (AppTrim::OnlyWorkbench, _) => DocumentTrim::Empty,
                        (AppTrim::NoDocuments, _) => DocumentTrim::Empty,
                    };
                    document.to_dto(doc_trim)
                })
                .collect(),
            current_document_path: self.current_document().map(|d| d.path().to_owned()),
            recent_document_paths: self
                .recent_documents()
                .map(|d| RecentDocument {
                    path: d.to_owned(),
                    name: d.to_file_name(),
                })
                .collect(),
            clipboard_manifest: self.clipboard_manifest().as_ref().map(|m| m.into()),
            is_release_build: !cfg!(debug_assertions),
            error: self.error().map(|e| e.into()),
        }
    }
}

impl From<&app::UserFacingError> for UserFacingError {
    fn from(error: &app::UserFacingError) -> Self {
        Self {
            key: error.key.to_string(),
            title: error.title.clone(),
            summary: error.summary.clone(),
            details: error.details.clone(),
        }
    }
}

impl From<&document::ClipboardManifest> for ClipboardManifest {
    fn from(manifest: &document::ClipboardManifest) -> Self {
        match manifest {
            document::ClipboardManifest::Animations => ClipboardManifest::Animations,
            document::ClipboardManifest::Keyframes => ClipboardManifest::Keyframes,
            document::ClipboardManifest::Hitboxes => ClipboardManifest::Hitboxes,
        }
    }
}

impl document::Document {
    fn to_dto(&self, trim: DocumentTrim) -> Document {
        let mut sheet = {
            let sheet_trim = match (trim, self.current_animation()) {
                (DocumentTrim::Full, _) => SheetTrim::Full,
                (DocumentTrim::OnlyWorkbench, Some(a)) => SheetTrim::OnlyAnimation(a.clone()),
                (DocumentTrim::OnlyWorkbench, None) => SheetTrim::Empty,
                (DocumentTrim::Empty, _) => SheetTrim::Empty,
            };
            self.sheet().to_dto(sheet_trim)
        };

        for frame in sheet.frames.iter_mut() {
            frame.selected = self.selection().is_frame_selected(&frame.path);
            frame.filtered_out = self.is_frame_filtered_out(&frame.path);
            frame.missing_on_disk = self.is_frame_missing_on_disk(&frame.path);
        }

        for animation in sheet.animations.iter_mut() {
            animation.selected = self.selection().is_animation_selected(&animation.name);
            animation.filtered_out = self.is_animation_filtered_out(&animation.name);
            for (direction, sequence) in animation.sequences.iter_mut() {
                let mut time_millis = 0;
                for (index, keyframe) in sequence.keyframes.iter_mut().enumerate() {
                    keyframe.selected = self.selection().is_keyframe_selected(
                        &animation.name,
                        (*direction).into(),
                        index,
                    );
                    keyframe.start_time_millis = time_millis;
                    time_millis += keyframe.duration_millis;
                    for hitbox in keyframe.hitboxes.iter_mut() {
                        hitbox.selected = self.selection().is_hitbox_selected(
                            &animation.name,
                            (*direction).into(),
                            index,
                            &hitbox.name,
                        );
                    }
                }
            }
        }

        Document {
            animation_being_renamed: self.animation_being_renamed().cloned(),
            animations_filter: self.animations_filter().to_owned(),
            animations_list_offset: self.animations_list_offset(),
            current_animation_name: self.current_animation().to_owned(),
            current_keyframe_index: self
                .workbench_sequence()
                .ok()
                .and_then(|(_, s)| s.keyframe_index_at(self.timeline_clock())),
            current_sequence_direction: self.current_sequence().map(|d| d.into()),
            darken_sprites: self.should_darken_sprites(),
            export_settings_being_edited: self.export_settings_edit().ok().map(|s| s.into()),
            export_settings_validation: self.validate_export_settings().ok().map(|s| (&s).into()),
            frames_being_dragged: self.frames_being_dragged(),
            frames_being_relocated: self.relocate_frames_edit().ok().cloned(),
            frames_filter: self.frames_filter().to_owned(),
            frames_list_mode: self.frames_list_mode().into(),
            frames_list_offset: self.frames_list_offset(),
            has_unsaved_changes: !self.is_saved(),
            hide_hitboxes: self.is_hiding_hitboxes(),
            hide_origin: self.is_hiding_origin(),
            hide_sprite: self.is_hiding_sprite(),
            hitbox_being_renamed: self.hitbox_being_renamed().cloned(),
            hitboxes_being_nudged: self
                .hitboxes_being_nudged()
                .into_iter()
                .map(String::from)
                .collect(),
            hitboxes_being_resized: self
                .hitboxes_being_resized()
                .into_iter()
                .map(String::from)
                .collect(),
            hitboxes_list_offset: self.hitboxes_list_offset(),
            is_dragging_keyframe_duration: self.is_dragging_keyframe_duration(),
            keyframe_snapping_base_duration_millis: self
                .keyframe_snapping_base_duration()
                .as_millis() as u64,
            keyframes_being_dragged: self
                .keyframes_being_dragged()
                .into_iter()
                .map(|(d, i)| (d.into(), i))
                .collect(),
            last_interacted_animation: self.selection().last_interacted_animation().to_owned(),
            last_interacted_frame: self.selection().last_interacted_frame().to_owned(),
            last_interacted_hitbox: self
                .selection()
                .last_interacted_hitbox()
                .as_ref()
                .map(|(_, _, _, h)| h.to_owned()),
            lock_hitboxes: self.are_hitboxes_locked(),
            name: self.path().to_file_name(),
            path: self.path().to_owned(),
            preserve_aspect_ratio: self.preserves_aspect_ratio(),
            redo_effect: self.redo_effect(),
            sheet,
            snap_keyframe_durations: self.should_snap_keyframe_durations(),
            snap_keyframes_to_multiples_of_duration: self
                .should_snap_keyframes_to_multiples_of_duration(),
            snap_keyframes_to_other_keyframes: self.should_snap_keyframes_to_other_keyframes(),
            timeline_clock_millis: self.timeline_clock().as_millis() as u64,
            timeline_is_playing: self.is_timeline_playing(),
            timeline_offset_millis: self.timeline_offset().as_secs_f32() * 1_000.0,
            timeline_zoom_amount: self.timeline_zoom_amount(),
            timeline_zoom_factor: self.timeline_zoom_factor(),
            undo_effect: self.undo_effect(),
            was_close_requested: self.close_requested(),
            workbench_offset: self.workbench_offset().to_tuple(),
            workbench_zoom: self.workbench_zoom(),
        }
    }
}

impl<P: Paths> sheet::Sheet<P> {
    fn to_dto(&self, trim: SheetTrim) -> Sheet {
        Sheet {
            frames: match trim {
                SheetTrim::Full => self.sorted_frames().into_iter().map(|f| f.into()).collect(),
                SheetTrim::OnlyAnimation(_) | SheetTrim::Empty => vec![],
            },
            animations: self
                .sorted_animations()
                .into_iter()
                .filter_map(|(name, animation)| match &trim {
                    SheetTrim::Full => Some(animation.to_dto(name)),
                    SheetTrim::OnlyAnimation(n) => {
                        if name == n {
                            Some(animation.to_dto(name))
                        } else {
                            Some(Animation::default())
                        }
                    }
                    SheetTrim::Empty => None,
                })
                .collect(),
        }
    }
}

impl<P: Paths> From<&sheet::Frame<P>> for Frame {
    fn from(frame: &sheet::Frame<P>) -> Self {
        Self {
            path: frame.source().to_owned(),
            name: frame.source().to_file_stem(),
            selected: false,
            filtered_out: false,
            missing_on_disk: false,
        }
    }
}

impl<P: Paths> sheet::Animation<P> {
    fn to_dto<T: AsRef<str>>(&self, name: T) -> Animation {
        Animation {
            name: name.as_ref().to_owned(),
            selected: false,
            filtered_out: false,
            sequences: self
                .sequences_iter()
                .map(|(d, s)| ((*d).into(), s.into()))
                .collect(),
            direction_preset: self.direction_preset().map(|p| p.into()),
            is_looping: self.looping(),
            key: self.key(),
        }
    }
}

impl From<sheet::Direction> for Direction {
    fn from(direction: sheet::Direction) -> Self {
        match direction {
            sheet::Direction::East => Direction::East,
            sheet::Direction::NorthEast => Direction::NorthEast,
            sheet::Direction::North => Direction::North,
            sheet::Direction::NorthWest => Direction::NorthWest,
            sheet::Direction::West => Direction::West,
            sheet::Direction::SouthWest => Direction::SouthWest,
            sheet::Direction::South => Direction::South,
            sheet::Direction::SouthEast => Direction::SouthEast,
        }
    }
}

impl From<Direction> for sheet::Direction {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::East => sheet::Direction::East,
            Direction::NorthEast => sheet::Direction::NorthEast,
            Direction::North => sheet::Direction::North,
            Direction::NorthWest => sheet::Direction::NorthWest,
            Direction::West => sheet::Direction::West,
            Direction::SouthWest => sheet::Direction::SouthWest,
            Direction::South => sheet::Direction::South,
            Direction::SouthEast => sheet::Direction::SouthEast,
        }
    }
}

impl From<sheet::DirectionPreset> for DirectionPreset {
    fn from(preset: sheet::DirectionPreset) -> Self {
        match preset {
            sheet::DirectionPreset::FourDirections => DirectionPreset::FourDirections,
            sheet::DirectionPreset::EightDirections => DirectionPreset::EightDirections,
            sheet::DirectionPreset::LeftRight => DirectionPreset::LeftRight,
            sheet::DirectionPreset::UpDown => DirectionPreset::UpDown,
            sheet::DirectionPreset::Isometric => DirectionPreset::Isometric,
            sheet::DirectionPreset::FixedAngle => DirectionPreset::FixedAngle,
        }
    }
}

impl From<DirectionPreset> for sheet::DirectionPreset {
    fn from(preset: DirectionPreset) -> Self {
        match preset {
            DirectionPreset::FourDirections => sheet::DirectionPreset::FourDirections,
            DirectionPreset::EightDirections => sheet::DirectionPreset::EightDirections,
            DirectionPreset::LeftRight => sheet::DirectionPreset::LeftRight,
            DirectionPreset::UpDown => sheet::DirectionPreset::UpDown,
            DirectionPreset::Isometric => sheet::DirectionPreset::Isometric,
            DirectionPreset::FixedAngle => sheet::DirectionPreset::FixedAngle,
        }
    }
}

impl<P: Paths> From<&sheet::Sequence<P>> for Sequence {
    fn from(sequence: &sheet::Sequence<P>) -> Self {
        Self {
            keyframes: sequence.keyframes_iter().map(|k| k.into()).collect(),
            duration_millis: sequence.duration_millis(),
        }
    }
}

impl<P: Paths> From<&sheet::Keyframe<P>> for Keyframe {
    fn from(keyframe: &sheet::Keyframe<P>) -> Self {
        Self {
            frame: keyframe.frame().to_owned(),
            name: keyframe.frame().to_file_stem(),
            selected: false,
            start_time_millis: 0,
            duration_millis: keyframe.duration_millis(),
            offset: keyframe.offset().to_tuple(),
            hitboxes: keyframe
                .sorted_hitboxes()
                .into_iter()
                .map(|(n, h)| (n.clone(), h).into())
                .collect(),
            key: keyframe.key(),
        }
    }
}

impl From<(String, &sheet::Hitbox)> for Hitbox {
    fn from((name, hitbox): (String, &sheet::Hitbox)) -> Self {
        Self {
            name,
            selected: false,
            top_left: hitbox.position().to_tuple(),
            size: hitbox.size().to_tuple(),
            key: hitbox.key(),
        }
    }
}

impl From<ListMode> for document::ListMode {
    fn from(list_mode: ListMode) -> Self {
        match list_mode {
            ListMode::Linear => document::ListMode::Linear,
            ListMode::Grid4xN => document::ListMode::Grid4xN,
        }
    }
}

impl From<document::ListMode> for ListMode {
    fn from(list_mode: document::ListMode) -> Self {
        match list_mode {
            document::ListMode::Linear => ListMode::Linear,
            document::ListMode::Grid4xN => ListMode::Grid4xN,
        }
    }
}

impl From<ResizeAxis> for document::ResizeAxis {
    fn from(resize_axis: ResizeAxis) -> Self {
        match resize_axis {
            ResizeAxis::N => document::ResizeAxis::N,
            ResizeAxis::S => document::ResizeAxis::S,
            ResizeAxis::W => document::ResizeAxis::W,
            ResizeAxis::E => document::ResizeAxis::E,
            ResizeAxis::NW => document::ResizeAxis::NW,
            ResizeAxis::NE => document::ResizeAxis::NE,
            ResizeAxis::SE => document::ResizeAxis::SE,
            ResizeAxis::SW => document::ResizeAxis::SW,
        }
    }
}

impl From<NudgeDirection> for document::NudgeDirection {
    fn from(nudge_direction: NudgeDirection) -> Self {
        match nudge_direction {
            NudgeDirection::Up => document::NudgeDirection::Up,
            NudgeDirection::Down => document::NudgeDirection::Down,
            NudgeDirection::Left => document::NudgeDirection::Left,
            NudgeDirection::Right => document::NudgeDirection::Right,
        }
    }
}

impl From<BrowseDirection> for document::BrowseDirection {
    fn from(direction: BrowseDirection) -> Self {
        match direction {
            BrowseDirection::Up => document::BrowseDirection::Up,
            BrowseDirection::Down => document::BrowseDirection::Down,
            BrowseDirection::Left => document::BrowseDirection::Left,
            BrowseDirection::Right => document::BrowseDirection::Right,
        }
    }
}

impl<P: Paths> From<&sheet::ExportSettings<P>> for ExportSettings {
    fn from(settings: &sheet::ExportSettings<P>) -> Self {
        match settings {
            sheet::ExportSettings::Template(template_settings) => Self {
                template_file: template_settings.template_file().to_owned(),
                texture_file: template_settings.texture_file().to_owned(),
                metadata_file: template_settings.metadata_file().to_owned(),
                metadata_paths_root: template_settings.metadata_paths_root().to_owned(),
            },
        }
    }
}

impl From<&document::ExportSettingsValidation> for ExportSettingsValidation {
    fn from(validation: &document::ExportSettingsValidation) -> Self {
        match validation {
            document::ExportSettingsValidation::Template(s) => Self {
                valid_settings: *s == document::TemplateExportSettingsValidation::default(),
                template_file_error: s.template_file_error().map(|e| e.into()),
                texture_file_error: s.texture_file_error().map(|e| e.into()),
                metadata_file_error: s.metadata_file_error().map(|e| e.into()),
                metadata_paths_root_error: s.metadata_paths_root_error().map(|e| e.into()),
            },
        }
    }
}

impl From<&document::ExportSettingsError> for ExportSettingsError {
    fn from(e: &document::ExportSettingsError) -> Self {
        match e {
            document::ExportSettingsError::ExpectedAbsolutePath => {
                ExportSettingsError::ExpectedAbsolutePath
            }
            document::ExportSettingsError::ExpectedDirectory => {
                ExportSettingsError::ExpectedDirectory
            }
            document::ExportSettingsError::ExpectedFile => ExportSettingsError::ExpectedFile,
            document::ExportSettingsError::FileNotFound => ExportSettingsError::FileNotFound,
            document::ExportSettingsError::TemplateError(details) => {
                ExportSettingsError::TemplateError(details.clone())
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::document;

    #[test]
    fn can_preserve_all() {
        let mut app = app::App::default();
        app.open_document(document::Document::open("test-data/samurai.tiger").unwrap());
        app.open_document(document::Document::open("test-data/flame.tiger").unwrap());
        let dto = app.to_dto(AppTrim::Full);
        assert_eq!(dto.documents.len(), 2);
        assert!(!dto.documents[0].sheet.animations.is_empty());
        assert!(!dto.documents[1].sheet.animations.is_empty());
    }

    #[test]
    fn can_trim_inactive_documents() {
        let mut app = app::App::default();
        app.open_document(document::Document::open("test-data/samurai.tiger").unwrap());
        app.open_document(document::Document::open("test-data/flame.tiger").unwrap());
        let dto = app.to_dto(AppTrim::OnlyCurrentDocument);
        assert_eq!(dto.documents.len(), 2);
        assert!(dto.documents[0].sheet.animations.is_empty());
        assert!(!dto.documents[1].sheet.animations.is_empty());
    }

    #[test]
    fn can_trim_all_except_workbench() {
        let mut app = app::App::default();
        app.open_document(document::Document::open("test-data/samurai.tiger").unwrap());
        app.open_document(document::Document::open("test-data/flame.tiger").unwrap());
        let animation_name = app
            .current_document()
            .unwrap()
            .current_animation()
            .as_ref()
            .cloned()
            .unwrap();
        let dto = app.to_dto(AppTrim::OnlyWorkbench);
        assert_eq!(dto.documents.len(), 2);
        assert!(dto.documents[0].sheet.animations.is_empty());
        // Must preserve size of animations array to avoid patching incorrect array entries
        assert_eq!(
            dto.documents[1].sheet.animations.len(),
            app.current_document()
                .unwrap()
                .sheet()
                .animations_iter()
                .count()
        );
        for animation in &dto.documents[1].sheet.animations {
            assert!(animation.sequences.is_empty() || animation.name == animation_name);
        }
    }

    #[test]
    fn can_trim_all_documents() {
        let mut app = app::App::default();
        app.open_document(document::Document::open("test-data/samurai.tiger").unwrap());
        let dto = app.to_dto(AppTrim::NoDocuments);
        assert!(!dto.documents.is_empty());
        assert!(dto.documents[0].sheet.animations.is_empty());
    }
}
