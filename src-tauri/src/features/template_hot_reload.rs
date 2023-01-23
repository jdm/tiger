use parking_lot::RwLock;
use std::{collections::HashSet, sync::Arc, thread, time::Duration};

use crate::{app::TigerApp, sheet, utils::file_watcher::FileWatcher};

#[cfg(not(test))]
static PERIOD: Duration = Duration::from_millis(1_000);
#[cfg(test)]
static PERIOD: Duration = Duration::from_millis(100);

#[derive(Clone)]
pub struct TemplateHotReloadInfo {
    #[cfg(test)]
    file_watcher: Arc<RwLock<FileWatcher>>,
}

pub fn init<A: TigerApp + Send + Sync + Clone + 'static>(app: A) -> TemplateHotReloadInfo {
    let (file_watcher, events_receiver) = FileWatcher::new({
        let app = app.clone();
        move || {
            let state_handle = app.state();
            let state = state_handle.lock();
            state
                .documents_iter()
                .flat_map(|d| d.export_settings_edit())
                .map(|s| match s {
                    sheet::ExportSettings::Template(s) => s.template_file().to_owned(),
                })
                .collect::<HashSet<_>>()
        }
    });
    let file_watcher = Arc::new(RwLock::new(file_watcher));

    thread::Builder::new()
        .name("template-hot-reload-update-watcher-thread".to_owned())
        .spawn({
            #[cfg(test)]
            let file_watcher = file_watcher.clone();
            move || loop {
                file_watcher.write().update_watched_files();
                thread::sleep(PERIOD);
            }
        })
        .unwrap();

    thread::Builder::new()
        .name("template-hot-reload-replace-state-thread".to_owned())
        .spawn(move || loop {
            if let Ok(Ok(_)) = events_receiver.recv() {
                app.replace_state();
            }
        })
        .unwrap();

    TemplateHotReloadInfo {
        #[cfg(test)]
        file_watcher,
    }
}

#[cfg(test)]
mod tests {
    use retry::{delay::Fixed, retry};
    use std::path::PathBuf;
    use sugar_path::SugarPath;

    use super::*;
    use crate::{app::mock::TigerAppMock, dto::ExportSettingsError};

    #[tokio::test]
    async fn detects_template_errors_on_file_change() {
        let test_template_path =
            PathBuf::from("test-output/detects_template_errors_on_file_change.template").resolve();
        let bad_template_path = PathBuf::from("test-data/malformed.template").resolve();

        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        app.begin_export_as();

        let good_template_path = app
            .document()
            .export_settings_being_edited
            .as_ref()
            .and_then(|s| s.template_file.canonicalize().ok())
            .unwrap();
        std::fs::copy(good_template_path, &test_template_path).unwrap();
        app.set_export_template_file(&test_template_path);

        assert!(app
            .document()
            .export_settings_validation
            .as_ref()
            .unwrap()
            .template_file_error
            .is_none());

        let watching_changes = retry(Fixed::from(PERIOD).take(100), || {
            if app
                .template_hot_reload_info()
                .file_watcher
                .read()
                .is_watching(&test_template_path)
            {
                Ok(())
            } else {
                Err(())
            }
        });
        assert!(watching_changes.is_ok());

        assert!(app
            .document()
            .export_settings_validation
            .as_ref()
            .unwrap()
            .template_file_error
            .is_none());

        std::fs::copy(bad_template_path, &test_template_path).unwrap();
        let template_file_error = retry(Fixed::from(PERIOD).take(100), || {
            match app
                .document()
                .export_settings_validation
                .as_ref()
                .unwrap()
                .template_file_error
            {
                None => Err("No template file error"),
                Some(ref e) => Ok(e.clone()),
            }
        });

        assert!(matches!(
            template_file_error,
            Ok(ExportSettingsError::TemplateError(_))
        ));
    }
}
