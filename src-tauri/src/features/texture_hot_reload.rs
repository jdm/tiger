use parking_lot::RwLock;
use std::{sync::Arc, time::Duration};

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

    std::thread::spawn({
        let file_watcher = file_watcher.clone();
        move || loop {
            file_watcher.write().update_watched_files();
            std::thread::sleep(PERIOD);
        }
    });

    std::thread::spawn(move || loop {
        if let Ok(Ok(events)) = events_receiver.recv() {
            for event in events {
                app.emit_all(
                    dto::EVENT_INVALIDATE_TEXTURE,
                    dto::TextureInvalidationEvent { path: event.path },
                );
            }
        }
    });

    TextureHotReloadInfo {
        #[cfg(test)]
        file_watcher,
    }
}

#[cfg(test)]
mod tests {
    use retry::{delay::Fixed, retry};

    use super::*;
    use crate::app::mock::TigerAppMock;

    #[test]
    fn emits_invalidate_event() {
        let dir = std::env::current_dir().unwrap();
        let frame = dir.join("test-output/emits_invalidate_event.png");
        let before_frame = dir.join("test-data/samurai-dead-all.png");
        let after_frame = dir.join("test-data/samurai-attack-north.png");

        let app = TigerAppMock::new();
        app.new_document("test.tiger");

        std::fs::copy(&before_frame, &frame).unwrap();
        app.import_frames(vec![frame.clone()]);
        let watching_changes = retry(Fixed::from(PERIOD).take(10), || {
            if app
                .texture_hot_reload_info()
                .file_watcher
                .read()
                .is_watching(&frame)
            {
                Ok(())
            } else {
                Err(())
            }
        });
        assert!(watching_changes.is_ok());

        assert!(app
            .events()
            .iter()
            .all(|(event, _)| event != dto::EVENT_INVALIDATE_TEXTURE));

        std::fs::copy(after_frame, &frame).unwrap();

        let expected_payload = dto::TextureInvalidationEvent { path: frame };
        let invalidation_event_emitted = retry(Fixed::from(PERIOD).take(10), || {
            app.events()
                .into_iter()
                .any(|(event, payload)| {
                    event.as_str() == dto::EVENT_INVALIDATE_TEXTURE
                        && match serde_json::from_value::<dto::TextureInvalidationEvent>(payload) {
                            Ok(payload) => payload == expected_payload,
                            Err(_) => false,
                        }
                })
                .then_some(())
                .ok_or(())
        });
        assert!(invalidation_event_emitted.is_ok());
    }
}
