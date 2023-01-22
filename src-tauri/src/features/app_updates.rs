use std::{thread, time::Duration};

use log::error;

use crate::{app::TigerApp, dto::StateTrim};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum UpdateStep {
    #[default]
    Idle,
    CheckingUpdate,
    UpdateAvailable,
    InstallingUpdate,
}

pub fn init<A: TigerApp + Clone + Send + Sync + 'static>(app: A) {
    thread::spawn({
        let app = app.clone();
        move || {
            loop {
                if app.is_startup_complete() {
                    break;
                } else {
                    thread::sleep(Duration::from_secs(1));
                }
            }
            loop {
                if app.state().lock().update_step() == UpdateStep::Idle {
                    app.patch_state(StateTrim::NoDocuments, |state| {
                        if state.update_step() == UpdateStep::Idle {
                            state.set_update_step(UpdateStep::CheckingUpdate)
                        }
                    });
                    match app.check_update() {
                        Ok(true) => app.patch_state(StateTrim::NoDocuments, |state| {
                            if state.update_step() == UpdateStep::CheckingUpdate {
                                state.set_update_step(UpdateStep::UpdateAvailable)
                            }
                        }),
                        Ok(false) => {
                            app.patch_state(StateTrim::NoDocuments, |state| {
                                if state.update_step() == UpdateStep::CheckingUpdate {
                                    state.set_update_step(UpdateStep::Idle)
                                }
                            });
                        }
                        Err(e) => {
                            error!("Failed to check for update: {e}");
                        }
                    }
                }
                thread::sleep(Duration::from_secs(60 * 60));
            }
        }
    });
}

#[cfg(test)]
mod tests {

    use retry::{delay::Fixed, retry};

    use super::*;
    use crate::{app::mock::TigerAppMockBuilder, dto};

    #[tokio::test]
    async fn can_auto_install_updates() {
        let app = TigerAppMockBuilder::new().with_startup_guard().build();

        thread::sleep(Duration::from_millis(500));
        assert_eq!(app.client_state().update_step, dto::UpdateStep::Idle);

        app.finalize_startup().await;
        let found_update = retry(Fixed::from_millis(100).take(100), || {
            match app.client_state().update_step {
                dto::UpdateStep::UpdateAvailable => Ok(()),
                other_state => Err(other_state),
            }
        });
        assert_eq!(found_update, Ok(()));

        app.install_update();
        assert_eq!(
            app.client_state().update_step,
            dto::UpdateStep::InstallingUpdate
        );
    }
}
