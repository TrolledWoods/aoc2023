pub fn part1(input: &str) -> u64 {
    let mut lines = input.lines().map(|v| v.trim());
    let times = lines.by_ref().find_map(|l| l.strip_prefix("Time:")).unwrap().split_whitespace().map(|v| v.parse::<u64>().unwrap()).collect::<Vec<_>>();
    let distances = lines.by_ref().find_map(|l| l.strip_prefix("Distance:")).unwrap().split_whitespace().map(|v| v.parse::<u64>().unwrap()).collect::<Vec<_>>();

    let mut prod = 1;
    for (&time, &distance) in times.iter().zip(&distances) {
        // Simple brute way
        // let ways = (0..time).map(|input| input * (time - input)).filter(|&d| d > distance).count();
        // prod *= ways as u64;

        let inner_sqrt = (time as f64).powi(2) - 4.0 * distance as f64;
        if inner_sqrt >= 0.0 {
            // The weird floor + 1 and ceil - 1 is because our answer requires the distance to be > record, not >= record. So if we find an exact solution
            // for zero, we still need to get the number above/below it!
            let lower = (((time as f64 - f64::sqrt(inner_sqrt)) / 2.0).floor() + 1.0).max(0.0) as u64;
            let upper = (((time as f64 + f64::sqrt(inner_sqrt)) / 2.0).ceil() - 1.0).min(time as f64) as u64;
            prod *= upper - lower + 1;
        } else {
            prod *= 0;
        }
    }
    prod
}

pub fn part2(input: &str) -> u64 {
    let mut lines = input.lines().map(|v| v.trim());
    let time = lines.by_ref().find_map(|l| l.strip_prefix("Time:")).unwrap().split_whitespace().collect::<String>().parse::<u64>().unwrap();
    let distance = lines.by_ref().find_map(|l| l.strip_prefix("Distance:")).unwrap().split_whitespace().collect::<String>().parse::<u64>().unwrap();

    let inner_sqrt = (time as f64).powi(2) - 4.0 * distance as f64;
    if inner_sqrt >= 0.0 {
        // The weird floor + 1 and ceil - 1 is because our answer requires the distance to be > record, not >= record. So if we find an exact solution
        // for zero, we still need to get the number above/below it!
        let lower = (((time as f64 - f64::sqrt(inner_sqrt)) / 2.0).floor() + 1.0).max(0.0) as u64;
        let upper = (((time as f64 + f64::sqrt(inner_sqrt)) / 2.0).ceil() - 1.0).min(time as f64) as u64;
        upper - lower + 1
    } else {
        0
    }
}

#[test]
fn test_answers() {
    let input = std::fs::read_to_string("inputs/day6.txt").unwrap();
    assert_eq!(part1(&input), 4568778);
    assert_eq!(part2(&input), 28973936);
}