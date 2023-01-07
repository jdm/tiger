use parking_lot::Mutex;
use std::{sync::Arc, time::Duration};

use crate::{
    app::{App, AppState},
    features::{self, texture_cache::TextureCache},
    TigerApp,
};

#[derive(Clone)]
pub struct TigerAppMock {
    app_state: AppState,
    texture_cache: TextureCache,
}

impl TigerAppMock {
    const PERIOD: Duration = Duration::from_millis(50);

    pub fn new() -> Self {
        let app = Self {
            app_state: AppState(Arc::new(Mutex::new(App::default()))),
            texture_cache: TextureCache::default(),
        };
        app.texture_cache.init(app.clone(), Self::PERIOD);
        features::missing_textures::init(app.clone(), Self::PERIOD);
        app
    }

    pub fn wait_for_periodic_scans(&self) {
        std::thread::sleep(2 * Self::PERIOD);
    }
}

impl TigerApp for TigerAppMock {
    fn app_state(&self) -> AppState {
        self.app_state.clone()
    }

    fn texture_cache(&self) -> TextureCache {
        self.texture_cache.clone()
    }

    fn patch_state<F: FnOnce(&mut App)>(&self, _app_trim: crate::dto::AppTrim, operation: F) {
        let app_state = self.app_state();
        let mut app = app_state.0.lock();
        operation(&mut app);
    }

    fn replace_state(&self) {}
}
