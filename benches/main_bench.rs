use criterion::criterion_main;
mod get;
mod rm;
mod set;

criterion_main!(get::get_benches, set::set_benches, rm::remove_benches);
