use json_patch::Patch;
use log::error;
use squeak::{Delegate, Observable};
use std::path::{Path, PathBuf};
use sugar_path::SugarPath;
use thiserror::Error;

use crate::{
    document::{ClipboardManifest, Document, DocumentError},
    dto::{self, StateTrim},
    features::{app_updates::UpdateStep, onboarding::OnboardingStep},
    utils::handle,
};

#[derive(Error, Debug)]
pub enum AppError {
    #[error("The requested document (`{0}`) is not currently opened.")]
    DocumentNotFound(PathBuf),
    #[error(transparent)]
    DocumentError(#[from] DocumentError),
}

pub type Handle = handle::Handle<State>;

#[derive(Debug, Default)]
pub struct State {
    documents: Vec<Document>,
    current_document: Option<PathBuf>,
    recent_documents: Observable<'static, Vec<PathBuf>>,
    errors: Vec<UserFacingError>,
    startup_finalized: bool,
    clipboard_manifest: Option<ClipboardManifest>,
    onboarding_step: Observable<'static, OnboardingStep>,
    update_step: UpdateStep,
    about_dialog_open: bool,
    opened_startup_documents: bool,
    exit_requested: bool,
}

#[derive(Debug)]
pub struct UserFacingError {
    pub key: uuid::Uuid,
    pub title: String,
    pub summary: String,
    pub details: String,
}

impl State {
    pub fn patch<F: FnOnce(&mut State)>(&mut self, state_trim: StateTrim, operation: F) -> Patch {
        let old_state: dto::State = self.to_dto(state_trim);
        operation(self);
        self.after_change();
        let new_state: dto::State = self.to_dto(state_trim);

        let old_json = serde_json::to_value(old_state);
        let new_json = serde_json::to_value(new_state);

        match (old_json, new_json) {
            (Ok(o), Ok(n)) => json_patch::diff(&o, &n),
            _ => {
                error!("App state serialization error");
                Patch(Vec::new())
            }
        }
    }

    fn after_change(&mut self) {
        self.advance_onboarding();
        self.advance_exit();
    }

    pub fn documents_iter(&self) -> impl Iterator<Item = &Document> {
        self.documents.iter()
    }

    pub fn documents_iter_mut(&mut self) -> impl Iterator<Item = &mut Document> {
        self.documents.iter_mut()
    }

    pub fn new_document<T: AsRef<Path>>(&mut self, path: T) {
        match self.document_mut(&path) {
            Some(d) => *d = Document::new(path.as_ref()),
            None => {
                let document = Document::new(path.as_ref());
                self.documents.push(document);
            }
        }
        self.focus_document(&path).unwrap();
        self.add_recent_document(path);
    }

    pub fn open_document(&mut self, document: Document) {
        let path = document.path().to_owned();
        if self.document(document.path()).is_none() {
            self.documents.push(document);
        }
        self.focus_document(&path).unwrap();
        self.add_recent_document(path);
        self.set_onboarding_step(OnboardingStep::Completed);
    }

    pub fn focus_document<T: AsRef<Path>>(&mut self, path: T) -> Result<(), AppError> {
        self.document(&path)
            .ok_or_else(|| AppError::DocumentNotFound(path.as_ref().to_owned()))?;
        self.current_document = Some(path.as_ref().to_owned());
        Ok(())
    }

    pub fn focus_next_document(&mut self) -> Result<(), AppError> {
        self.cycle_focused_document(false)
    }

    pub fn focus_previous_document(&mut self) -> Result<(), AppError> {
        self.cycle_focused_document(true)
    }

    fn cycle_focused_document(&mut self, backward: bool) -> Result<(), AppError> {
        let Some(current_index) = self
            .current_document
            .as_ref()
            .and_then(|p| self.documents_iter().position(|d| d.path() == p))
        else {
            return Ok(())
        };
        let delta: isize = if backward { -1 } else { 1 };
        let num_documents = self.documents.len();
        let new_index =
            (num_documents + current_index).saturating_add_signed(delta) % self.documents.len();
        let new_document = self.documents[new_index].path().to_owned();
        self.focus_document(new_document)
    }

    pub fn current_document(&self) -> Option<&Document> {
        match &self.current_document {
            None => None,
            Some(p) => self.documents.iter().find(|d| d.path() == p),
        }
    }

    pub fn current_document_mut(&mut self) -> Option<&mut Document> {
        self.current_document
            .clone()
            .and_then(|path| self.documents.iter_mut().find(|d| d.path() == path))
    }

