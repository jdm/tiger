use once_cell::sync::OnceCell;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Paths {
    pub log_file: PathBuf,
    pub recent_documents_file: PathBuf,
}

static PATHS: OnceCell<Paths> = OnceCell::new();

fn get() -> &'static Paths {
    PATHS.get().unwrap()
}

pub fn init() {
    let project_dirs = directories::ProjectDirs::from("org", "Permafrost", "Tiger").unwrap();
    let data_local_dir = project_dirs.data_local_dir();
    std::fs::create_dir_all(data_local_dir).unwrap();
    PATHS
        .set(Paths {
            log_file: data_local_dir.join("tiger.log"),
            recent_documents_file: data_local_dir.join("recent-documents.json"),
        })
        .unwrap();
}

pub fn log_file() -> &'static Path {
    get().log_file.as_path()
}

pub fn recent_documents_file() -> &'static Path {
    get().recent_documents_file.as_path()
}
