use log::error;
use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path, thread, time::Duration};

use crate::{
    app::TigerApp,
    dto::{self, StateTrim},
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum UpdateStep {
    #[default]
    Idle,
    UpdateAvailable,
    UpdateRequested,
    InstallingUpdate,
}

#[derive(Deserialize, Serialize, Eq, PartialEq)]
struct UpdateData {
    last_used_version: semver::Version,
}

pub fn init<A: TigerApp + Clone + Send + Sync + 'static>(app: A) {
    let updates_file = app.paths().lock().updates_file.clone();

    thread::Builder::new()
        .name("app-update-checks-thread".to_owned())
        .spawn({
            let app = app.clone();
            move || {
                loop {
                    if app.is_startup_complete() {
                        break;
                    } else {
                        thread::sleep(Duration::from_secs(1));
                    }
                }

                let current_version = app.version();
                let last_used_version = read_from_disk(&updates_file).map(|d| d.last_used_version);
                let upgraded = matches!(
                    last_used_version.as_ref().map(|v| *v < current_version),
                    Ok(true)
                );

                if upgraded {
                    app.emit_all(
                        dto::EVENT_APP_UPDATE_SUCCESS,
                        dto::UpdateSuccess {
                            version_number: current_version.clone(),
                        },
                    );
                }

                if upgraded || last_used_version.is_err() {
                    if let Err(e) = write_to_disk(
                        &UpdateData {
                            last_used_version: current_version,
                        },
                        updates_file,
                    ) {
                        error!("Error while writing last used version to disk: {e}");
                    }
                }

                loop {
                    let update_step = app.state().lock().update_step();
                    match update_step {
                        UpdateStep::Idle => {
                            if !check_update(app.clone()) {
                                thread::sleep(Duration::from_secs(60 * 60));
                            }
                        }
                        UpdateStep::UpdateAvailable => {
                            thread::sleep(Duration::from_millis(100));
                        }
                        UpdateStep::UpdateRequested => {
                            if app.state().lock().should_update() {
                                install_update(app.clone());
                            } else {
                                thread::sleep(Duration::from_millis(100));
                            }
                        }
                        UpdateStep::InstallingUpdate => {
                            thread::sleep(Duration::from_millis(100));
                        }
                    }
                }
            }
        })
        .unwrap();
}

fn check_update<A: TigerApp>(app: A) -> bool {
    let result = app.check_update();
    if result {
        app.patch_state(StateTrim::NoDocuments, |state| {
            state.set_update_step(UpdateStep::UpdateAvailable)
        });
    }
    result
}

fn install_update<A: TigerApp>(app: A) {
    app.patch_state(StateTrim::NoDocuments, |state| {
        state.set_update_step(UpdateStep::InstallingUpdate);
    });
    match app.install_update() {
        Ok(()) => (),
        Err(e) => {
            app.emit_all(dto::EVENT_APP_UPDATE_ERROR, dto::UpdateError { details: e });
            app.patch_state(StateTrim::NoDocuments, |state| {
                state.set_update_step(UpdateStep::UpdateAvailable);
            });
        }
    }
}

fn write_to_disk<P: AsRef<Path>>(data: &UpdateData, destination: P) -> Result<(), std::io::Error> {
    let file = File::create(destination.as_ref())?;
    serde_json::to_writer_pretty(file, data)?;
    Ok(())
}

fn read_from_disk<P: AsRef<Path>>(source: P) -> Result<UpdateData, std::io::Error> {
    let file = File::open(source.as_ref())?;
    let data: UpdateData = serde_json::from_reader(file)?;
    Ok(data)
}

#[cfg(test)]
mod tests {

    use retry::{delay::Fixed, retry};
    use std::path::PathBuf;

    use super::*;
    use crate::{
        app::mock::{TigerAppMock, TigerAppMockBuilder},
        dto,
    };

    #[tokio::test]
    async fn can_install_update() {
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

        app.request_install_update();
        let installing_update = retry(Fixed::from_millis(100).take(100), || {
            match app.client_state().update_step {
                dto::UpdateStep::InstallingUpdate => Ok(()),
                other_state => Err(other_state),
            }
        });
        assert_eq!(installing_update, Ok(()));
    }