    pub fn document<T: AsRef<Path>>(&mut self, path: T) -> Option<&Document> {
        self.documents.iter().find(|d| d.path() == path.as_ref())
    }

    pub fn document_mut<T: AsRef<Path>>(&mut self, path: T) -> Option<&mut Document> {
        self.documents
            .iter_mut()
            .find(|d| d.path() == path.as_ref())
    }

    pub fn relocate_document<T: AsRef<Path>, U: AsRef<Path>>(&mut self, from: T, to: U) {
        if from.as_ref() == to.as_ref() {
            return;
        }
        self.documents.retain(|d| d.path() != to.as_ref());
        if let Some(moved_document) = self.document_mut(&from) {
            moved_document.set_path(to.as_ref().to_owned());
        }
        if Some(from.as_ref()) == self.current_document.as_deref() {
            self.focus_document(&to).unwrap();
        }
        self.add_recent_document(to);
    }

    pub fn close_all_documents(&mut self) {
        for document in &mut self.documents {
            document.request_close();
        }
    }

    pub fn close_document<T: AsRef<Path>>(&mut self, path: T) {
        if let Some(index) = self
            .documents
            .iter()
            .position(|d| d.path() == path.as_ref())
        {
            self.documents.remove(index);
            self.current_document = if self.documents.is_empty() {
                None
            } else {
                Some(
                    self.documents[std::cmp::min(index, self.documents.len() - 1)]
                        .path()
                        .to_owned(),
                )
            };
        }
    }

    pub fn show_error_message(&mut self, title: String, summary: String, details: String) {
        self.errors.push(UserFacingError {
            key: uuid::Uuid::new_v4(),
            title,
            summary,
            details,
        });
    }

    pub fn error(&self) -> Option<&UserFacingError> {
        self.errors.first()
    }

    pub fn request_exit(&mut self) {
        self.exit_requested = true;
        self.close_all_documents();
        self.advance_exit();
    }

    pub fn cancel_close_document(&mut self) {
        self.exit_requested = false;
        if self.update_step == UpdateStep::UpdateRequested {
            self.update_step = UpdateStep::UpdateAvailable;
        }
        for document in &mut self.documents {
            document.cancel_close();
        }
    }

    pub fn advance_exit(&mut self) {
        let closable_documents: Vec<PathBuf> = self
            .documents
            .iter()
            .filter(|d| d.should_close())
            .map(|d| d.path().to_owned())
            .collect();
        for path in closable_documents {
            self.close_document(path);
        }
    }

    pub fn should_exit(&self) -> bool {
        self.exit_requested && self.documents.is_empty()
    }

    pub fn opened_startup_documents(&self) -> bool {
        self.opened_startup_documents
    }

    pub fn set_opened_startup_documents(&mut self, opened: bool) {
        self.opened_startup_documents = opened;
    }

    pub fn acknowledge_error(&mut self) {
        if !self.errors.is_empty() {
            self.errors.remove(0);
        }
    }

    pub fn open_about_dialog(&mut self) {
        self.about_dialog_open = true;
    }

    pub fn close_about_dialog(&mut self) {
        self.about_dialog_open = false;
    }

    pub fn is_about_dialog_open(&self) -> bool {
        self.about_dialog_open
    }

    fn add_recent_document<T: AsRef<Path>>(&mut self, path: T) {
        let path = path.as_ref().resolve();
        self.recent_documents.mutate(|d| {
            d.retain(|p| p.as_path() != path);
            d.insert(0, path);
            d.truncate(10);
        });
    }

    pub fn recent_documents(&self) -> impl Iterator<Item = &Path> {
        self.recent_documents.iter().map(|d| d.as_path())
    }

    pub fn set_recent_documents(&mut self, documents: Vec<PathBuf>) {
        self.recent_documents.mutate(|d| {
            *d = documents;
        });
    }

    pub fn recent_documents_delegate(&self) -> &Delegate<'static, Vec<PathBuf>> {
        self.recent_documents.delegate()
    }

    pub fn finalize_startup(&mut self) {
        self.startup_finalized = true;
    }

    pub fn is_startup_finalized(&self) -> bool {
        self.startup_finalized
    }

    pub fn set_clipboard_manifest(&mut self, new_manifest: Option<ClipboardManifest>) {
        self.clipboard_manifest = new_manifest;
    }

