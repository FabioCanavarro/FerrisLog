use std::{clone, env::Args, fmt::Debug, sync::{mpsc::{Receiver, Sender}, Arc, Mutex}, thread::{self, JoinHandle}};
use crate::kvstore::error::KvResult;

use super::ThreadPool;

#[derive(Debug)]
struct Worker<T>{
    thread: Option<JoinHandle<T>>
    
}

impl<T: Send + 'static> Worker<T> {
    fn spawn<F: FnOnce() + Send + 'static>(&mut self, rx: Arc<Mutex<Receiver<F>>>) {
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
pub struct SharedQueueThreadPool<T,F: Send + FnOnce() + 'static> {
    workers: Vec<Worker<T>>,
    jobs: Vec<F>,
    channel: (Arc<Mutex<Receiver<F>>>, Sender<F>)

}

impl<T,F: FnOnce() + Send + 'static> ThreadPool for SharedQueueThreadPool<T, F> {
    fn new (n: i32) -> KvResult<SharedQueueThreadPool<T,F>> {
        todo!()
    }

    fn spawn<Y> (&self, f: Y) {
        todo!()
    }
}
