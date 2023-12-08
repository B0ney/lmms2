use std::sync::{Arc, atomic::{AtomicU64, Ordering, AtomicU8}};

use portable_atomic::AtomicF32;


pub struct AutomationID(u64);

impl AutomationID {
    pub fn new() -> Self {
        static ID_COUNTER: AtomicU64 = AtomicU64::new(0);
        Self(ID_COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}


/// Automatable parameters
#[derive(Clone)]
pub struct ParamF32(Arc<(AtomicF32, AutomationID)>);


#[derive(Clone)]
pub struct ParamU8(Arc<(AtomicU8, AutomationID)>);