use serde::Serialize;
use std::path::PathBuf;

use crate::state;

// Typescript: @/stores/app
#[derive(Serialize)]
pub struct App {
    documents: Vec<PathBuf>,
}

impl From<&state::App> for App {
    fn from(app: &state::App) -> Self {
        App {
            documents: app
                .documents_iter()
                .map(|d| d.source().to_owned())
                .collect(),
        }
    }
}
