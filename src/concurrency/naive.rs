use std::{fmt::Debug, sync::{mpsc::{channel, Receiver, Sender}, Arc, Mutex}, thread::{self, JoinHandle}};
use crate::kvstore::error::KvResult;

use super::ThreadPool;

#[derive(Debug)]
struct Worker{
    thread: JoinHandle<()>
    
}

impl Worker {
    pub fn spawn<F: FnOnce() + Send + 'static>(rx: Arc<Mutex<Receiver<F>>>) -> Worker{
        let handle = thread::spawn(
            move|| {
                loop {
                    let msg = rx.lock().unwrap().recv();
                    match msg {
                        Ok(f) => f(),
                        Err(_) => break
                    }
                }
            }
        );
        Worker { thread: handle }
    }
}

#[derive(Debug)]
pub struct SharedQueueThreadPool<F: Send + FnOnce() + 'static> {
    workers: Vec<Worker>,
    channel: (Sender<F>, Arc<Mutex<Receiver<F>>>)

}

impl<F: FnOnce() + Send + 'static> ThreadPool<F> for SharedQueueThreadPool<F> {
    fn new (n: i32) -> KvResult<SharedQueueThreadPool<F>> {
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

impl<F: 'static + Send + FnOnce()> Drop for SharedQueueThreadPool<F> {
    fn drop(&mut self) {
        for i in &self.workers {
            i.thread.join();
        }
    }
}






















