use core::f32;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ferris::kvstore::KvStore;
use tempfile::TempDir;

pub fn criterion_benchmark(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let mut store = KvStore::open(&temp_dir.path()).unwrap();
    c.bench_function("fib 20", |b| b.iter(|| 
        store.set(black_box(String::from(rand::random::<i32>())),black_box(String::from(rand::random::<i32>())))
        )
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
