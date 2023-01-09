use std::time::Duration;

use crate::document::clipboard_manifest;
use crate::dto::StateTrim;
use crate::TigerApp;

pub fn init<A: TigerApp + Send + 'static>(app: A, period: Duration) {
    std::thread::spawn(move || loop {
        std::thread::sleep(period);

        let clipboard_content = app.read_clipboard();

        let new_manifest = match clipboard_content {
            None => None,
            Some(s) => clipboard_manifest(s),
        };

        {
            let state_handle = app.state();
            let state = state_handle.lock();
            if *state.clipboard_manifest() == new_manifest {
                continue;
            }
        }

        app.patch_state(StateTrim::NoDocuments, |state| {
            state.set_clipboard_manifest(new_manifest);
        });
    });
}

#[cfg(test)]
mod test {
    use crate::{
        dto::{ClipboardManifest, Direction},
        mock::TigerAppMock,
        TigerApp,
    };

    #[tokio::test]
    async fn updates_clipboard_manifest() {
        let app = TigerAppMock::new();

        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        assert_eq!(app.client_state().clipboard_manifest, None);

        app.select_animation("idle", false, false);
        app.copy();
        app.wait_for_periodic_scans();
        assert_eq!(
            app.client_state().clipboard_manifest,
            Some(ClipboardManifest::Animations)
        );

        app.edit_animation("idle");
        app.select_keyframe(Direction::North, 0, false, false);
        app.copy();
        app.wait_for_periodic_scans();
        assert_eq!(
            app.client_state().clipboard_manifest,
            Some(ClipboardManifest::Keyframes)
        );

        app.select_hitbox("weak", false, false);
        app.copy();
        app.wait_for_periodic_scans();
        assert_eq!(
            app.client_state().clipboard_manifest,
            Some(ClipboardManifest::Hitboxes)
        );

        app.write_clipboard("random clipboard data");
        app.wait_for_periodic_scans();
        assert_eq!(app.client_state().clipboard_manifest, None);
    }
}
