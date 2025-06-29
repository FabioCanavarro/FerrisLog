use std::thread;
use crate::kvstore::error::KvResult;
pub mod naive;

pub trait ThreadPool {
    fn new(n: i32) -> KvResult<Self>
    where
        Self: Sized;
    fn spawn<F: Send + 'static + FnOnce()>(&self, f: F);
}

#[derive(Debug)]
pub struct NaiveThreadPool {}

#[derive(Debug)]
pub struct RayonThreadPool {}

impl ThreadPool for NaiveThreadPool {
    fn new(_: i32) -> KvResult<NaiveThreadPool> {
        Ok(NaiveThreadPool {})
    }

    fn spawn<F: Send + 'static + FnOnce()>(&self, f: F) {
        thread::spawn(f);
    }
}

impl ThreadPool for RayonThreadPool {
    fn new(_: i32) -> KvResult<RayonThreadPool> {
        todo!()
    }

    fn spawn<F: Send + 'static + FnOnce()>(&self, _: F) {
        todo!()
    }
}
