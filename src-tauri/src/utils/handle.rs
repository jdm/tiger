use std::sync::Arc;

use parking_lot::{Mutex, MutexGuard};

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

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<T: Default> Default for Handle<T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}
