use std::{
    fs::File,
    sync::{Arc, Mutex},
};

use criterion::{black_box, criterion_group, Criterion};
use crossbeam_utils::sync::WaitGroup;
use ferris::{
    concurrency::{rayon::RayonThreadPool, shared::SharedQueueThreadPool, ThreadPool},
    kvstore::KvStore,
};
use tempfile::TempDir;

fn fake_data() -> (String, String) {
    (
        rand::random::<i32>().to_string(),
        rand::random::<i32>().to_string(),
    )
}

fn multi_fake_data() -> Vec<(String, String)> {
    let mut r = Vec::with_capacity(100);
    for _ in 0..100 {
        r.push(fake_data());
    }
    r
}

pub fn single_set_benchmark(c: &mut Criterion) {
    let (key, value) = fake_data();
    c.bench_function("Single Random Set Operation", |b| {
        b.iter_batched(
            || {
                let temp_dir = TempDir::new().unwrap();
                let store = KvStore::open_custom(temp_dir.path()).unwrap();
                (store, temp_dir)
            },
            //NOTE: I kept tempdir, because, they keep dropping it after the setup finish which
            //cause it fo fail, cuz tempdir is dropped
            |(mut store, _tempdir)| store.set(black_box(key.clone()), black_box(value.clone())),
            criterion::BatchSize::SmallInput,
        )
    });
}

pub fn multi_set_benchmark(c: &mut Criterion) {
    let data = multi_fake_data();
    c.bench_function("100 Random Set Operation", |b| {
        b.iter_batched(
            || {
                let temp_dir = TempDir::new().unwrap();
                let store = KvStore::open_custom(temp_dir.path()).unwrap();
                (store, temp_dir)
            },
            |(mut store, _tempdir)| {
                for item in &data {
                    let _ = store.set(black_box(item.0.clone()), black_box(item.1.clone()));
                }
            },
            criterion::BatchSize::LargeInput,
        )
    });
}

pub fn set_benchmark_shared_pool_4_threads(c: &mut Criterion) {
    let data = multi_fake_data();
    let pool = SharedQueueThreadPool::new(4).unwrap();

    c.bench_function("100 Random Set SharedQueueThreadPool 4 threads", |b| {
        b.iter_batched(
            || {
                let temp_dir = TempDir::new().unwrap();
                let store = KvStore::open_custom(temp_dir.path()).unwrap();
                let shared_store = Arc::new(Mutex::new(store));
                // Create a WaitGroup for each batch
                let wg = WaitGroup::new();
                (shared_store, temp_dir, wg)
            },
            |(shared_store, _tempdir, wg)| {
                for item in &data {
                    let store_clone = Arc::clone(&shared_store);
                    let item_clone = item.clone();
                    // Clone the WaitGroup for each task
                    let wg_clone = wg.clone();
                    pool.spawn(move || {
                        if let Ok(mut store) = store_clone.lock() {
                            let _ = store.set_bench_specific(
                                black_box(item_clone.0.clone()),
                                black_box(item_clone.1.clone()),
                            );
                        }
                        // Drop the WaitGroup clone when the task is done
                        drop(wg_clone);
                    });
                }
                // Wait here until all spawned tasks are complete
                wg.wait();
            },
            criterion::BatchSize::LargeInput,
        )
    });
}

pub fn set_benchmark_shared_pool_8_threads(c: &mut Criterion) {
    let data = multi_fake_data();
    let pool = SharedQueueThreadPool::new(8).unwrap();

    c.bench_function("100 Random Set SharedQueueThreadPool 8 threads", |b| {
        b.iter_batched(
            || {
                let temp_dir = TempDir::new().unwrap();
                let store = KvStore::open_custom(temp_dir.path()).unwrap();
                let shared_store = Arc::new(Mutex::new(store));
                // Create a WaitGroup for each batch
                let wg = WaitGroup::new();
                (shared_store, temp_dir, wg)
            },
            |(shared_store, _tempdir, wg)| {
                for item in &data {
                    let store_clone = Arc::clone(&shared_store);
                    let item_clone = item.clone();
                    // Clone the WaitGroup for each task
                    let wg_clone = wg.clone();
                    pool.spawn(move || {
                        if let Ok(mut store) = store_clone.lock() {
                            let _ = store.set_bench_specific(
                                black_box(item_clone.0.clone()),
                                black_box(item_clone.1.clone()),
                            );
                        }
                        // Drop the WaitGroup clone when the task is done
                        drop(wg_clone);
                    });
                }
                // Wait here until all spawned tasks are complete
                wg.wait();
            },
            criterion::BatchSize::LargeInput,
        )
    });
}

pub fn set_benchmark_rayon_pool_4_threads(c: &mut Criterion) {
    let data = multi_fake_data();
    let pool = RayonThreadPool::new(4).unwrap();

    c.bench_function("100 Random Set RayonThreadPool 4 threads", |b| {
        b.iter_batched(
            || {
                let temp_dir = TempDir::new().unwrap();
                let store = KvStore::open_custom(temp_dir.path()).unwrap();
                let shared_store = Arc::new(Mutex::new(store));
                // Create a WaitGroup for each batch
                let wg = WaitGroup::new();
                (shared_store, temp_dir, wg)
            },
            |(shared_store, _tempdir, wg)| {
                for item in &data {
                    let store_clone = Arc::clone(&shared_store);
                    let item_clone = item.clone();
                    // Clone the WaitGroup for each task
                    let wg_clone = wg.clone();
                    pool.spawn(move || {
                        if let Ok(mut store) = store_clone.lock() {
                            let _ = store.set_bench_specific(
                                black_box(item_clone.0.clone()),
                                black_box(item_clone.1.clone()),
                            );
                        }
                        // Drop the WaitGroup clone when the task is done
                        drop(wg_clone);
                    });
                }
                // Wait here until all spawned tasks are complete
                wg.wait();
            },
            criterion::BatchSize::LargeInput,
        )
    });
}

pub fn set_benchmark_rayon_pool_8_threads(c: &mut Criterion) {
    let data = multi_fake_data();
    let pool = RayonThreadPool::new(8).unwrap();

    c.bench_function("100 Random Set RayonThreadPool 8 threads", |b| {
        b.iter_batched(
            || {
                let temp_dir = TempDir::new().unwrap();
                let store = KvStore::open_custom(temp_dir.path()).unwrap();
                let shared_store = Arc::new(Mutex::new(store));
                // Create a WaitGroup for each batch
                let wg = WaitGroup::new();
                (shared_store, temp_dir, wg)
            },
            |(shared_store, _tempdir, wg)| {
                for item in &data {
                    let store_clone = Arc::clone(&shared_store);
                    let item_clone = item.clone();
                    // Clone the WaitGroup for each task
                    let wg_clone = wg.clone();
                    pool.spawn(move || {
                        if let Ok(mut store) = store_clone.lock() {
                            let _ = store.set_bench_specific(
                                black_box(item_clone.0.clone()),
                                black_box(item_clone.1.clone()),
                            );
                        }
                        // Drop the WaitGroup clone when the task is done
                        drop(wg_clone);
                    });
                }
                // Wait here until all spawned tasks are complete
                wg.wait();
            },
            criterion::BatchSize::LargeInput,
        )
    });
}

criterion_group!(
    set_benches,
    single_set_benchmark,
    multi_set_benchmark,
    set_benchmark_shared_pool_4_threads,
    set_benchmark_shared_pool_8_threads,
    set_benchmark_rayon_pool_4_threads,
    set_benchmark_rayon_pool_8_threads
);
