use notify_debouncer_mini::{notify::*, *};
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::mpsc::*;
use std::time::Duration;

pub type ListFiles = dyn Fn() -> HashSet<PathBuf> + Send + Sync + 'static;
pub type BoxedListFiles = Box<ListFiles>;

pub struct FileWatcher {
    debouncer: Debouncer<RecommendedWatcher>,
    list_files_to_watch: BoxedListFiles,
    watched_files: HashSet<PathBuf>,
}

impl FileWatcher {
    pub fn new<F: Fn() -> HashSet<PathBuf> + Send + Sync + 'static>(
        list_files_to_watch: F,
    ) -> (Self, Receiver<DebounceEventResult>) {
        let (sender, receiver) = channel();

        let delay = Duration::from_millis(if cfg!(test) { 0 } else { 200 });
        let debouncer = new_debouncer(delay, None, sender).unwrap();
        let file_watcher = FileWatcher {
            debouncer,
            list_files_to_watch: Box::new(list_files_to_watch),
            watched_files: HashSet::new(),
        };

        (file_watcher, receiver)
    }

    #[cfg(test)]
    pub fn is_watching<P: AsRef<std::path::Path>>(&self, path: P) -> bool {
        self.watched_files.contains(path.as_ref())
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
            if self
                .debouncer
                .watcher()
                .watch(&file, RecursiveMode::NonRecursive)
                .is_err()
            {
                println!("Error adding file watch for {:?}", &file);
            } else {
                self.watched_files.insert(file.to_owned());
            }
        }
    }
}
