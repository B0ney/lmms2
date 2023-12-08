use std::sync::Arc;

use portable_atomic::AtomicF32;



#[derive(Clone, Default)]
pub struct Param(Arc<AtomicF32>);