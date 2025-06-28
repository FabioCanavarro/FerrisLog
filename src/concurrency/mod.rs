use std::{env::Args, thread};

use crate::kvstore::error::KvResult;
pub mod naive;
pub trait ThreadPool{
    fn new(n:i32) -> KvResult<Self>
    where Self : Sized;
    fn spawn<T: FnOnce() + Send + 'static> (&self, f: T);
} 

#[derive(Debug)]
pub struct NaiveThreadPool{
}



#[derive( Debug)]
pub struct RayonThreadPool{

}

impl ThreadPool for NaiveThreadPool {
    fn new (_: i32) -> KvResult<NaiveThreadPool> {
        Ok(NaiveThreadPool {})
    }

    fn spawn<T: FnOnce() + Send + 'static> (&self, f: T) {
        thread::spawn(f);
    }
}


impl ThreadPool for RayonThreadPool {
    fn new (n: i32) -> KvResult<RayonThreadPool> {
        todo!()
    }

    fn spawn<T> (&self, f: T) {
        todo!()
    }
}

