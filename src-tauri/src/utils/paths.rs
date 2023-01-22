use std::path::{Path, PathBuf};

use crate::utils::handle;

pub type Handle = handle::Handle<Paths>;

#[derive(Clone, Debug)]
pub struct Paths {
    pub log_file: PathBuf,
    pub recent_documents_file: PathBuf,
    pub onboarding_file: PathBuf,
    pub updates_file: PathBuf,
}

impl Paths {
    pub fn new<P: AsRef<Path>, S: AsRef<str>>(local_app_data_dir: P, suffix: S) -> Self {
        std::fs::create_dir_all(&local_app_data_dir).unwrap();
        let suffix = suffix.as_ref();
        Self {
            log_file: local_app_data_dir
                .as_ref()
                .join(format!("tiger{suffix}.log")),
            recent_documents_file: local_app_data_dir
                .as_ref()
                .join(format!("recent-documents{suffix}.json")),
            onboarding_file: local_app_data_dir
                .as_ref()
                .join(format!("onboarding{suffix}.json")),
            updates_file: local_app_data_dir
                .as_ref()
                .join(format!("updates{suffix}.json")),
        }
    }

    #[cfg(test)]
    pub fn remove_all(&self) {
        std::fs::remove_file(&self.log_file).ok();
        std::fs::remove_file(&self.recent_documents_file).ok();
        std::fs::remove_file(&self.onboarding_file).ok();
        std::fs::remove_file(&self.updates_file).ok();
    }
}
