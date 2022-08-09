use tokio::sync::RwLock;

use super::core::Inner;

pub static mut INSTANCES: Vec<RwLock<Inner>> = Vec::<RwLock<Inner>>::new();