use crate::kvstore::error::KvResult;

pub trait ThreadPool{
    fn new(n:i32) -> KvResult<Self>
    where Self : Sized;
    fn spawn<T> (&self, f: T);
} 

#[derive(Debug)]
pub struct NaiveThreadPool{

}

#[derive(Debug)]
pub struct SharedQueueThreadPool{

}

#[derive( Debug)]
pub struct RayonThreadPool{

}

impl ThreadPool for NaiveThreadPool {
    fn new (n: i32) -> KvResult<NaiveThreadPool> {
        todo!()
    }

    fn spawn<T> (&self, f: T) {
        todo!()
    }
}

impl ThreadPool for SharedQueueThreadPool {
    fn new (n: i32) -> KvResult<SharedQueueThreadPool> {
        todo!()
    }

    fn spawn<T> (&self, f: T) {
        todo!()
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
