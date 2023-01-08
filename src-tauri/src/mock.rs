use json_patch::Patch;
use parking_lot::Mutex;
use std::{ops::Deref, path::PathBuf, sync::Arc, time::Duration};

use crate::{
    api::Api,
    dto,
    features::{self, texture_cache},
    state::{self, State},
    TigerApp,
};

#[derive(Clone)]
pub struct TigerAppMock {
    state: state::Handle,
    texture_cache: texture_cache::Handle,
    client_state: Arc<Mutex<dto::App>>,
}

impl TigerAppMock {
    const PERIOD: Duration = Duration::from_millis(50);

    pub fn new() -> Self {
        let app = Self {
            state: state::Handle::default(),
            texture_cache: texture_cache::Handle::default(),
            client_state: Arc::new(Mutex::new(State::default().to_dto(dto::AppTrim::Full))),
        };
        app.texture_cache.init(app.clone(), Self::PERIOD);
        features::missing_textures::init(app.clone(), Self::PERIOD);
        app
    }

    pub fn wait_for_periodic_scans(&self) {
        std::thread::sleep(2 * Self::PERIOD);
    }

    pub fn client_state(&self) -> dto::App {
        self.client_state.lock().clone()
    }

    fn apply_patch(&self, patch: Patch) {
        let mut client_state = serde_json::to_value(self.client_state.lock().deref()).unwrap();
        json_patch::patch(&mut client_state, &patch).unwrap();
        *self.client_state.lock() = serde_json::from_value(client_state).unwrap();
    }
}

impl TigerAppMock {
    pub fn delete_frame(&self, path: PathBuf) {
        self.apply_patch(Api::delete_frame(self, path).unwrap());
    }

    pub async fn export(&self) {
        self.apply_patch(Api::export(self).await.unwrap());
    }

    pub fn import_frames(&self, paths: Vec<PathBuf>) {
        self.apply_patch(Api::import_frames(self, paths).unwrap());
    }

    pub fn new_document(&self, path: PathBuf) {
        self.apply_patch(Api::new_document(self, path).unwrap());
    }

    pub async fn open_documents(&self, paths: Vec<PathBuf>) {
        self.apply_patch(Api::open_documents(self, paths).await.unwrap());
    }
}

impl TigerApp for TigerAppMock {
    fn state(&self) -> state::Handle {
        self.state.clone()
    }

    fn texture_cache(&self) -> texture_cache::Handle {
        self.texture_cache.clone()
    }

    fn patch_state<F: FnOnce(&mut State)>(&self, app_trim: dto::AppTrim, operation: F) {
        let state_handle = self.state();
        let patch = state_handle.mutate(app_trim, operation);
        self.apply_patch(patch);
    }

    fn replace_state(&self) {
        let state_handle = self.state();
        let state = state_handle.0.lock();
        *self.client_state.lock() = state.to_dto(dto::AppTrim::Full);
    }
}
