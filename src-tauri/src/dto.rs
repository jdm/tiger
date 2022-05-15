use serde::{Deserialize, Serialize};
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
    view: View,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Sheet {
    frames: Vec<Frame>,
    animations: Vec<Animation>,
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
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct View {
    content_tab: ContentTab,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
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
        for animation in sheet.animations.iter_mut() {
            animation.selected = document
                .view()
                .selection()
                .is_animation_selected(&animation.name);
        }
        Self {
            path: document.path().to_owned(),
            name: document.path().to_file_name(),
            view: document.view().into(),
            sheet,
        }
    }
}

impl From<&sheet::Sheet> for Sheet {
    fn from(sheet: &sheet::Sheet) -> Self {
        Self {
            frames: sheet.frames_iter().map(|f| f.into()).collect(),
            animations: sheet
                .animations_iter()
                .map(|(n, a)| (n, a).into())
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
        }
    }
}

impl From<&state::View> for View {
    fn from(view: &state::View) -> Self {
        Self {
            content_tab: view.content_tab().into(),
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
