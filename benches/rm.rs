use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ferris::kvstore::KvStore;
use tempfile::TempDir;

fn fake_data() -> (String,String){
    (rand::random::<i32>().to_string(),rand::random::<i32>().to_string())
}

fn multi_fake_data() -> Vec<(String,String)> {
    let mut r = Vec::with_capacity(100);
    for _ in 1..100 {
        r.push(fake_data());
    }
    r
}

pub fn single_remove_benchmark(c: &mut Criterion) {
    let (key, value) = fake_data();
    c.bench_function("Remove random", 
        |b| b.iter_batched(
                || {
                    let temp_dir = TempDir::new().unwrap();
                    let mut store = KvStore::open(temp_dir.path()).unwrap();
                    store.set(key.clone(),value.clone()).unwrap();
                    (store,temp_dir)
                },
                //NOTE: I kept tempdir, because, they keep dropping it after the setup finish which
                //cause it fo fail, cuz tempdir is dropped
                |(mut store, _tempdir)| store.remove(black_box(key.clone())),
                criterion::BatchSize::SmallInput
        )
    );
}

criterion_group!(remove_benches, single_remove_benchmark);
criterion_main!(remove_benches);

