use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ferris::kvstore::KvStore;
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

criterion_group!(set_benches, single_set_benchmark, multi_set_benchmark);
criterion_main!(set_benches);
