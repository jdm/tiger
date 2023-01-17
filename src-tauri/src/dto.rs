use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::path::PathBuf;
use uuid::Uuid;

use crate::document::{self};
use crate::features::onboarding;
use crate::sheet::{self, Paths};
use crate::state;

// Typescript: @/stores/state

pub static EVENT_EXPORT_ERROR: &str = "export-error";
pub static EVENT_EXPORT_SUCCESS: &str = "export-success";
pub static EVENT_INVALIDATE_TEXTURE: &str = "invalidate-texture";
pub static EVENT_OPEN_DOCUMENT_ERROR: &str = "open-document-error";
pub static EVENT_PATCH_STATE: &str = "patch-state";
pub static EVENT_REPLACE_STATE: &str = "replace-state";
pub static EVENT_SAVE_DOCUMENT_ERROR: &str = "save-document-error";

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub documents: Vec<Document>,
    pub current_document_path: Option<PathBuf>,
    pub recent_document_paths: Vec<RecentDocument>,
    pub clipboard_manifest: Option<ClipboardManifest>,
    pub is_release_build: bool,
    pub error: Option<UserFacingError>,
    pub onboarding_step: OnboardingStep,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RecentDocument {
    pub path: PathBuf,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum ClipboardManifest {
    Animations,
    Keyframes,
    Hitboxes,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UserFacingError {
    pub key: String,
    pub title: String,
    pub summary: String,
    pub details: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum OnboardingStep {
    NotStarted,
    ImportFrame,
    CreateAnimation,
    PlaceFrameOnTimeline,
    Completed,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub animation_being_renamed: Option<String>,
    pub animations_filter: String,
    pub animations_list_offset: u32,
    pub current_animation_name: Option<String>,
    pub current_keyframe_index: Option<usize>,
    pub current_sequence_direction: Option<Direction>,
    pub darken_sprites: bool,
    pub export_settings_being_edited: Option<ExportSettings>,
    pub export_settings_validation: Option<ExportSettingsValidation>,
    pub frames_being_relocated: Option<HashMap<PathBuf, PathBuf>>,
    pub frames_being_dragged: HashSet<PathBuf>,
    pub frames_filter: String,
    pub frames_list_mode: ListMode,
    pub frames_list_offset: u32,
    pub has_unsaved_changes: bool,
    pub hide_hitboxes: bool,
    pub hide_origin: bool,
    pub hide_sprite: bool,
    pub hitbox_being_renamed: Option<String>,
    pub hitboxes_being_nudged: HashSet<String>,
    pub hitboxes_being_resized: HashSet<String>,
    pub hitboxes_list_offset: u32,
    pub is_dragging_keyframe_duration: bool,
    pub keyframe_snapping_base_duration_millis: u64,
    pub keyframes_being_dragged: HashSet<(Direction, usize)>,
    pub last_interacted_animation: Option<String>,
    pub last_interacted_frame: Option<PathBuf>,
    pub last_interacted_hitbox: Option<String>,
    pub lock_hitboxes: bool,
    pub name: String,
    pub path: PathBuf,
    pub preserve_aspect_ratio: bool,
    pub redo_effect: Option<String>,
    pub sheet: Sheet,
    pub snap_keyframe_durations: bool,
    pub snap_keyframes_to_multiples_of_duration: bool,
    pub snap_keyframes_to_other_keyframes: bool,
    pub timeline_clock_millis: u64,
    pub timeline_is_playing: bool,
    pub timeline_offset_millis: f32,
    pub timeline_zoom_amount: f32,
    pub timeline_zoom_factor: f32,
    pub undo_effect: Option<String>,
    pub was_close_requested: bool,
    pub workbench_offset: (f32, f32),
    pub workbench_zoom: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Sheet {
    pub frames: Vec<Frame>,
    pub animations: Vec<Animation>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    pub path: PathBuf,
    pub name: String,
    pub selected: bool,
    pub filtered_out: bool,
    pub missing_on_disk: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Animation {
    pub name: String,
    pub selected: bool,
    pub filtered_out: bool,
    pub sequences: HashMap<Direction, Sequence>,
    pub direction_preset: Option<DirectionPreset>,
    pub is_looping: bool,
    pub key: Uuid,
}

#[derive(Clone, Debug, Copy, Deserialize, Eq, PartialEq, Hash, Serialize)]
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

#[derive(Clone, Debug, Copy, Deserialize, Eq, PartialEq, Serialize)]
pub enum DirectionPreset {
    FourDirections,
    EightDirections,
    LeftRight,
    UpDown,
    Isometric,
    FixedAngle,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Sequence {
    pub keyframes: Vec<Keyframe>,
    pub duration_millis: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Keyframe {
    pub frame: PathBuf,
    pub name: String,
    pub selected: bool,
    pub start_time_millis: u64,
    pub duration_millis: u64,
    pub offset: (i32, i32),
    pub hitboxes: Vec<Hitbox>,
    pub key: Uuid,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Hitbox {
    pub name: String,
    pub selected: bool,
    pub top_left: (i32, i32),
    pub size: (u32, u32),
    pub key: Uuid,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportSettings {
    pub template_file: PathBuf,
    pub atlas_image_file: PathBuf,
    pub metadata_file: PathBuf,
    pub metadata_paths_root: PathBuf,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum ExportSettingsError {
    ExpectedAbsolutePath,
    ExpectedDirectory,
    ExpectedFile,
    FileNotFound,
    #[serde(rename = "templateError")]
    TemplateError(String),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportSettingsValidation {
    pub valid_settings: bool,
    pub template_file_error: Option<ExportSettingsError>,
    pub atlas_image_file_error: Option<ExportSettingsError>,
    pub metadata_file_error: Option<ExportSettingsError>,
    pub metadata_paths_root_error: Option<ExportSettingsError>,
}

#[derive(Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TextureInvalidation {
    pub path: PathBuf,
}

#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OpenDocumentError {
    pub document_name: String,
    pub error: String,
}

#[derive(Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SaveDocumentError {
    pub document_name: String,
    pub error: String,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportError {
    pub document_name: String,
    pub error: String,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportSuccess {
    pub document_name: String,
    pub atlas_image_file_path: PathBuf,
    pub atlas_image_file_name: String,
    pub metadata_file_path: PathBuf,
    pub metadata_file_name: String,
}

#[derive(Clone, Copy)]
pub enum StateTrim {
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

impl state::State {
    pub fn to_dto(&self, trim: StateTrim) -> State {
        State {
            documents: self
                .documents_iter()
                .map(|document| {
                    let is_current = |path| Some(path) == self.current_document().map(|d| d.path());
                    let doc_trim = match (trim, document.path()) {
                        (StateTrim::Full, _) => DocumentTrim::Full,
                        (StateTrim::OnlyCurrentDocument, p) if is_current(p) => DocumentTrim::Full,
                        (StateTrim::OnlyWorkbench, p) if is_current(p) => {
                            DocumentTrim::OnlyWorkbench
                        }
                        (StateTrim::OnlyCurrentDocument, _) => DocumentTrim::Empty,
                        (StateTrim::OnlyWorkbench, _) => DocumentTrim::Empty,
                        (StateTrim::NoDocuments, _) => DocumentTrim::Empty,
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
            onboarding_step: (&self.onboarding_step()).into(),
        }
    }
}

impl From<&state::UserFacingError> for UserFacingError {
    fn from(error: &state::UserFacingError) -> Self {
        Self {
            key: error.key.to_string(),
            title: error.title.clone(),
            summary: error.summary.clone(),
            details: error.details.clone(),
        }
    }
}

impl From<&onboarding::OnboardingStep> for OnboardingStep {
    fn from(step: &onboarding::OnboardingStep) -> Self {
        match step {
            onboarding::OnboardingStep::NotStarted => OnboardingStep::NotStarted,
            onboarding::OnboardingStep::ImportFrame => OnboardingStep::ImportFrame,
            onboarding::OnboardingStep::CreateAnimation => OnboardingStep::CreateAnimation,
            onboarding::OnboardingStep::PlaceFrameOnTimeline => {
                OnboardingStep::PlaceFrameOnTimeline
            }
            onboarding::OnboardingStep::Completed => OnboardingStep::Completed,
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

impl Document {
    #[cfg(test)]
    pub fn frames(&self) -> HashSet<String> {
        self.sheet.frames.iter().map(|f| f.name.clone()).collect()
    }

    #[cfg(test)]
    pub fn frame<S: AsRef<str>>(&self, name: S) -> &Frame {
        self.sheet
            .frames
            .iter()
            .find(|f| f.name == name.as_ref())
            .unwrap()
    }

    #[cfg(test)]
    pub fn animations(&self) -> &Vec<Animation> {
        &self.sheet.animations
    }

    #[cfg(test)]
    pub fn animation<S: AsRef<str>>(&self, animation: S) -> &Animation {
        self.animations()
            .iter()
            .find(|a| a.name == animation.as_ref())
            .unwrap()
    }

    #[cfg(test)]
    pub fn sequence<S: AsRef<str>>(&self, animation: S, direction: Direction) -> &Sequence {
        self.animation(animation).sequence(direction)
    }

    #[cfg(test)]
    pub fn keyframes<S: AsRef<str>>(&self, animation: S, direction: Direction) -> &Vec<Keyframe> {
        self.animation(animation).keyframes(direction)
    }

    #[cfg(test)]
    pub fn keyframe<S: AsRef<str>>(
        &self,
        animation: S,
        direction: Direction,
        index: usize,
    ) -> &Keyframe {
        self.sequence(animation, direction).keyframe(index)
    }

    #[cfg(test)]
    pub fn hitboxes<S: AsRef<str>>(
        &self,
        animation: S,
        direction: Direction,
        index: usize,
    ) -> &Vec<Hitbox> {
        &self.keyframe(animation, direction, index).hitboxes
    }

    #[cfg(test)]
    pub fn hitbox<S: AsRef<str>, T: AsRef<str>>(
        &self,
        animation: S,
        direction: Direction,
        index: usize,
        hitbox: T,
    ) -> &Hitbox {
        self.keyframe(animation, direction, index).hitbox(hitbox)
    }

    #[cfg(test)]
    pub fn selected_frames(&self) -> HashSet<PathBuf> {
        self.sheet
            .frames
            .iter()
            .filter_map(|f| f.selected.then_some(f.path.clone()))
            .collect()
    }

    #[cfg(test)]
    pub fn selected_animations(&self) -> HashSet<String> {
        self.sheet
            .animations
            .iter()
            .filter_map(|a| a.selected.then_some(a.name.clone()))
            .collect()
    }

    #[cfg(test)]
    pub fn selected_keyframes(&self) -> HashSet<(Direction, usize)> {
        self.sheet
            .animations
            .iter()
            .find(|a| Some(&a.name) == self.current_animation_name.as_ref())
            .unwrap()
            .sequences
            .iter()
            .flat_map(|(d, s)| {
                s.keyframes
                    .iter()
                    .enumerate()
                    .filter_map(|(i, k)| k.selected.then_some((*d, i)))
            })
            .collect()
    }

    #[cfg(test)]
    pub fn selected_hitboxes(&self) -> HashSet<String> {
        self.keyframe(
            self.current_animation_name.as_ref().unwrap(),
            self.current_sequence_direction.unwrap(),
            self.current_keyframe_index.unwrap(),
        )
        .hitboxes
        .iter()
        .filter_map(|h| h.selected.then_some(h.name.clone()))
        .collect()
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

impl Animation {
    #[cfg(test)]
    pub fn sequence(&self, direction: Direction) -> &Sequence {
        self.sequences.get(&direction).unwrap()
    }

    #[cfg(test)]
    pub fn keyframes(&self, direction: Direction) -> &Vec<Keyframe> {
        &self.sequence(direction).keyframes
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

impl Sequence {
    #[cfg(test)]
    pub fn keyframe(&self, index: usize) -> &Keyframe {
        self.keyframes.get(index).unwrap()
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

impl Keyframe {
    #[cfg(test)]
    pub fn hitbox<S: AsRef<str>>(&self, name: S) -> &Hitbox {
        self.hitboxes
            .iter()
            .find(|h| h.name == name.as_ref())
            .unwrap()
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
                atlas_image_file: template_settings.atlas_image_file().to_owned(),
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
                atlas_image_file_error: s.atlas_image_file_error().map(|e| e.into()),
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
mod tests {

    use super::*;
    use crate::document;

    #[test]
    fn can_preserve_all() {
        let mut state = state::State::default();
        state.open_document(document::Document::open("test-data/samurai.tiger").unwrap());
        state.open_document(document::Document::open("test-data/flame.tiger").unwrap());
        let dto = state.to_dto(StateTrim::Full);
        assert_eq!(dto.documents.len(), 2);
        assert!(!dto.documents[0].sheet.animations.is_empty());
        assert!(!dto.documents[1].sheet.animations.is_empty());
    }

    #[test]
    fn can_trim_inactive_documents() {
        let mut state = state::State::default();
        state.open_document(document::Document::open("test-data/samurai.tiger").unwrap());
        state.open_document(document::Document::open("test-data/flame.tiger").unwrap());
        let dto = state.to_dto(StateTrim::OnlyCurrentDocument);
        assert_eq!(dto.documents.len(), 2);
        assert!(dto.documents[0].sheet.animations.is_empty());
        assert!(!dto.documents[1].sheet.animations.is_empty());
    }

    #[test]
    fn can_trim_all_except_workbench() {
        let mut state = state::State::default();
        state.open_document(document::Document::open("test-data/samurai.tiger").unwrap());
        state.open_document(document::Document::open("test-data/flame.tiger").unwrap());
        let animation_name = state
            .current_document()
            .and_then(|d| d.current_animation().as_ref())
            .cloned()
            .unwrap();
        let dto = state.to_dto(StateTrim::OnlyWorkbench);
        assert_eq!(dto.documents.len(), 2);
        assert!(dto.documents[0].sheet.animations.is_empty());
        // Must preserve size of animations array to avoid patching incorrect array entries
        assert_eq!(
            dto.documents[1].sheet.animations.len(),
            state
                .current_document()
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
        let mut state = state::State::default();
        state.open_document(document::Document::open("test-data/samurai.tiger").unwrap());
        let dto = state.to_dto(StateTrim::NoDocuments);
        assert!(!dto.documents.is_empty());
        assert!(dto.documents[0].sheet.animations.is_empty());
    }
}
