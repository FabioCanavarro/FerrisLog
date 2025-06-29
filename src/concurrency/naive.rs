use std::{clone, env::Args, fmt::Debug, sync::{mpsc::{self, channel, Receiver, Sender}, Arc, Mutex}, thread::{self, JoinHandle}};
use crate::kvstore::error::KvResult;

use super::ThreadPool;

#[derive(Debug)]
struct Worker<T>{
    thread: JoinHandle<T>
    
}

impl<T: Send + 'static> Worker<T> {
    pub fn spawn<F: FnOnce() + Send + 'static>(rx: Arc<Mutex<Receiver<F>>>) -> Worker<T>{
        let handle = thread::spawn(
            move|| {
                loop {
                    let f = rx.lock().unwrap().recv().unwrap();
                    f()
                }
            }
        );
        Worker { thread: handle }
    }
}

#[derive(Debug)]
pub struct SharedQueueThreadPool<T,F: Send + FnOnce() + 'static> {
    workers: Vec<Worker<T>>,
    channel: (Sender<F>, Arc<Mutex<Receiver<F>>>)

}

impl<T: Send + 'static, F: FnOnce() + Send + 'static> ThreadPool<F> for SharedQueueThreadPool<T, F> {
    fn new (n: i32) -> KvResult<SharedQueueThreadPool<T,F>> {
        let mut workers = vec![];
        let (sx, rx) = channel();
        let rx = Arc::new(Mutex::new(rx));
        for _ in 0..n {
            workers.push(
                Worker::spawn(rx.clone())
            );
        }
        Ok(
            SharedQueueThreadPool { 
                workers,
                channel: (sx, rx)
            }
        )

    }

    fn spawn(&self, f: F){
        self.channel.0.send(f);
    }
}
