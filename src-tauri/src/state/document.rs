use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
pub struct Document {
    source: PathBuf,
    transient: Option<Transient>,
}

impl Document {
    pub fn new<T: AsRef<Path>>(path: T) -> Document {
        Document {
            source: path.as_ref().to_owned(),
            transient: None,
        }
    }

    pub fn source(&self) -> &Path {
        &self.source
    }

    pub fn clear_transient(&mut self) {
        self.transient = None;
    }
}

#[derive(Debug)]
pub struct Transient {}
