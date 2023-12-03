fn main() {
    const TEST: &str = r"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
    let input = std::fs::read_to_string("inputs/day3.txt").unwrap();
    println!("Part 1: {}", rust_solutions::day3::part1(&input));
    println!("Part 2: {}", rust_solutions::day3::part2(&input));
}
