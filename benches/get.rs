use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ferris::kvstore::KvStore;
use tempfile::TempDir;

fn fake_data() -> (String,String){
    (rand::random::<i32>().to_string(),rand::random::<i32>().to_string())
}


pub fn single_get_benchmark(c: &mut Criterion) {
    let (key, value) = fake_data();
    c.bench_function("Get random", 
        |b| b.iter_batched(
                || {
                    let temp_dir = TempDir::new().unwrap();
                    let mut store = KvStore::open(temp_dir.path()).unwrap();
                    store.set(key.clone(),value.clone()).unwrap();
                    (store,temp_dir)
                },
                //NOTE: I kept tempdir, because, they keep dropping it after the setup finish which
                //cause it fo fail, cuz tempdir is dropped
                |(store, _tempdir)| store.get(black_box(key.clone())),
                criterion::BatchSize::SmallInput
        )
    );
}

criterion_group!(get_benches, single_get_benchmark);
criterion_main!(get_benches);

