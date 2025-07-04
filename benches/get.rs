use std::sync::{Arc, Mutex};

use criterion::{black_box, criterion_group, Criterion};
use crossbeam_utils::sync::WaitGroup;
use ferris::{concurrency::{rayon::RayonThreadPool, shared::SharedQueueThreadPool, ThreadPool}, kvstore::KvStore};
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

pub fn single_get_benchmark(c: &mut Criterion) {
    let (key, value) = fake_data();
    c.bench_function("Get random", |b| {
        b.iter_batched(
            || {
                let temp_dir = TempDir::new().unwrap();
                let mut store = KvStore::open_custom(temp_dir.path()).unwrap();
                store.set(key.clone(), value.clone()).unwrap();
                (store, temp_dir)
            },
            //NOTE: I kept tempdir, because, they keep dropping it after the setup finish which
            //cause it fo fail, cuz tempdir is dropped
            |(store, _tempdir)| store.get(black_box(key.clone())),
            criterion::BatchSize::SmallInput,
        )
    });
}

pub fn multi_get_benchmark(c: &mut Criterion) {
    let data = multi_fake_data();
    c.bench_function("100 Random Get Operation", |b| {
        b.iter_batched(
            || {
                let temp_dir = TempDir::new().unwrap();
                let mut store = KvStore::open_custom(temp_dir.path()).unwrap();
                for item in &data {
                    store
                        .set(black_box(item.0.clone()), black_box(item.1.clone()))
                        .unwrap();
                }
                (store, temp_dir)
            },
            |(store, _tempdir)| {
                for item in &data {
                    let _ = store.get(black_box(item.0.clone()));
                }
            },
            criterion::BatchSize::LargeInput,
        )
    });
}

pub fn get_benchmark_shared_pool_4_threads(c: &mut Criterion) {
    let data = multi_fake_data();
    let pool = SharedQueueThreadPool::new(4).unwrap();

    c.bench_function("100 Random Get SharedQueueThreadPool 4 threads", |b| {
        b.iter_batched(
            || {
                let temp_dir = TempDir::new().unwrap();
                let mut store = KvStore::open_custom(temp_dir.path()).unwrap();
                for item in &data {
                    store
                        .set(black_box(item.0.clone()), black_box(item.1.clone()))
                        .unwrap();
                }                let shared_store = Arc::new(Mutex::new(store));
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
                    pool.spawn(
                        move || {
                            if let Ok(mut store) = store_clone.lock() {
                                let _ = store.get(black_box(item.0.clone()));
                            }
                            // Drop the WaitGroup clone when the task is done
                            drop(wg_clone);
                        }
                    );
                }
                // Wait here until all spawned tasks are complete
                wg.wait();
            },
            criterion::BatchSize::LargeInput,
        )
    });
}

pub fn get_benchmark_shared_pool_8_threads(c: &mut Criterion) {
    let data = multi_fake_data();
    let pool = SharedQueueThreadPool::new(8).unwrap();

    c.bench_function("100 Random Get SharedQueueThreadPool 8 threads", |b| {
        b.iter_batched(
            || {
                let temp_dir = TempDir::new().unwrap();
                let mut store = KvStore::open_custom(temp_dir.path()).unwrap();
                for item in &data {
                    store
                        .set(black_box(item.0.clone()), black_box(item.1.clone()))
                        .unwrap();
                }                let shared_store = Arc::new(Mutex::new(store));
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
                    pool.spawn(
                        move || {
                            if let Ok(mut store) = store_clone.lock() {
                                let _ = store.get(black_box(item.0.clone()));
                            }
                            // Drop the WaitGroup clone when the task is done
                            drop(wg_clone);
                        }
                    );
                }
                // Wait here until all spawned tasks are complete
                wg.wait();
            },
            criterion::BatchSize::LargeInput,
        )
    });
}

pub fn get_benchmark_rayon_pool_4_threads(c: &mut Criterion) {
    let data = multi_fake_data();
    let pool = RayonThreadPool::new(4).unwrap();

    c.bench_function("100 Random Get RayonThreadPool 4 threads", |b| {
        b.iter_batched(
            || {
                let temp_dir = TempDir::new().unwrap();
                let mut store = KvStore::open_custom(temp_dir.path()).unwrap();
                for item in &data {
                    store
                        .set(black_box(item.0.clone()), black_box(item.1.clone()))
                        .unwrap();
                }                let shared_store = Arc::new(Mutex::new(store));
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
                    pool.spawn(
                        move || {
                            if let Ok(mut store) = store_clone.lock() {
                                let _ = store.get(black_box(item.0.clone()));
                            }
                            // Drop the WaitGroup clone when the task is done
                            drop(wg_clone);
                        }
                    );
                }
                // Wait here until all spawned tasks are complete
                wg.wait();
            },
            criterion::BatchSize::LargeInput,
        )
    });
}

pub fn get_benchmark_rayon_pool_8_threads(c: &mut Criterion) {
    let data = multi_fake_data();
    let pool = RayonThreadPool::new(8).unwrap();

    c.bench_function("100 Random Get RayonThreadPool 8 threads", |b| {
        b.iter_batched(
            || {
                let temp_dir = TempDir::new().unwrap();
                let mut store = KvStore::open_custom(temp_dir.path()).unwrap();
                for item in &data {
                    store
                        .set(black_box(item.0.clone()), black_box(item.1.clone()))
                        .unwrap();
                }                let shared_store = Arc::new(Mutex::new(store));
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
                    pool.spawn(
                        move || {
                            if let Ok(mut store) = store_clone.lock() {
                                let _ = store.get(black_box(item.0.clone()));
                            }
                            // Drop the WaitGroup clone when the task is done
                            drop(wg_clone);
                        }
                    );
                }
                // Wait here until all spawned tasks are complete
                wg.wait();
            },
            criterion::BatchSize::LargeInput,
        )
    });
}

criterion_group!(
    get_benches,
    single_get_benchmark,
    multi_get_benchmark
);
