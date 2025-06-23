use std::{env::Args, process::Output};

pub trait ThreadPool{
    fn new<T> (n: i32) -> T;
    fn spawn<T> (&self, f: T);
} 

pub struct NaiveThreadPool{

}

pub struct SharedQueueThreadPool{

}

pub struct RayonThreadPool{

}