    pub fn clipboard_manifest(&self) -> &Option<ClipboardManifest> {
        &self.clipboard_manifest
    }

    pub fn set_onboarding_step(&mut self, new_onboarding_step: OnboardingStep) {
        if new_onboarding_step != self.onboarding_step() {
            self.onboarding_step.mutate(|s| *s = new_onboarding_step);
        }
    }

    pub fn onboarding_step(&self) -> OnboardingStep {
        *self.onboarding_step
    }

    pub fn onboarding_step_delegate(&self) -> &Delegate<'static, OnboardingStep> {
        self.onboarding_step.delegate()
    }

    pub fn request_update(&mut self) {
        if self.update_step == UpdateStep::UpdateAvailable {
            self.update_step = UpdateStep::UpdateRequested;
        }
        self.close_all_documents();
    }

    pub fn should_update(&self) -> bool {
        self.update_step == UpdateStep::UpdateRequested && self.documents.is_empty()
    }

    pub fn update_step(&self) -> UpdateStep {
        self.update_step
    }

    pub fn set_update_step(&mut self, new_update_step: UpdateStep) {
        self.update_step = new_update_step;
    }
}

#[cfg(test)]
mod tests {

    use std::path::PathBuf;

    use super::*;
    use crate::app::mock::TigerAppMock;

    #[tokio::test]
    async fn can_open_and_close_documents() {
        let app = TigerAppMock::new();

        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        assert_eq!(app.client_state().documents.len(), 1);
        assert_eq!(app.client_state().documents[0].name, "samurai.tiger");

        app.open_documents(vec!["test-data/flame.tiger"]).await;
        assert_eq!(app.client_state().documents.len(), 2);
        assert_eq!(app.client_state().documents[0].name, "samurai.tiger");
        assert_eq!(app.client_state().documents[1].name, "flame.tiger");

        app.close_document("test-data/flame.tiger");
        assert_eq!(app.client_state().documents.len(), 1);
        assert_eq!(app.client_state().documents[0].name, "samurai.tiger");
    }

    #[tokio::test]
    async fn can_open_documents_on_start() {
        let app = TigerAppMock::new();
        app.set_command_line_arguments(vec!["test-data/samurai.tiger"]);
        app.open_startup_documents().await;
        assert_eq!(app.client_state().documents.len(), 1);
        app.close_all_documents();
        assert!(app.client_state().documents.is_empty());
        app.open_startup_documents().await;
        assert!(app.client_state().documents.is_empty());
    }

    #[tokio::test]
    async fn shows_main_window_on_start() {
        let app = TigerAppMock::new();
        assert!(!app.is_main_window_visible());
        app.finalize_startup();
        assert!(app.is_main_window_visible());
    }

    #[tokio::test]
    async fn marks_startup_as_finalized() {
        let app = TigerAppMock::new();
        assert!(!app.client_state().startup_finalized);
        app.finalize_startup();
        assert!(app.client_state().startup_finalized);
    }

    #[tokio::test]
    async fn emits_event_for_open_errors() {
        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/missing-file.tiger"])
            .await;
        assert!(app
            .events()
            .into_iter()
            .any(|(event, payload)| event == dto::EVENT_OPEN_DOCUMENT_ERROR
                && serde_json::from_value::<dto::OpenDocumentError>(payload).is_ok()));
    }

    #[tokio::test]
    async fn open_and_close_updates_focused_document() {
        let app = TigerAppMock::new();

        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        assert_eq!(
            app.client_state().current_document_path,
            Some("test-data/samurai.tiger".into())
        );

        app.open_documents(vec!["test-data/flame.tiger"]).await;
        assert_eq!(
            app.client_state().current_document_path,
            Some("test-data/flame.tiger".into())
        );

        app.close_document("test-data/flame.tiger");
        assert_eq!(
            app.client_state().current_document_path,
            Some("test-data/samurai.tiger".into())
        );
    }

    #[tokio::test]
    async fn can_close_all_documents() {
        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger", "test-data/flame.tiger"])
            .await;
        assert_eq!(app.client_state().documents.len(), 2);
        app.close_all_documents();
        assert_eq!(app.client_state().documents.len(), 0);
    }

    #[tokio::test]
    async fn can_close_current_document() {
        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger", "test-data/flame.tiger"])
            .await;
        assert_eq!(
            app.client_state().current_document_path,
            Some("test-data/flame.tiger".into())
        );
        app.close_current_document();
        assert_eq!(app.client_state().documents.len(), 1);
        assert_eq!(
            app.client_state().current_document_path,
            Some("test-data/samurai.tiger".into())
        );
    }

