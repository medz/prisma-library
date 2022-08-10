use std::sync::{Arc, RwLock};

use intmap::IntMap;
use once_cell::sync::Lazy;

use super::core::Inner;

// pub static mut INSTANCES: Vec<RwLock<Inner>> = Vec::<RwLock<Inner>>::new();

pub static mut INSTANCES: Lazy<RwLock<IntMap<Arc<Inner>>>> =
    Lazy::new(|| RwLock::new(IntMap::new()));

static mut NEXT_ID: i64 = 0;

/// Insert an engine into the global list of engines.
pub fn insert(engine: Arc<Inner>) -> i64 {
    let id = unsafe {
        let id = NEXT_ID;
        NEXT_ID += 1;

        id
    };

    let mut lock = unsafe { INSTANCES.write().unwrap() };

    lock.insert(id.unsigned_abs(), engine);

    id
}
