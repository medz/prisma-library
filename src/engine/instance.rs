use tokio::sync::RwLock;

use super::core::Inner;

pub static INSTANCES: Vec<RwLock<Inner>> = Vec::<RwLock<Inner>>::new();