    #[tokio::test]
    async fn can_request_exit_and_cancel() {
        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        app.delete_frame(&app.document().sheet.frames[0].path);
        app.request_exit();
        assert!(!app.is_closed());
        assert!(app.document().was_close_requested);
        app.cancel_close_document();
        assert!(!app.is_closed());
        assert!(!app.document().was_close_requested);
        assert!(app.document().has_unsaved_changes);
    }

    #[tokio::test]
    async fn can_close_app() {
        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        assert!(!app.is_closed());
        app.request_exit();
        assert!(app.is_closed());
    }

    #[tokio::test]
    async fn can_request_exit_and_discard_changes() {
        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        let deleted_frame = app.document().sheet.frames[0].path.to_owned();
        app.delete_frame(&app.document().sheet.frames[0].path);
        app.request_exit();
        assert!(!app.is_closed());
        assert!(app.document().was_close_requested);
        app.close_without_saving();
        assert!(app.is_closed());

        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        assert!(app
            .document()
            .sheet
            .frames
            .iter()
            .any(|f| f.path == deleted_frame));
    }

    #[tokio::test]
    async fn can_request_exit_and_save_changes() {
        let sheet_file = PathBuf::from("test-output/can_request_exit_and_save_changes.tiger");
        std::fs::copy("test-data/samurai.tiger", &sheet_file).unwrap();

        let app = TigerAppMock::new();
        app.open_documents(vec![&sheet_file]).await;
        let deleted_frame = app.document().sheet.frames[0].path.to_owned();
        app.delete_frame(&deleted_frame);
        app.request_exit();
        assert!(!app.is_closed());
        assert!(app.document().was_close_requested);
        app.save().await;
        assert!(app.is_closed());

        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        assert!(app
            .document()
            .sheet
            .frames
            .iter()
            .all(|f| f.path != deleted_frame));
    }

    #[tokio::test]
    async fn can_resolve_multiple_files_when_exiting() {
        let app = TigerAppMock::new();

        let samurai_file = PathBuf::from("test-data/samurai.tiger").resolve();
        let flame_file = PathBuf::from("test-data/flame.tiger").resolve();
        app.open_documents(vec![&samurai_file, &flame_file]).await;

        app.focus_document(&samurai_file);
        app.delete_frame(&app.document().sheet.frames[0].path);
        app.focus_document(&flame_file);
        app.delete_frame(&app.client_state().documents[1].sheet.frames[0].path);

        app.request_exit();
        assert!(!app.is_closed());
        assert!(app.document().was_close_requested);
        assert!(app.client_state().documents[1].was_close_requested);
        app.close_without_saving();
        assert!(!app.is_closed());
        assert_eq!(app.client_state().documents.len(), 1);
        app.close_without_saving();
        assert!(app.is_closed());
    }

    #[tokio::test]
    async fn can_save_all() {
        let app = TigerAppMock::new();

        let samurai_file = PathBuf::from("test-output/can_save_all_samurai.tiger").resolve();
        let flame_file = PathBuf::from("test-output/can_save_all_flame.tiger").resolve();

        std::fs::copy("test-data/samurai.tiger", &samurai_file).unwrap();
        std::fs::copy("test-data/flame.tiger", &flame_file).unwrap();

        app.open_documents(vec![&samurai_file, &flame_file]).await;

        app.focus_document(&samurai_file);
        app.delete_frame(&app.document().sheet.frames[0].path);
        app.focus_document(&flame_file);
        app.delete_frame(&app.client_state().documents[1].sheet.frames[0].path);

        assert!(app.document().has_unsaved_changes);
        assert!(app.client_state().documents[1].has_unsaved_changes);
        app.save_all().await;
        assert!(!app.document().has_unsaved_changes);
        assert!(!app.client_state().documents[1].has_unsaved_changes);
    }

