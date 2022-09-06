use criterion::{criterion_group, criterion_main, Criterion};
use day6::bench_function;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fishtest", |b| b.iter(|| bench_function()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
