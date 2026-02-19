use criterion::{criterion_group, criterion_main};

fn book_benchmarks(_c: &mut criterion::Criterion) {}

criterion_group!(benches, book_benchmarks);
criterion_main!(benches);
