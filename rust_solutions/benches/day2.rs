use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn benchmark_day2(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day2.txt").unwrap();
    c.bench_function("day 2 part 1", |b| b.iter(|| rust_solutions::day2::part_1(black_box(&input))));
    c.bench_function("day 2 part 2", |b| b.iter(|| rust_solutions::day2::part_2(black_box(&input))));
}

criterion_group!(day2, benchmark_day2, benchmark_day3);
criterion_main!(day2);

pub fn benchmark_day3(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day3.txt").unwrap();
    c.bench_function("day 3 part 1", |b| b.iter(|| rust_solutions::day3::part1(black_box(&input))));
    c.bench_function("day 3 part 2", |b| b.iter(|| rust_solutions::day3::part2(black_box(&input))));
}
