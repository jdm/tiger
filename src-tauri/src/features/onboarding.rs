use log::error;
use serde::{Deserialize, Serialize};
use squeak::Response;
use std::fs::File;
use std::path::Path;
use std::sync::mpsc::channel;

use crate::app::TigerApp;
use crate::state::State;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum OnboardingStep {
    #[default]
    NotStarted,
    ImportFrame,
    CreateAnimation,
    PlaceFrameOnTimeline,
    Completed,
}

#[derive(Default, Deserialize, Serialize, Eq, PartialEq)]
struct OnboardingData {
    onboarding_complete: bool,
}

pub fn init<A: TigerApp + Send>(app: A) {
    let state_handle = app.state();
    let mut state = state_handle.lock();
    let onboarding_file = app.paths().lock().onboarding_file.clone();

    match read_from_disk(&onboarding_file) {
        Ok(data) => {
            if data.onboarding_complete {
                state.set_onboarding_step(OnboardingStep::Completed);
            }
        }
        Err(e) => error!("Error while reading onboarding file: {e}"),
    };

    let (tx, rx) = channel();
    state
        .onboarding_step_delegate()
        .subscribe(move |onboarding_step| {
            let onboarding_data = OnboardingData {
                onboarding_complete: *onboarding_step == OnboardingStep::Completed,
            };
            tx.send(onboarding_data).ok();
            Response::StaySubscribed
        });

    std::thread::spawn(move || loop {
        let Ok(onboarding_data) = rx.recv() else { break };
        if let Err(e) = write_to_disk(&onboarding_data, &onboarding_file) {
            error!("Error while saving list of recently opened documents: {e}");
        }
    });
}

impl State {
    pub fn advance_onboarding(&mut self) {
        self.set_onboarding_step(self.compute_onboarding_step());
    }

    fn compute_onboarding_step(&self) -> OnboardingStep {
        if self.onboarding_step() == OnboardingStep::Completed {
            return OnboardingStep::Completed;
        }

        let has_document = self.documents_iter().count() != 0;

        let has_frame = self
            .documents_iter()
            .flat_map(|d| d.sheet().frames_iter())
            .count()
            != 0;

        let has_animation = self
            .documents_iter()
            .flat_map(|d| d.sheet().animations_iter())
            .count()
            != 0;

        let has_keyframe = self
            .documents_iter()
            .flat_map(|d| d.sheet().animations_iter())
            .flat_map(|(_, a)| a.sequences_iter())
            .any(|(_, s)| s.num_keyframes() != 0);

        if !has_document {
            OnboardingStep::NotStarted
        } else if !has_frame {
            OnboardingStep::ImportFrame
        } else if !has_animation {
            OnboardingStep::CreateAnimation
        } else if !has_keyframe {
            OnboardingStep::PlaceFrameOnTimeline
        } else {
            OnboardingStep::Completed
        }
    }
}

fn write_to_disk(data: &OnboardingData, destination: &Path) -> Result<(), std::io::Error> {
    let file = File::create(destination)?;
    serde_json::to_writer_pretty(file, data)?;
    Ok(())
}

fn read_from_disk(source: &Path) -> Result<OnboardingData, std::io::Error> {
    if !source.exists() {
        return Ok(OnboardingData::default());
    }
    let file = File::open(source)?;
    let data: OnboardingData = serde_json::from_reader(file)?;
    Ok(data)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{
        app::mock::TigerAppMock,
        dto::{self},
    };

    #[test]
    fn can_complete_onboarding_flow() {
        let app = TigerAppMock::new();
        app.new_document("tmp");
        assert_eq!(
            app.client_state().onboarding_step,
            dto::OnboardingStep::ImportFrame
        );
        app.import_frames(vec!["frame.png"]);
        assert_eq!(
            app.client_state().onboarding_step,
            dto::OnboardingStep::CreateAnimation
        );
        app.create_animation();
        assert_eq!(
            app.client_state().onboarding_step,
            dto::OnboardingStep::PlaceFrameOnTimeline
        );
        app.begin_drag_and_drop_frame("frame.png");
        app.drop_frame_on_timeline(dto::Direction::North, 0);
        assert_eq!(
            app.client_state().onboarding_step,
            dto::OnboardingStep::Completed
        );
    }

    #[tokio::test]
    async fn can_skip_onboarding_flow_by_opening_a_sheet() {
        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        assert_eq!(
            app.client_state().onboarding_step,
            dto::OnboardingStep::Completed
        );
        app.new_document("tmp");
        assert_eq!(
            app.client_state().onboarding_step,
            dto::OnboardingStep::Completed
        );
    }

    #[test]
    fn reads_onboarding_data() {
        let app = TigerAppMock::new_uninitialized();

        let onboarding_file = app.paths().lock().onboarding_file.clone();
        let onboarding_data = OnboardingData {
            onboarding_complete: true,
        };

        std::fs::write(
            onboarding_file,
            serde_json::to_string(&onboarding_data).unwrap(),
        )
        .unwrap();

        app.init();
        assert_eq!(
            app.client_state().onboarding_step,
            dto::OnboardingStep::Completed
        );
    }

    #[tokio::test]
    async fn writes_onboarding_data_to_disk() {
        let app = TigerAppMock::new();
        let onboarding_file = app.paths().lock().onboarding_file.clone();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        app.assert_eventually(|| {
            let Ok(onboarding_data) = read_from_disk(&onboarding_file) else {
                return false
            };
            onboarding_data
                == OnboardingData {
                    onboarding_complete: true,
                }
        });
    }
}
