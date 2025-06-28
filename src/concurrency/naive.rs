use std::{clone, env::Args, sync::{mpsc::Receiver, Arc, Mutex}, thread::{self, JoinHandle}};
use crate::kvstore::error::KvResult;

use super::ThreadPool;

struct Worker<T>{
    thread: Option<JoinHandle<T>>
    
}

impl<T: Send + 'static> Worker<T> {
    fn spawn<f: FnOnce() + Send + 'static>(&mut self, rx: Arc<Mutex<Receiver<f>>>) {
        let handle = thread::spawn(
            move|| {
                loop {
                    let f = rx.lock().unwrap().recv().unwrap();
                    f()
                }
            }
        );
        self.thread = Some(handle);
    }
}

#[derive(Debug)]
pub struct SharedQueueThreadPool{

}

impl ThreadPool for SharedQueueThreadPool {
    fn new (n: i32) -> KvResult<SharedQueueThreadPool> {
        todo!()
    }

    fn spawn<T> (&self, f: T) {
        todo!()
    }
}
