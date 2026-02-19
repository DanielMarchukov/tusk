use criterion::{criterion_group, criterion_main};

fn parser_benchmarks(_c: &mut criterion::Criterion) {}

criterion_group!(benches, parser_benchmarks);
criterion_main!(benches);
