use serde::{Deserialize, Serialize};
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
    frames: Vec<PathBuf>,
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

impl From<&state::App> for App {
    fn from(app: &state::App) -> Self {
        App {
            documents: app.documents_iter().map(|d| d.into()).collect(),
            current_document_path: app
                .current_document()
                .map(|d| d.path().to_string_lossy().into_owned()),
        }
    }
}

impl From<&state::Document> for Document {
    fn from(document: &state::Document) -> Self {
        Document {
            path: document.path().to_owned(),
            name: document
                .path()
                .file_name()
                .map(|s| s.to_string_lossy().into_owned())
                .unwrap_or("??".to_owned()),
            sheet: document.sheet().into(),
            view: document.view().into(),
        }
    }
}

impl From<&sheet::Sheet> for Sheet {
    fn from(sheet: &sheet::Sheet) -> Self {
        Sheet {
            frames: sheet.frames_iter().map(|f| f.source().to_owned()).collect(),
        }
    }
}

impl From<&state::View> for View {
    fn from(view: &state::View) -> Self {
        View {
            content_tab: view.content_tab.into(),
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
