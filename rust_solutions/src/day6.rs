pub fn part1(input: &str) -> u64 {
    let mut lines = input.lines().map(|v| v.trim());
    let times = lines.by_ref().find_map(|l| l.strip_prefix("Time:")).unwrap().split_whitespace().map(|v| v.parse::<u64>().unwrap()).collect::<Vec<_>>();
    let distances = lines.by_ref().find_map(|l| l.strip_prefix("Distance:")).unwrap().split_whitespace().map(|v| v.parse::<u64>().unwrap()).collect::<Vec<_>>();

    let mut prod = 1;
    for (&time, &distance) in times.iter().zip(&distances) {
        // Simple brute way
        // let ways = (0..time).map(|input| input * (time - input)).filter(|&d| d > distance).count();
        // prod *= ways as u64;
        prod *= match time.pow(2).checked_sub(4 * distance) {
            Some(v) => {
                let mut s = v.isqrt();
                if s.pow(2) == v {
                    s -= 1;
                }

                let lower = (time.saturating_sub(s) + 1) / 2;
                let upper = (time + s) / 2;

                upper - lower + 1
            }
            None => 0,
        };
    }
    prod
}

pub fn part2(input: &str) -> u64 {
    let mut lines = input.lines().map(|v| v.trim());
    let time = lines.by_ref().find_map(|l| l.strip_prefix("Time:")).unwrap().split_whitespace().collect::<String>().parse::<u64>().unwrap();
    let distance = lines.by_ref().find_map(|l| l.strip_prefix("Distance:")).unwrap().split_whitespace().collect::<String>().parse::<u64>().unwrap();

    match time.pow(2).checked_sub(4 * distance) {
        Some(v) => {
            let mut s = v.isqrt();
            if s.pow(2) == v {
                s -= 1;
            }

            let lower = (time.saturating_sub(s) + 1) / 2;
            let upper = (time + s) / 2;

            upper - lower + 1
        }
        None => 0,
    }
}

#[test]
fn test_answers() {
    let input = std::fs::read_to_string("inputs/day6.txt").unwrap();
    assert_eq!(part1(&input), 4568778);
    assert_eq!(part2(&input), 28973936);
}