use crate::kvstore::error::KvResult;
use std::{
    fmt::Debug,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

use super::ThreadPool;

#[derive(Debug)]
struct Worker {
    // NOTE: The reason why we use Option, is so that we can take ownership, in the drop method,
    // without it we can't
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn spawn<F: FnOnce() + Send + 'static>(rx: Arc<Mutex<Receiver<F>>>) -> Worker {
        let handle = thread::spawn(move || loop {
            let msg = rx.lock().unwrap().recv();
            match msg {
                Ok(f) => f(),
                Err(_) => break,
            }
        });
        Worker {
            thread: Some(handle),
        }
    }
}

#[derive(Debug)]
pub struct SharedQueueThreadPool {
    workers: Vec<Worker>,
    sx: Sender<Box<dyn FnOnce() + 'static + Send>>,
}

impl ThreadPool for SharedQueueThreadPool {
    fn new(n: i32) -> KvResult<SharedQueueThreadPool> {
        let mut workers = vec![];
        let (sx, rx) = channel();
        let rx = Arc::new(Mutex::new(rx));
        for _ in 0..n {
            workers.push(Worker::spawn(rx.clone()));
        }
        Ok(SharedQueueThreadPool {
            workers,
            sx
        })
    }

    fn spawn<F: Send + 'static + FnOnce()>(&self, f: F) {
        let _ = self.sx.send(Box::new(f));
    }
}

impl Drop for SharedQueueThreadPool {
    fn drop(&mut self) {
        for i in &mut self.workers {
            let _ = i.thread.take().unwrap().join();
        }
    }
}
