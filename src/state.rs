use std::sync::{Arc, atomic::AtomicUsize};
use tokio::sync::RwLock;
use crate::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<RwLock<AppConfig>>,
    pub active_connections: Arc<AtomicUsize>,
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config: Arc::new(RwLock::new(config)),
            active_connections: Arc::new(AtomicUsize::new(0)),
        }
    }
}
