use std::sync::{Arc, atomic::AtomicUsize};
use crate::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub active_connections: Arc<AtomicUsize>,
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config: Arc::new(config),
            active_connections: Arc::new(AtomicUsize::new(0)),
        }
    }
}
