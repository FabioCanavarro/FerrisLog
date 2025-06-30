use crate::kvstore::error::KvResult;
use std::panic::UnwindSafe;

pub mod naive;
pub mod shared;
pub mod rayon;

pub trait ThreadPool {
    fn new(n: i32) -> KvResult<Self>
    where
        Self: Sized;
    fn spawn<F: Send + 'static + FnOnce() + UnwindSafe>(&self, f: F);
}


