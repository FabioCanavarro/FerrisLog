use criterion::{black_box, criterion_group, Criterion};
use ferris_log::kvstore::KvStore;
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

pub fn single_remove_benchmark(c: &mut Criterion) {
    let (key, value) = fake_data();
    c.bench_function("Remove random", |b| {
        b.iter_batched(
            || {
                let temp_dir = TempDir::new().unwrap();
                let mut store = KvStore::open_custom(temp_dir.path()).unwrap();
                store.set(key.clone(), value.clone()).unwrap();
                (store, temp_dir)
            },
            //NOTE: I kept tempdir, because, they keep dropping it after the setup finish which
            //cause it fo fail, cuz tempdir is dropped
            |(mut store, _tempdir)| store.remove(black_box(key.clone())),
            criterion::BatchSize::SmallInput,
        )
    });
}

pub fn multi_remove_benchmark(c: &mut Criterion) {
    let data = multi_fake_data();
    c.bench_function("100 Random Remove Operation", |b| {
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
            |(mut store, _tempdir)| {
                for item in &data {
                    let _ = store.remove(black_box(item.0.clone()));
                }
            },
            criterion::BatchSize::LargeInput,
        )
    });
}

criterion_group!(
    remove_benches,
    single_remove_benchmark,
    multi_remove_benchmark
);
