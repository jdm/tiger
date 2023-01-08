use json_patch::Patch;
use parking_lot::Mutex;
use std::{ops::Deref, path::PathBuf, sync::Arc, time::Duration};

use crate::{
    api::Api,
    app::{App, AppState},
    dto,
    features::{self, texture_cache::TextureCache},
    TigerApp,
};

#[derive(Clone)]
pub struct TigerAppMock {
    app_state: AppState,
    texture_cache: TextureCache,
    client_state: Arc<Mutex<dto::App>>,
}

impl TigerAppMock {
    const PERIOD: Duration = Duration::from_millis(50);

    pub fn new() -> Self {
        let app = Self {
            app_state: AppState(Arc::new(Mutex::new(App::default()))),
            texture_cache: TextureCache::default(),
            client_state: Arc::new(Mutex::new(App::default().to_dto(dto::AppTrim::Full))),
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
    fn app_state(&self) -> AppState {
        self.app_state.clone()
    }

    fn texture_cache(&self) -> TextureCache {
        self.texture_cache.clone()
    }

    fn patch_state<F: FnOnce(&mut App)>(&self, app_trim: dto::AppTrim, operation: F) {
        let app_state = self.app_state();
        let patch = app_state.mutate(app_trim, operation);
        self.apply_patch(patch);
    }

    fn replace_state(&self) {
        let app_state = self.app_state();
        let app = app_state.0.lock();
        *self.client_state.lock() = app.to_dto(dto::AppTrim::Full);
    }
}
