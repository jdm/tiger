use serde::Serialize;
use std::path::PathBuf;

use crate::sheet;
use crate::state;

// Typescript: @/stores/app

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct App {
    documents: Vec<PathBuf>,
    current_document: Option<Document>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    source: PathBuf,
    sheet: Sheet,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Sheet {
    frames: Vec<PathBuf>,
}

impl From<&state::App> for App {
    fn from(app: &state::App) -> Self {
        App {
            documents: app
                .documents_iter()
                .map(|d| d.source().to_owned())
                .collect(),
            current_document: app.current_document().map(|d| d.into()),
        }
    }
}

impl From<&state::Document> for Document {
    fn from(document: &state::Document) -> Self {
        Document {
            source: document.source().to_owned(),
            sheet: document.sheet().into(),
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
