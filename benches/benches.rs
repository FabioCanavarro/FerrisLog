use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ferris::kvstore::KvStore;
use tempfile::TempDir;

fn fake_data() -> (String,String){
    (rand::random::<i32>().to_string(),rand::random::<i32>().to_string())
}


pub fn criterion_benchmark(c: &mut Criterion) {
    let (key, value) = fake_data();
    c.bench_function("Set random", 
        |b| b.iter_batched(
                || {
                    let temp_dir = TempDir::new().unwrap();
                    let store = KvStore::open(&temp_dir.path()).unwrap();
                    store
                },
                |mut store| store.set(key.clone(), value.clone()).unwrap(),
                criterion::BatchSize::NumIterations(10)
        )
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);




























































