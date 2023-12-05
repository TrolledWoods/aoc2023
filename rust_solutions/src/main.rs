fn main() {
    const TEST: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
    let input = std::fs::read_to_string("inputs/day5.txt").unwrap();
    println!("TEST Part 1: {}", rust_solutions::day5::part1(TEST));
    println!("TEST Part 2: {}", rust_solutions::day5::part2(TEST));
    println!("Part 1: {}", rust_solutions::day5::part1(&input));
    println!("Part 2: {}", rust_solutions::day5::part2(&input));

    let timer = std::time::Instant::now();
    println!("Brute: {}", rust_solutions::day5::part2_brute(&input));
    println!("Brute method took {:.6}s", timer.elapsed().as_secs_f64());
}
