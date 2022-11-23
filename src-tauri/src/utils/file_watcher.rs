use notify_debouncer_mini::{notify::*, *};
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::mpsc::*;
use std::time::Duration;

pub struct FileWatcher<F: Fn() -> HashSet<PathBuf>> {
    debouncer: Debouncer<RecommendedWatcher>,
    list_files_to_watch: F,
    watched_files: HashSet<PathBuf>,
}

impl<F: Fn() -> HashSet<PathBuf>> FileWatcher<F> {
    pub fn new(list_files_to_watch: F) -> (Self, Receiver<DebounceEventResult>) {
        let (sender, receiver) = channel();

        let debouncer = new_debouncer(Duration::from_millis(200), None, sender).unwrap();
        let file_watcher = FileWatcher {
            debouncer,
            list_files_to_watch,
            watched_files: HashSet::new(),
        };

        (file_watcher, receiver)
    }

    pub fn update_watched_files(&mut self) {
        let all_relevant_files = (self.list_files_to_watch)();

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
