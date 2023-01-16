use std::time::Duration;

use crate::{
    app::TigerApp,
    dto,
    utils::{file_watcher::FileWatcher, texture_list::TextureList},
};

pub fn init<A: TigerApp + Send + Clone + 'static>(app: A, period: Duration) {
    let (mut file_watcher, events_receiver) = FileWatcher::new({
        let app = app.clone();
        move || {
            let state_handle = app.state();
            let state = state_handle.lock();
            state.list_textures()
        }
    });

    std::thread::spawn(move || loop {
        file_watcher.update_watched_files();
        std::thread::sleep(period);
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
}

#[cfg(test)]
mod tests {
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

        std::fs::copy(before_frame, &frame).unwrap();
        app.import_frames(vec![frame.clone()]);
        app.wait_for_periodic_scans();

        assert!(app
            .events()
            .iter()
            .all(|(event, _)| event != dto::EVENT_INVALIDATE_TEXTURE));
        std::fs::copy(after_frame, &frame).unwrap();
        app.wait_for_periodic_scans();

        let expected_payload = dto::TextureInvalidationEvent { path: frame };
        assert!(app
            .events()
            .into_iter()
            .any(
                |(event, payload)| event.as_str() == dto::EVENT_INVALIDATE_TEXTURE
                    && match serde_json::from_value::<dto::TextureInvalidationEvent>(payload) {
                        Ok(payload) => payload == expected_payload,
                        Err(_) => false,
                    }
            ));
    }
}
