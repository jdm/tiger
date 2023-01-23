use parking_lot::RwLock;
use std::{sync::Arc, thread, time::Duration};

use crate::{
    app::TigerApp,
    dto,
    utils::{file_watcher::FileWatcher, texture_list::TextureList},
};

#[cfg(not(test))]
static PERIOD: Duration = Duration::from_millis(1_000);
#[cfg(test)]
static PERIOD: Duration = Duration::from_millis(100);

#[derive(Clone)]
pub struct TextureHotReloadInfo {
    #[cfg(test)]
    file_watcher: Arc<RwLock<FileWatcher>>,
}

pub fn init<A: TigerApp + Send + Sync + Clone + 'static>(app: A) -> TextureHotReloadInfo {
    let (file_watcher, events_receiver) = FileWatcher::new({
        let app = app.clone();
        move || {
            let state_handle = app.state();
            let state = state_handle.lock();
            state.list_textures()
        }
    });
    let file_watcher = Arc::new(RwLock::new(file_watcher));

    thread::Builder::new()
        .name("texture-hot-reload-update-watcher-thread".to_owned())
        .spawn({
            let file_watcher = file_watcher.clone();
            move || loop {
                file_watcher.write().update_watched_files();
                thread::sleep(PERIOD);
            }
        })
        .unwrap();

    thread::Builder::new()
        .name("texture-hot-reload-event-thread".to_owned())
        .spawn(move || loop {
            if let Ok(Ok(events)) = events_receiver.recv() {
                for event in events {
                    app.emit_all(
                        dto::EVENT_INVALIDATE_TEXTURE,
                        dto::TextureInvalidation { path: event.path },
                    );
                }
            }
        })
        .unwrap();

    TextureHotReloadInfo {
        #[cfg(test)]
        file_watcher,
    }
}

#[cfg(test)]
mod tests {
    use retry::{delay::Fixed, retry};
    use std::path::Path;

    use super::*;
    use crate::app::mock::TigerAppMock;

    fn wait_for_file_watch<P: AsRef<Path>>(
        app: &TigerAppMock,
        frame: P,
    ) -> Result<(), retry::Error<()>> {
        retry(Fixed::from(PERIOD).take(100), || {
            if app
                .texture_hot_reload_info()
                .file_watcher
                .read()
                .is_watching(frame.as_ref())
            {
                Ok(())
            } else {
                Err(())
            }
        })
    }

    fn wait_for_invalidation_event<P: AsRef<Path>>(
        app: &TigerAppMock,
        frame: P,
    ) -> Result<(), retry::Error<()>> {
        let expected_payload = dto::TextureInvalidation {
            path: frame.as_ref().to_owned(),
        };
        retry(Fixed::from(PERIOD).take(100), || {
            app.events()
                .into_iter()
                .any(|(event, payload)| {
                    event.as_str() == dto::EVENT_INVALIDATE_TEXTURE
                        && match serde_json::from_value::<dto::TextureInvalidation>(payload) {
                            Ok(payload) => payload == expected_payload,
                            Err(_) => false,
                        }
                })
                .then_some(())
                .ok_or(())
        })
    }

    #[test]
    fn emits_invalidate_event_on_file_change() {
        let dir = std::env::current_dir().unwrap();
        let frame = dir.join("test-output/emits_invalidate_event_on_file_change.png");
        let before_frame = dir.join("test-data/samurai-dead-all.png");
        let after_frame = dir.join("test-data/samurai-attack-north.png");

        std::fs::copy(&before_frame, &frame).unwrap();

        let app = TigerAppMock::new();
        app.new_document("test.tiger");
        app.import_frames(vec![frame.clone()]);

        let watching_changes = wait_for_file_watch(&app, &frame);
        assert!(watching_changes.is_ok());

        assert!(app
            .events()
            .iter()
            .all(|(event, _)| event != dto::EVENT_INVALIDATE_TEXTURE));

        std::fs::copy(after_frame, &frame).unwrap();

        let invalidation_event_emitted = wait_for_invalidation_event(&app, frame);
        assert!(invalidation_event_emitted.is_ok());
    }

    #[test]
    fn emits_invalidate_event_on_file_add() {
        let dir = std::env::current_dir().unwrap();
        let frame = dir.join("test-output/emits_invalidate_event_on_file_add.png");
        let source_frame = dir.join("test-data/samurai-dead-all.png");

        std::fs::remove_file(&frame).ok();

        let app = TigerAppMock::new();
        app.new_document("test.tiger");
        app.import_frames(vec![&frame]);

        let watching_changes = wait_for_file_watch(&app, &frame);
        assert!(watching_changes.is_ok());

        std::fs::copy(source_frame, &frame).unwrap();

        let invalidation_event_emitted = wait_for_invalidation_event(&app, frame);
        assert!(invalidation_event_emitted.is_ok());
    }
}
