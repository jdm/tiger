use parking_lot::Mutex;
use std::{sync::Arc, time::Duration};

use crate::{
    app::{App, AppState},
    features, TigerApp,
};

#[derive(Clone)]
pub struct TigerAppMock {
    app_state: AppState,
}

impl TigerAppMock {
    const PERIOD: Duration = Duration::from_millis(50);

    pub fn new() -> Self {
        let app = Self {
            app_state: AppState(Arc::new(Mutex::new(App::default()))),
        };
        features::missing_textures::init(app.clone(), Self::PERIOD);
        app
    }

    pub fn wait_for_periodic_scans(&self) {
        std::thread::sleep(2 * Self::PERIOD);
    }
}

impl TigerApp for TigerAppMock {
    fn state<T>(&self) -> AppState {
        self.app_state.clone()
    }

    fn patch_state<F: FnOnce(&mut App)>(&self, _app_trim: crate::dto::AppTrim, operation: F) {
        let app_state = self.state::<AppState>();
        let mut app = app_state.0.lock();
        operation(&mut app);
    }

    fn replace_state(&self) {}
}
