use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn bench_day1(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day1.txt").unwrap();
    c.bench_function("day 1 part 1", |b| b.iter(|| rust_solutions::day1::part_1(black_box(&input))));
    c.bench_function("day 1 part 1 dandi", |b| b.iter(|| rust_solutions::day1::part_1_dandi(black_box(&input))));
    c.bench_function("day 1 part 1 ascii dandi", |b| b.iter(|| rust_solutions::day1::part_1_dandi_ascii(black_box(&input))));
    c.bench_function("day 1 part 2", |b| b.iter(|| rust_solutions::day1::part_2(black_box(&input))));
    c.bench_function("day 1 part 2 naiive", |b| b.iter(|| rust_solutions::day1::part_2_naiive(black_box(&input))));
    c.bench_function("day 1 part 2 less naiive", |b| b.iter(|| rust_solutions::day1::part_2_less_naiive(black_box(&input))));
    c.bench_function("day 1 part 2 dandi", |b| b.iter(|| rust_solutions::day1::part_2_dandi(black_box(&input))));
    c.bench_function("day 1 part 2 dandi naiive", |b| b.iter(|| rust_solutions::day1::part_2_dandi_naiive(black_box(&input))));
}

criterion_group!(day1, bench_day1);
criterion_main!(day1);
