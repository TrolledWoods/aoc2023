use day2_rust::{part_1, part_2};
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let input = std::fs::read_to_string("input.txt").unwrap();
    c.bench_function("part 1", |b| b.iter(|| part_1(&input)));
    c.bench_function("part 2", |b| b.iter(|| part_2(&input)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);