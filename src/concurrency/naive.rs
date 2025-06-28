use std::{clone, sync::{mpsc::Receiver, Arc, Mutex}, thread::{self, JoinHandle}};

use crossbeam_utils::thread;

use crate::kvstore::error::KvResult;

use super::ThreadPool;

struct Worker<T>{
    thread: Option<JoinHandle<T>>
    
}

impl Worker<T> {
    fn spawn(&self, rx: Arc<Mutex<Receiver<()>>>) {
        let handle = thread::spawn(
            || {
                loop {
                    let f = rx.clone().lock();
                }
            }
        );
        &self.thread = Some(handle);
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
