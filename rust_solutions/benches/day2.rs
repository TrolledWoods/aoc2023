use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn benchmark_day2(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day2.txt").unwrap();
    c.bench_function("day 2 part 1", |b| b.iter(|| rust_solutions::day2::part_1(black_box(&input))));
    c.bench_function("day 2 part 2", |b| b.iter(|| rust_solutions::day2::part_2(black_box(&input))));
}

criterion_group!(day2, benchmark_day2, benchmark_day3, benchmark_day4, benchmark_day5, benchmark_day6);
criterion_main!(day2);

pub fn benchmark_day3(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day3.txt").unwrap();
    c.bench_function("day 3 part 1", |b| b.iter(|| rust_solutions::day3::part1(black_box(&input))));
    c.bench_function("day 3 part 2", |b| b.iter(|| rust_solutions::day3::part2(black_box(&input))));
}

pub fn benchmark_day4(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day4.txt").unwrap();
    c.bench_function("day 4 part 1", |b| b.iter(|| rust_solutions::day4::part1(black_box(&input))));
    c.bench_function("day 4 part 2", |b| b.iter(|| rust_solutions::day4::part2(black_box(&input))));
}

pub fn benchmark_day5(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day5.txt").unwrap();
    c.bench_function("day 5 part 1", |b| b.iter(|| rust_solutions::day5::part1(black_box(&input))));
    c.bench_function("day 5 part 2", |b| b.iter(|| rust_solutions::day5::part2(black_box(&input))));
}

pub fn benchmark_day6(c: &mut Criterion) {
    let input = std::fs::read_to_string("inputs/day6.txt").unwrap();
    c.bench_function("day 6 part 1", |b| b.iter(|| rust_solutions::day6::part1(black_box(&input))));
    c.bench_function("day 6 part 2", |b| b.iter(|| rust_solutions::day6::part2(black_box(&input))));
}