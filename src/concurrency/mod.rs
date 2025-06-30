use rayon::ThreadPoolBuilder;

use crate::kvstore::error::KvResult;
use std::{panic::UnwindSafe, thread};
pub mod naive;

pub trait ThreadPool {
    fn new(n: i32) -> KvResult<Self>
    where
        Self: Sized;
    fn spawn<F: Send + 'static + FnOnce() + UnwindSafe>(&self, f: F);
}

#[derive(Debug)]
pub struct NaiveThreadPool {}

#[derive(Debug)]
pub struct RayonThreadPool {
    pool: rayon::ThreadPool
}

impl ThreadPool for NaiveThreadPool {
    fn new(_: i32) -> KvResult<NaiveThreadPool> {
        Ok(NaiveThreadPool {})
    }

    fn spawn<F: Send + 'static + FnOnce()>(&self, f: F) {
        thread::spawn(f);
    }
}

impl ThreadPool for RayonThreadPool {
    fn new(n: i32) -> KvResult<RayonThreadPool> {
        let pool = ThreadPoolBuilder::new().num_threads(n as usize).build().unwrap();
        Ok(
            RayonThreadPool { pool }
        )
    }

    fn spawn<F: Send + 'static + FnOnce()>(&self, f: F) {
        self.pool.spawn(f);
    }
}
