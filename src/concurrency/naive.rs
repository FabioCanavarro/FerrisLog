use super::ThreadPool;
use crate::kvstore::error::KvResult;
use std::thread;

#[derive(Debug)]
pub struct NaiveThreadPool {}

impl ThreadPool for NaiveThreadPool {
    fn new(_: i32) -> KvResult<NaiveThreadPool> {
        Ok(NaiveThreadPool {})
    }

    fn spawn<F: Send + 'static + FnOnce()>(&self, f: F) {
        thread::spawn(f);
    }
}
