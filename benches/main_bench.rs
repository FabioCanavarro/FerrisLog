use criterion::criterion_main;
mod get;
mod set;
mod rm;

/*criterion_main!(
    get::get_benches,
    set::set_benches,
    rm::remove_benches
); */

criterion_main!(
    set::set_benches
);
