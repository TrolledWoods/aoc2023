pub fn part1(input: &str) -> u64 {
    let mut lines = input.lines().map(|v| v.trim());
    let times = lines.by_ref().find_map(|l| l.strip_prefix("Time:")).unwrap().split_whitespace().map(|v| v.parse::<u64>().unwrap()).collect::<Vec<_>>();
    let distances = lines.by_ref().find_map(|l| l.strip_prefix("Distance:")).unwrap().split_whitespace().map(|v| v.parse::<u64>().unwrap()).collect::<Vec<_>>();

    let mut prod = 1;
    for (&time, &distance) in times.iter().zip(&distances) {
        let ways = (0..time).map(|input| input * (time - input)).filter(|&d| d > distance).count();
        prod *= ways as u64;
    }
    prod
}

pub fn part2(input: &str) -> u64 {
    let mut lines = input.lines().map(|v| v.trim());
    let time = lines.by_ref().find_map(|l| l.strip_prefix("Time:")).unwrap().split_whitespace().collect::<String>().parse::<u64>().unwrap();
    let distance = lines.by_ref().find_map(|l| l.strip_prefix("Distance:")).unwrap().split_whitespace().collect::<String>().parse::<u64>().unwrap();

    (0..time).map(|input| input * (time - input)).filter(|&d| d > distance).count() as u64
}