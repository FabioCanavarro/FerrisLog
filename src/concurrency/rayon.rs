use rayon::ThreadPoolBuilder;

use crate::kvstore::error::KvResult;

use super::ThreadPool;

#[derive(Debug)]
pub struct RayonThreadPool {
    pool: rayon::ThreadPool
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
