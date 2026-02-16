use std::sync::{
    Arc,
    atomic::{AtomicU32, Ordering},
};
use tokio::sync::Mutex;

use crate::models::Todo;

#[derive(Clone)]
pub struct AppState {
    pub todos: Arc<Mutex<Vec<Todo>>>,
    pub next_id: Arc<AtomicU32>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            todos: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(AtomicU32::new(1)),
        }
    }

    pub fn allocate_id(&self) -> u32 {
        self.next_id.fetch_add(1, Ordering::Relaxed)
    }
}
