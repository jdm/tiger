use std::path::PathBuf;

use crate::utils::handle;

pub type Handle = handle::Handle<Paths>;

#[derive(Clone, Debug)]
pub struct Paths {
    pub log_file: PathBuf,
    pub recent_documents_file: PathBuf,
}

impl Paths {
    pub fn new() -> Self {
        let project_dirs = directories::ProjectDirs::from("org", "Permafrost", "Tiger").unwrap();
        let data_local_dir = project_dirs.data_local_dir();
        std::fs::create_dir_all(data_local_dir).unwrap();
        Self {
            log_file: data_local_dir.join("tiger.log"),
            recent_documents_file: data_local_dir.join("recent-documents.json"),
        }
    }

    #[cfg(test)]
    pub fn test_outputs() -> Self {
        use std::{
            collections::hash_map::DefaultHasher,
            hash::{Hash, Hasher},
        };

        let backtrace = std::backtrace::Backtrace::force_capture();
        let backtrace = backtrace.to_string();
        let mut s = DefaultHasher::new();
        backtrace.hash(&mut s);
        let hash = s.finish();
        Paths {
            recent_documents_file: format!("test-output/recent_documents-{hash}.json").into(),
            log_file: format!("test-output/log-{hash}.log").into(),
        }
    }
}

impl Default for Paths {
    fn default() -> Self {
        Self::new()
    }
}
