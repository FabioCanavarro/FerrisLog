use std::thread;

use crate::kvstore::error::KvResult;
pub mod naive;
pub trait ThreadPool<F>{
    fn new(n:i32) -> KvResult<Self>
    where Self : Sized;
    fn spawn(&self, f: F);
} 

#[derive(Debug)]
pub struct NaiveThreadPool{
}



#[derive( Debug)]
pub struct RayonThreadPool{

}

impl<F: Send + 'static + FnOnce()> ThreadPool<F> for NaiveThreadPool {
    fn new (_: i32) -> KvResult<NaiveThreadPool> {
        Ok(NaiveThreadPool {})
    }

    fn spawn(&self, f: F) {
        thread::spawn(f);
    }
}


impl<F: Send + 'static + FnOnce()> ThreadPool<F> for RayonThreadPool {
    fn new (_: i32) -> KvResult<RayonThreadPool> {
        todo!()
    }

    fn spawn(&self, _: F) {
        todo!()
    }
}

