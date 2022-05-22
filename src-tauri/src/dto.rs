use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use crate::sheet;
use crate::state;

// Typescript: @/stores/app

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct App {
    documents: Vec<Document>,
    current_document_path: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    path: PathBuf,
    name: String,
    sheet: Sheet,
    content_tab: ContentTab,
    workbench_offset: (i32, i32),
    current_animation_name: Option<String>,
    current_sequence_direction: Option<Direction>,
    timeline_clock_millis: u64,
    timeline_is_playing: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Sheet {
    frames: Vec<Frame>,
    animations: HashMap<String, Animation>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    path: PathBuf,
    name: String,
    selected: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Animation {
    name: String,
    selected: bool,
    sequences: HashMap<Direction, Sequence>,
}

#[derive(Eq, PartialEq, Hash, Serialize)]
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Sequence {
    keyframes: Vec<Keyframe>,
    duration_millis: Option<u32>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Keyframe {
    frame: PathBuf,
    name: String,
    duration_millis: u32,
}

#[derive(Deserialize, Serialize)]
pub enum ContentTab {
    Frames,
    Animations,
}

trait ToFileName {
    fn to_file_name(&self) -> String;
}

impl<T: AsRef<Path>> ToFileName for T {
    fn to_file_name(&self) -> String {
        self.as_ref()
            .file_name()
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
        }
    }
}

impl From<&state::Document> for Document {
    fn from(document: &state::Document) -> Self {
        let mut sheet: Sheet = document.sheet().into();
        for frame in sheet.frames.iter_mut() {
            frame.selected = document.view().selection().is_frame_selected(&frame.path);
        }
        for (name, animation) in sheet.animations.iter_mut() {
            animation.selected = document.view().selection().is_animation_selected(name);
        }
        Self {
            path: document.path().to_owned(),
            name: document.path().to_file_name(),
            sheet,
            content_tab: document.view().content_tab().into(),
            workbench_offset: document.view().workbench_offset().to_i32().to_tuple(),
            current_animation_name: document.view().current_animation().to_owned(),
            current_sequence_direction: document.view().current_sequence().map(|d| d.into()),
            timeline_clock_millis: document.view().timeline_clock().as_millis() as u64,
            timeline_is_playing: document.persistent().is_timeline_playing(),
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
            duration_millis: keyframe.duration_millis(),
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
