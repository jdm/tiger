use notify_debouncer_mini::{notify::*, *};
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::mpsc::*;
use std::time::Duration;

use crate::state::*;

pub struct FileWatcher {
    debouncer: Debouncer<RecommendedWatcher>,
    watched_files: HashSet<PathBuf>,
}

impl FileWatcher {
    pub fn init() -> (Self, Receiver<DebounceEventResult>) {
        let (sender, receiver) = channel();
        let file_watcher = Self::new(sender);
        (file_watcher, receiver)
    }

    fn new(event_sink: Sender<DebounceEventResult>) -> FileWatcher {
        let debouncer = new_debouncer(Duration::from_millis(200), None, event_sink).unwrap();
        FileWatcher {
            debouncer,
            watched_files: HashSet::new(),
        }
    }

    pub fn update_watched_files(&mut self, app_state: &AppState) {
        let app = app_state.0.lock().unwrap();

        let all_relevant_files = app
            .documents_iter()
            .flat_map(|d| d.sheet().frames_iter())
            .map(|f| f.source().to_owned())
            .collect::<HashSet<_>>();

        let files_to_unwatch = self
            .watched_files
            .difference(&all_relevant_files)
            .map(|f| f.to_owned())
            .collect::<HashSet<_>>();

        for file in files_to_unwatch {
            self.watched_files.remove(&file);
            if self.debouncer.watcher().unwatch(&file).is_err() {
                println!("Error removing file watch for {:?}", &file);
            }
        }

        let files_to_watch = all_relevant_files
            .difference(&self.watched_files)
            .map(|f| f.to_owned())
            .collect::<HashSet<_>>();

        for file in files_to_watch {
            self.watched_files.insert(file.to_owned());
            if self
                .debouncer
                .watcher()
                .watch(&file, RecursiveMode::NonRecursive)
                .is_err()
            {
                println!("Error adding file watch for {:?}", &file);
            }
        }
    }
}
