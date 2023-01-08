use std::sync::Arc;

use parking_lot::{Mutex, MutexGuard};

#[derive(Clone)]
pub struct Handle<T>(pub Arc<Mutex<T>>);

impl<T> Handle<T> {
    pub fn new(inner: T) -> Self {
        Self(Arc::new(Mutex::new(inner)))
    }
}

impl<T> Handle<T> {
    pub fn lock(&self) -> MutexGuard<T> {
        self.0.lock()
    }
}

impl<T: Default> Default for Handle<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}
