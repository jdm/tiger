use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::path::PathBuf;
use uuid::Uuid;

use crate::sheet;
use crate::state;

// Typescript: @/stores/app

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct App {
    documents: Vec<Document>,
    current_document_path: Option<String>,
    is_release_build: bool,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    path: PathBuf,
    name: String,
    has_unsaved_changes: bool,
    was_close_requested: bool,
    sheet: Sheet,
    content_tab: ContentTab,
    workbench_offset: (i32, i32),
    workbench_zoom: f32,
    current_animation_name: Option<String>,
    current_sequence_direction: Option<Direction>,
    current_keyframe_index: Option<usize>,
    timeline_clock_millis: u64,
    timeline_is_playing: bool,
    timeline_zoom: f32,
    darken_sprites: bool,
    hide_hitboxes: bool,
    is_dragging_keyframe_duration: bool,
    frames_being_dragged: Vec<PathBuf>,
    keyframes_being_dragged: HashSet<(Direction, usize)>,
    hitboxes_being_nudged: HashSet<String>,
    hitboxes_being_resized: HashSet<String>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Sheet {
    frames: Vec<Frame>,
    animations: HashMap<String, Animation>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    path: PathBuf,
    name: String,
    selected: bool,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Animation {
    name: String,
    selected: bool,
    sequences: HashMap<Direction, Sequence>,
    direction_preset: Option<DirectionPreset>,
    is_looping: bool,
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
    hitboxes: HashMap<String, Hitbox>,
    key: Uuid,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Hitbox {
    selected: bool,
    top_left: (i32, i32),
    size: (u32, u32),
    key: Uuid,
}

#[derive(Clone, Deserialize, Serialize)]
pub enum ContentTab {
    Frames,
    Animations,
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

trait ToFileName {
    fn to_file_name(&self) -> String;
}

impl<T: AsRef<Path>> ToFileName for T {
    fn to_file_name(&self) -> String {
        self.as_ref()
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or("??".to_owned())
    }
}

impl From<&state::App> for App {
    fn from(app: &state::App) -> Self {
        Self {
            documents: app.documents_iter().map(|d| d.into()).collect(),
            current_document_path: app
                .current_document()
                .map(|d| d.path().to_string_lossy().into_owned()),
            is_release_build: !cfg!(debug_assertions),
        }
    }
}

impl From<&state::Document> for Document {
    fn from(document: &state::Document) -> Self {
        let mut sheet: Sheet = document.sheet().into();
        for frame in sheet.frames.iter_mut() {
            frame.selected = document.view().selection().is_frame_selected(&frame.path);
        }
        for (animation_name, animation) in sheet.animations.iter_mut() {
            animation.selected = document
                .view()
                .selection()
                .is_animation_selected(animation_name);
            for (direction, sequence) in animation.sequences.iter_mut() {
                let mut time_millis = 0;
                for (index, keyframe) in sequence.keyframes.iter_mut().enumerate() {
                    keyframe.selected = document.view().selection().is_keyframe_selected(
                        animation_name,
                        (*direction).into(),
                        index,
                    );
                    keyframe.start_time_millis = time_millis;
                    time_millis += keyframe.duration_millis;
                    for (hitbox_name, hitbox) in keyframe.hitboxes.iter_mut() {
                        hitbox.selected = document.view().selection().is_hitbox_selected(
                            animation_name,
                            (*direction).into(),
                            index,
                            hitbox_name,
                        );
                    }
                }
            }
        }
        Self {
            path: document.path().to_owned(),
            name: document.path().to_file_name(),
            has_unsaved_changes: !document.is_saved(),
            was_close_requested: document.persistent().close_requested(),
            sheet,
            content_tab: document.view().content_tab().into(),
            workbench_offset: document.view().workbench_offset().to_i32().to_tuple(),
            workbench_zoom: document.view().workbench_zoom(),
            current_animation_name: document.view().current_animation().to_owned(),
            current_sequence_direction: document.view().current_sequence().map(|d| d.into()),
            current_keyframe_index: document
                .get_workbench_sequence()
                .ok()
                .and_then(|(_, s)| s.keyframe_index_at(document.view().timeline_clock())),
            timeline_clock_millis: document.view().timeline_clock().as_millis() as u64,
            timeline_is_playing: document.persistent().is_timeline_playing(),
            timeline_zoom: document.view().timeline_zoom(),
            darken_sprites: document.view().should_darken_sprites(),
            hide_hitboxes: document.view().is_hiding_hitboxes(),
            is_dragging_keyframe_duration: document.is_dragging_keyframe_duration(),
            frames_being_dragged: document.frames_being_dragged(),
            keyframes_being_dragged: document
                .keyframes_being_dragged()
                .into_iter()
                .map(|(d, i)| (d.into(), i))
                .collect(),
            hitboxes_being_nudged: document
                .hitboxes_being_nudged()
                .into_iter()
                .cloned()
                .collect(),
            hitboxes_being_resized: document
                .hitboxes_being_resized()
                .into_iter()
                .cloned()
                .collect(),
        }
    }
}

impl From<&sheet::Sheet> for Sheet {
    fn from(sheet: &sheet::Sheet) -> Self {
        Self {
            frames: sheet.frames_iter().map(|f| f.into()).collect(),
            animations: sheet
                .animations_iter()
                .map(|(n, a)| (n.to_owned(), (n, a).into()))
                .collect(),
        }
    }
}

impl From<&sheet::Frame> for Frame {
    fn from(frame: &sheet::Frame) -> Self {
        Self {
            path: frame.source().to_owned(),
            name: frame.source().to_file_name(),
            selected: false,
        }
    }
}

impl<T> From<(T, &sheet::Animation)> for Animation
where
    T: AsRef<str>,
{
    fn from(animation: (T, &sheet::Animation)) -> Self {
        Self {
            name: animation.0.as_ref().to_owned(),
            selected: false,
            sequences: animation
                .1
                .sequences_iter()
                .map(|(d, s)| ((*d).into(), s.into()))
                .collect(),
            direction_preset: sheet::DirectionPreset::from_directions(
                animation.1.sequences_iter().map(|(d, _s)| *d),
            )
            .map(|p| p.into()),
            is_looping: animation.1.looping(),
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

impl From<&sheet::Sequence> for Sequence {
    fn from(sequence: &sheet::Sequence) -> Self {
        Self {
            keyframes: sequence.keyframes_iter().map(|k| k.into()).collect(),
            duration_millis: sequence.duration_millis(),
        }
    }
}

impl From<&sheet::Keyframe> for Keyframe {
    fn from(keyframe: &sheet::Keyframe) -> Self {
        Self {
            frame: keyframe.frame().to_owned(),
            name: keyframe.frame().to_file_name(),
            selected: false,
            start_time_millis: 0,
            duration_millis: keyframe.duration_millis(),
            offset: keyframe.offset().to_tuple(),
            hitboxes: keyframe
                .hitboxes_iter()
                .map(|(n, h)| (n.clone(), h.into()))
                .collect(),
            key: keyframe.key(),
        }
    }
}

impl From<&sheet::Hitbox> for Hitbox {
    fn from(hitbox: &sheet::Hitbox) -> Self {
        Self {
            selected: false,
            top_left: hitbox.position().to_tuple(),
            size: hitbox.size().to_tuple(),
            key: hitbox.key(),
        }
    }
}

impl From<ContentTab> for state::ContentTab {
    fn from(content_tab: ContentTab) -> Self {
        match content_tab {
            ContentTab::Frames => state::ContentTab::Frames,
            ContentTab::Animations => state::ContentTab::Animations,
        }
    }
}

impl From<state::ContentTab> for ContentTab {
    fn from(content_tab: state::ContentTab) -> Self {
        match content_tab {
            state::ContentTab::Frames => ContentTab::Frames,
            state::ContentTab::Animations => ContentTab::Animations,
        }
    }
}

impl From<ResizeAxis> for state::ResizeAxis {
    fn from(resize_axis: ResizeAxis) -> Self {
        match resize_axis {
            ResizeAxis::N => state::ResizeAxis::N,
            ResizeAxis::S => state::ResizeAxis::S,
            ResizeAxis::W => state::ResizeAxis::W,
            ResizeAxis::E => state::ResizeAxis::E,
            ResizeAxis::NW => state::ResizeAxis::NW,
            ResizeAxis::NE => state::ResizeAxis::NE,
            ResizeAxis::SE => state::ResizeAxis::SE,
            ResizeAxis::SW => state::ResizeAxis::SW,
        }
    }
}
