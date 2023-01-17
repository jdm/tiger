use std::time::Duration;

use crate::{app::TigerApp, document::clipboard_manifest, dto::StateTrim};

static PERIOD: Duration = Duration::from_millis(100);

pub fn init<A: TigerApp + Send + 'static>(app: A) {
    std::thread::spawn(move || loop {
        std::thread::sleep(PERIOD);

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
mod tests {
    use retry::{delay::Fixed, retry};

    use super::*;
    use crate::{
        app::{mock::TigerAppMock, TigerApp},
        dto::{ClipboardManifest, Direction},
    };

    #[tokio::test]
    async fn updates_clipboard_manifest() {
        let app = TigerAppMock::new();

        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        assert_eq!(app.client_state().clipboard_manifest, None);

        app.select_animation("idle", false, false);
        app.copy();

        let check_manifest = |desired_manifest: Option<ClipboardManifest>| {
            retry(Fixed::from(PERIOD).take(100), || {
                match (&desired_manifest, app.client_state().clipboard_manifest) {
                    (None, None) => Ok(()),
                    (None, Some(m)) => Err(Some(m)),
                    (Some(_), None) => Err(None),
                    (Some(ref m), Some(n)) if *m == n => Ok(()),
                    (Some(_), Some(n)) => Err(Some(n)),
                }
            })
        };
        assert_eq!(check_manifest(Some(ClipboardManifest::Animations)), Ok(()));

        app.edit_animation("idle");
        app.select_keyframe(Direction::North, 0, false, false);
        app.copy();
        assert_eq!(check_manifest(Some(ClipboardManifest::Keyframes)), Ok(()));

        app.select_hitbox("weak", false, false);
        app.copy();
        assert_eq!(check_manifest(Some(ClipboardManifest::Hitboxes)), Ok(()));

        app.write_clipboard("random clipboard data");
        assert_eq!(check_manifest(None), Ok(()));
    }
}
