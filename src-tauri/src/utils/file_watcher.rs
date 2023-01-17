use log::error;
use notify_debouncer_mini::{notify::*, *};
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::mpsc::*;
use std::time::Duration;

pub type ListFiles = dyn Fn() -> HashSet<PathBuf> + Send + Sync + 'static;
pub type BoxedListFiles = Box<ListFiles>;

pub struct FileWatcher {
    debouncer: Debouncer<RecommendedWatcher>,
    list_relevant_files: BoxedListFiles,
    watched_directories: HashSet<PathBuf>,
}

struct EventFilter {
    list_relevant_files: BoxedListFiles,
    sender: Sender<DebounceEventResult>,
}

impl DebounceEventHandler for EventFilter {
    fn handle_event(&mut self, event: DebounceEventResult) {
        let all_relevant_files = (self.list_relevant_files)();
        match event {
            Err(e) => error!("Filesystem watch error: `{e:?}`"),
            Ok(mut events) => {
                events.retain(|e| all_relevant_files.contains(&e.path));
                self.sender.send(Ok(events)).unwrap();
            }
        }
    }
}

impl FileWatcher {
    pub fn new<F: Fn() -> HashSet<PathBuf> + Clone + Send + Sync + 'static>(
        list_relevant_files: F,
    ) -> (Self, Receiver<DebounceEventResult>) {
        let (sender, receiver) = channel();

        let event_filter = EventFilter {
            list_relevant_files: Box::new(list_relevant_files.clone()),
            sender,
        };

        let delay = Duration::from_millis(if cfg!(test) { 0 } else { 200 });
        let debouncer = new_debouncer(delay, None, event_filter).unwrap();
        let file_watcher = FileWatcher {
            debouncer,
            list_relevant_files: Box::new(list_relevant_files),
            watched_directories: HashSet::new(),
        };

        (file_watcher, receiver)
    }

    #[cfg(test)]
    pub fn is_watching<P: AsRef<std::path::Path>>(&self, path: P) -> bool {
        match path.as_ref().parent() {
            Some(directory) => self.watched_directories.contains(directory),
            None => false,
        }
    }

    pub fn update_watched_files(&mut self) {
        let relevant_files = (self.list_relevant_files)();
        let relevant_directories = relevant_files
            .iter()
            .filter_map(|f| f.parent().map(PathBuf::from))
            .collect::<HashSet<_>>();

        let directories_to_unwatch = self
            .watched_directories
            .difference(&relevant_directories)
            .map(|f| f.to_owned())
            .collect::<HashSet<_>>();

        for directory in directories_to_unwatch {
            self.watched_directories.remove(&directory);
            if self.debouncer.watcher().unwatch(&directory).is_err() {
                println!("Error removing filesystem watch for {:?}", &directory);
            }
        }

        let directories_to_watch = relevant_directories
            .difference(&self.watched_directories)
            .map(|f| f.to_owned())
            .collect::<HashSet<_>>();

        for directory in directories_to_watch {
            if self
                .debouncer
                .watcher()
                .watch(&directory, RecursiveMode::NonRecursive)
                .is_err()
            {
                println!("Error adding filesystem watch for {:?}", &directory);
            } else {
                self.watched_directories.insert(directory.to_owned());
            }
        }
    }
}
