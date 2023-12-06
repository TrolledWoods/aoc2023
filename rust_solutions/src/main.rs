fn main() {
    const TEST: &str = "\
Time:      7  15   30
Distance:  9  40  200
";
    let input = std::fs::read_to_string("inputs/day6.txt").unwrap();
    println!("TEST Part 1: {}", rust_solutions::day6::part1(TEST));
    println!("TEST Part 2: {}", rust_solutions::day6::part2(TEST));
    println!("Part 1: {}", rust_solutions::day6::part1(&input));
    println!("Part 2: {}", rust_solutions::day6::part2(&input));
}
