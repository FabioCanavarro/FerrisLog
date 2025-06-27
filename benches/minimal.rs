use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn minimal_benchmark(c: &mut Criterion) {
    c.bench_function("minimal_test", |b| b.iter(|| black_box(1 + 1)));
}

criterion_group!(benches, minimal_benchmark);
criterion_main!(benches);