    #[tokio::test]
    async fn can_save_to_new_location() {
        let original_location = PathBuf::from("test-data/samurai.tiger");
        let new_location = PathBuf::from("test-output/can_save_to_new_location.tiger");

        let app = TigerAppMock::new();
        app.open_documents(vec![&original_location]).await;
        let deleted_frame = app.document().sheet.frames[0].path.to_owned();
        app.delete_frame(&deleted_frame);
        app.save_as(&new_location).await;

        let app = TigerAppMock::new();
        app.open_documents(vec![&original_location]).await;
        assert!(app
            .document()
            .sheet
            .frames
            .iter()
            .any(|f| f.path == deleted_frame));

        let app = TigerAppMock::new();
        app.open_documents(vec![&new_location]).await;
        assert!(app
            .document()
            .sheet
            .frames
            .iter()
            .all(|f| f.path != deleted_frame));
    }

    #[tokio::test]
    async fn can_manually_focus_a_document() {
        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        app.open_documents(vec!["test-data/flame.tiger"]).await;
        app.focus_document("test-data/samurai.tiger");
        assert_eq!(
            app.client_state().current_document_path,
            Some("test-data/samurai.tiger".into())
        );
    }

    #[test]
    fn can_cycle_documents() {
        let app = TigerAppMock::new();
        app.new_document("1");
        app.new_document("2");
        app.new_document("3");
        assert_eq!(app.client_state().current_document_path, Some("3".into()));
        app.focus_next_document();
        assert_eq!(app.client_state().current_document_path, Some("1".into()));
        app.focus_next_document();
        assert_eq!(app.client_state().current_document_path, Some("2".into()));
        app.focus_next_document();
        assert_eq!(app.client_state().current_document_path, Some("3".into()));
        app.focus_previous_document();
        assert_eq!(app.client_state().current_document_path, Some("2".into()));
        app.focus_previous_document();
        assert_eq!(app.client_state().current_document_path, Some("1".into()));
        app.focus_previous_document();
        assert_eq!(app.client_state().current_document_path, Some("3".into()));
    }

    #[test]
    fn keeps_track_of_recently_opened_documents() {
        let mut state = State::default();
        let flame_file = PathBuf::from("test-data/flame.tiger").resolve();
        let samurai_file = PathBuf::from("test-data/samurai.tiger").resolve();
        let relocated_file = PathBuf::from("relocated").resolve();

        state.open_document(Document::open("test-data/samurai.tiger").unwrap());
        assert_eq!(*state.recent_documents, vec![samurai_file.clone()]);

        state.open_document(Document::open("test-data/flame.tiger").unwrap());
        assert_eq!(
            *state.recent_documents,
            vec![flame_file.clone(), samurai_file.clone()]
        );

        state.open_document(Document::open("test-data/samurai.tiger").unwrap());
        assert_eq!(
            *state.recent_documents,
            vec![samurai_file.clone(), flame_file.clone(),]
        );

        state.relocate_document("test-data/samurai.tiger", "relocated");
        assert_eq!(
            *state.recent_documents,
            vec![
                relocated_file.clone(),
                samurai_file.clone(),
                flame_file.clone(),
            ]
        );

        state.new_document("new");
        assert_eq!(
            *state.recent_documents,
            vec![
                PathBuf::from("new").resolve(),
                relocated_file,
                samurai_file,
                flame_file,
            ]
        );
    }

    #[test]
    fn limits_list_of_recent_documents() {
        let mut state = State::default();

        for i in 0..100 {
            state.add_recent_document(PathBuf::from(format!("doc_{i}")));
        }

        assert_eq!(
            *state.recent_documents,
            (90..=99)
                .rev()
                .map(|i| PathBuf::from(format!("doc_{i}")).resolve())
                .collect::<Vec<_>>()
        );
    }

    #[tokio::test]
    async fn can_bring_up_error_message() {
        let app = TigerAppMock::new();
        app.open_documents(vec!["test-data/samurai.tiger"]).await;
        app.show_error_message("title", "summary", "details");
        let error = app.client_state().error.unwrap();
        assert_eq!(error.title, "title");
        assert_eq!(error.summary, "summary");
        assert_eq!(error.details, "details");
    }

    #[test]
    fn can_acknowledge_error() {
        let app = TigerAppMock::new();
        app.show_error_message("title", "summary", "details");
        assert!(app.client_state().error.is_some());
        app.acknowledge_error();
        assert!(app.client_state().error.is_none());
    }

    #[test]
    fn can_open_and_close_about_dialog() {
        let app = TigerAppMock::new();
        assert!(!app.client_state().about_dialog_open);
        app.open_about_dialog();
        assert!(app.client_state().about_dialog_open);
        app.close_about_dialog();
        assert!(!app.client_state().about_dialog_open);
    }
}