    #[tokio::test]
    async fn can_request_update_and_save_changes() {
        let sheet_file = PathBuf::from("test-output/can_request_update_and_save_changes.tiger");
        std::fs::copy("test-data/samurai.tiger", &sheet_file).unwrap();

        let app = TigerAppMock::new();
        app.open_documents(vec![sheet_file]).await;
        app.delete_animation("idle");

        let found_update = retry(Fixed::from_millis(100).take(100), || {
            match app.client_state().update_step {
                dto::UpdateStep::UpdateAvailable => Ok(()),
                other_state => Err(other_state),
            }
        });
        assert_eq!(found_update, Ok(()));

        app.request_install_update();
        thread::sleep(Duration::from_millis(500));
        assert_eq!(
            app.client_state().update_step,
            dto::UpdateStep::UpdateRequested
        );

        app.save().await;
        let installing_update = retry(Fixed::from_millis(100).take(100), || {
            match app.client_state().update_step {
                dto::UpdateStep::InstallingUpdate => Ok(()),
                other_state => Err(other_state),
            }
        });
        assert_eq!(installing_update, Ok(()));
    }

    #[tokio::test]
    async fn can_request_update_and_discard_changes() {
        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        app.delete_animation("idle");

        let found_update = retry(Fixed::from_millis(100).take(100), || {
            match app.client_state().update_step {
                dto::UpdateStep::UpdateAvailable => Ok(()),
                other_state => Err(other_state),
            }
        });
        assert_eq!(found_update, Ok(()));

        app.request_install_update();
        thread::sleep(Duration::from_millis(500));
        assert_eq!(
            app.client_state().update_step,
            dto::UpdateStep::UpdateRequested
        );

        app.close_without_saving();
        let installing_update = retry(Fixed::from_millis(100).take(100), || {
            match app.client_state().update_step {
                dto::UpdateStep::InstallingUpdate => Ok(()),
                other_state => Err(other_state),
            }
        });
        assert_eq!(installing_update, Ok(()));
    }

    #[tokio::test]
    async fn can_request_update_and_cancel() {
        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        app.delete_animation("idle");

        let found_update = retry(Fixed::from_millis(100).take(100), || {
            match app.client_state().update_step {
                dto::UpdateStep::UpdateAvailable => Ok(()),
                other_state => Err(other_state),
            }
        });
        assert_eq!(found_update, Ok(()));

        app.request_install_update();
        thread::sleep(Duration::from_millis(500));
        assert_eq!(
            app.client_state().update_step,
            dto::UpdateStep::UpdateRequested
        );

        app.cancel_close_document();
        assert_eq!(
            app.client_state().update_step,
            dto::UpdateStep::UpdateAvailable
        );
    }

    #[tokio::test]
    async fn write_current_version_to_disk() {
        let new_version = semver::Version::parse("0.10.0").unwrap();
        let app_builder = TigerAppMockBuilder::new().with_version(new_version.clone());
        let updates_file = app_builder.paths().updates_file.clone();
        std::fs::remove_file(&updates_file).ok();
        let _app = app_builder.build();

        let wrote_to_disk = retry(Fixed::from_millis(500).take(10), || {
            let Ok(update_data) = read_from_disk(&updates_file) else {
                return Err("Read error");
            };
            let expected_data = UpdateData {
                last_used_version: new_version.clone(),
            };
            match update_data == expected_data {
                true => Ok(()),
                false => Err("Content mismatch: {update_data}"),
            }
        });
        assert_eq!(Ok(()), wrote_to_disk);
    }

    #[tokio::test]
    async fn advertises_new_version() {
        let old_version = semver::Version::parse("0.9.0").unwrap();
        let new_version = semver::Version::parse("0.10.0").unwrap();

        let app_builder = TigerAppMockBuilder::new().with_version(new_version.clone());
        write_to_disk(
            &UpdateData {
                last_used_version: old_version,
            },
            &app_builder.paths().updates_file,
        )
        .unwrap();
        let app = app_builder.build();

        let emitted_event = retry(Fixed::from_millis(100).take(100), || {
            let expected_payload = dto::UpdateSuccess {
                version_number: new_version.clone(),
            };
            app.events()
                .into_iter()
                .any(|(event, payload)| {
                    event.as_str() == dto::EVENT_APP_UPDATE_SUCCESS
                        && match serde_json::from_value::<dto::UpdateSuccess>(payload) {
                            Ok(payload) => payload == expected_payload,
                            Err(_) => false,
                        }
                })
                .then_some(())
                .ok_or(())
        });
        assert_eq!(emitted_event, Ok(()));
    }
}
