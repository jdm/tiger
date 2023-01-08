use std::{collections::HashSet, time::Duration};

use crate::sheet;
use crate::utils::file_watcher::FileWatcher;
use crate::TigerApp;

pub fn init<A: TigerApp + Send + Clone + 'static>(app: A, period: Duration) {
    let (mut file_watcher, events_receiver) = FileWatcher::new({
        let app = app.clone();
        move || {
            let state_handle = app.state();
            let state = state_handle.0.lock();
            state
                .documents_iter()
                .flat_map(|d| d.export_settings_edit())
                .map(|s| match s {
                    sheet::ExportSettings::Template(s) => s.template_file().to_owned(),
                })
                .collect::<HashSet<_>>()
        }
    });

    std::thread::spawn(move || loop {
        file_watcher.update_watched_files();
        std::thread::sleep(period);
    });

    std::thread::spawn(move || loop {
        if let Ok(Ok(_)) = events_receiver.recv() {
            app.replace_state();
        }
    });
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::{dto::ExportSettingsError, mock::TigerAppMock};

    #[tokio::test]
    async fn detects_template_errors() {
        let test_template_path = PathBuf::from("test-output/detects_template_errors.template");
        let bad_template_path = PathBuf::from("test-data/malformed.template")
            .canonicalize()
            .unwrap();

        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger".into()])
            .await;

        app.begin_export_as();
        let good_template_path = app.client_state().documents[0]
            .export_settings_being_edited
            .as_ref()
            .unwrap()
            .template_file
            .canonicalize()
            .unwrap();

        app.wait_for_periodic_scans();
        assert!(app.client_state().documents[0]
            .export_settings_validation
            .as_ref()
            .unwrap()
            .template_file_error
            .is_none());

        std::fs::copy(good_template_path, &test_template_path).unwrap();
        app.set_export_template_file(test_template_path.canonicalize().unwrap());
        app.wait_for_periodic_scans();
        assert!(app.client_state().documents[0]
            .export_settings_validation
            .as_ref()
            .unwrap()
            .template_file_error
            .is_none());

        std::fs::copy(bad_template_path, &test_template_path).unwrap();
        app.wait_for_periodic_scans();
        assert!(matches!(
            app.client_state().documents[0]
                .export_settings_validation
                .as_ref()
                .unwrap()
                .template_file_error,
            Some(ExportSettingsError::TemplateError(_))
        ));
    }
}
