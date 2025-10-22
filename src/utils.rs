use std::sync::OnceLock;
use parking_lot::{Mutex, MutexGuard};


/// Global Data
pub struct GData<T> {
    data: OnceLock<Mutex<T>>,
    buffer: T,
}
impl<T: Clone> GData<T> {
    pub const fn new(data: T) -> Self {
        Self {
            data: OnceLock::new(),
            buffer: data,
        }
    }
    pub fn lock(&self) -> MutexGuard<'_, T> {
        self.data.get_or_init(|| {
            Mutex::new(self.buffer.clone())
        }).lock()
    }
}