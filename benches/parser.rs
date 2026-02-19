use criterion::{criterion_group, criterion_main};

#[allow(clippy::missing_const_for_fn)]
fn parser_benchmarks(_c: &mut criterion::Criterion) {}

criterion_group!(benches, parser_benchmarks);
criterion_main!(benches);
