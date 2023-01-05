use parking_lot::Mutex;
use std::sync::Arc;

use crate::{
    app::{App, AppState},
    TigerApp,
};

#[derive(Clone)]
pub struct TigerAppMock {
    app_state: AppState,
}

impl TigerAppMock {
    pub fn new() -> Self {
        Self {
            app_state: AppState(Arc::new(Mutex::new(App::default()))),
        }
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
