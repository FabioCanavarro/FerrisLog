use crate::kvstore::error::KvResult;
use std::{
    fmt::Debug, panic::{catch_unwind, UnwindSafe}, sync::{
        atomic::AtomicBool, mpsc::{channel, Receiver, Sender}, Arc, Mutex
    }, thread::{self, sleep, JoinHandle}, time::Duration
};

use super::ThreadPool;

#[derive(Debug)]
pub struct SharedQueueThreadPool {
    workers: Arc<Mutex<Vec<Worker>>>,
    /* NOTE:
    *   The rust drop methods drops the fields first before our implementation so, which means that
    *   the receiver will be dropped first (old field is (sender,receiver) ), then the thread, but
    *   the thread.join() means to finish its current task then to terminate the thread, but
    *   the thread is accessing invalidated memory, receiver, which was dropped, so the thread is
    *   stuck just waiting for the receiver
    */
    sx: Sender<Box<dyn FnOnce() + 'static + Send + UnwindSafe>>,
    analyzer_thread: Option<JoinHandle<()>>,
    /* NOTE:
    *   Found the solution, what if we have another thread that checks the field of the thread, if
    *   they died, then we join and spawn a new one, that would require each thread to have be able
    *   to mutate their fields, so we use Arc<>, arc is also concurrency safe
    */
    shutdown: Arc<AtomicBool>
}

#[derive(Debug)]
struct Worker {
    // NOTE: The reason why we use Option, is so that we can take ownership, in the drop method,
    // without it we can't
    thread: Option<JoinHandle<()>>,
    dead: Arc<AtomicBool>
}

impl Worker {
    pub fn spawn<F: FnOnce() + Send + 'static + UnwindSafe>(rx: Arc<Mutex<Receiver<F>>>) -> Worker {
        let dead = Arc::new(AtomicBool::new(false));
        let dead_clone: Arc<AtomicBool> = Arc::clone(&dead);
        let handle = thread::spawn(
            move || loop {
                let msg = rx.lock().unwrap().recv();
                match msg {
                    Ok(f) => {
                        let result = catch_unwind(
                            move|| {
                                f()
                            }
                        );
                        if let Err(_) = result { 
                            dead_clone.store(true, std::sync::atomic::Ordering::SeqCst);
                        }
                    },
                    Err(_) => break,
                }
            }
        );
        Worker {
            thread: Some(handle),
            dead,
        }
    }
}

impl ThreadPool for SharedQueueThreadPool {
    fn new(n: i32) -> KvResult<SharedQueueThreadPool> {
        let workers: Arc<Mutex<Vec<Worker>>> = Arc::new(Mutex::new(Vec::new()));
        let worker_clone = Arc::clone(&workers);
        let (sx, rx) = channel();
        let rx = Arc::new(Mutex::new(rx));
        let shutdown: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let shutdown_clone = Arc::clone(&shutdown);
        for _ in 0..n {
            workers.lock().unwrap().push(Worker::spawn(rx.clone()));
        }
        let thread = thread::spawn(
            move || {
                loop{
                    sleep(Duration::from_millis(100));
                    let mut workers_guard = worker_clone.lock().unwrap();
                    let mut to_add = 0;
                    let mut active_worker: Vec<Worker> = Vec::new();
                    for mut i in worker_clone.lock().unwrap().drain(..) {
                        if i.dead.load(std::sync::atomic::Ordering::SeqCst) {
                            let _ = (&mut i).thread.take().unwrap().join();
                            to_add +=1;
                        }
                        else {
                            active_worker.push(i);
                        }
                    };
                    *workers_guard = active_worker;

                    for _ in 0..to_add {
                       workers_guard.push(Worker::spawn(rx.clone())); 
                    }

                    if shutdown_clone.load(std::sync::atomic::Ordering::SeqCst) {
                        break;
                    }
                }
            }
        );
        Ok(SharedQueueThreadPool {
            workers,
            sx,
            analyzer_thread: Some(thread),
            shutdown
        })
    }

    fn spawn<F: Send + 'static + FnOnce() + UnwindSafe>(&self, f: F) {
        let _ = self.sx.send(Box::new(f));
    }
}

impl Drop for SharedQueueThreadPool {
    fn drop(&mut self) {
        for i in self.workers.lock().unwrap().iter_mut() {
            let thread = i.thread.take().unwrap().join();
            match thread {
                    Ok(_) => (),
                    Err(e) =>  println!("{:?}",e),
            }
        }
        self.shutdown.store(true, std::sync::atomic::Ordering::SeqCst);
        self.analyzer_thread.take().unwrap().join().unwrap();
    }
}

/* WARNING:
*   The reason why the panic test is failing is because, they made all the threads panic and want
*   to see how well can we manage panics, so our job is to find the panic or dead threads and
*   .join() them and replace them with new ones
*
*/
