fn main() {
    const TEST: &str = "\
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";
    let input = std::fs::read_to_string("inputs/day7.txt").unwrap();
    println!("Part 1 TEST: {}", rust_solutions::day7::part1(TEST));
    println!("Part 2 TEST: {}", rust_solutions::day7::part2(TEST));
    println!("Part 1: {}", rust_solutions::day7::part1(&input));
    println!("Part 2: {}", rust_solutions::day7::part2(&input));
}
