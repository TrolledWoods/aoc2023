pub fn part1(input: &str) -> i64 {
    input.lines()
        .map(|v| {
            let (a, b) = v.split_once('|').unwrap();
            let winnings = a.split_once(':').unwrap().1;
            let winnings = winnings.split_whitespace().map(|v| v.trim()).collect::<Vec<_>>();
            let count = b.split_whitespace().map(|v| v.trim()).filter(|v| winnings.contains(v)).count();
            if count == 0 {
                0
            } else {
                2_i64.pow(count as u32 - 1)
            }
        })
        .sum::<i64>()
}

pub fn part2(input: &str) -> usize {
    let mut copies = vec![];
    input.lines()
        .enumerate()
        .map(|(i, v)| {
            let (a, b) = v.split_once('|').unwrap();
            let winnings = a.split_once(':').unwrap().1;
            let winnings = winnings.split_whitespace().map(|v| v.trim()).collect::<Vec<_>>();
            let count = b.split_whitespace().map(|v| v.trim()).filter(|v| winnings.contains(v)).count();
            copies.resize_with(copies.len().max(i + count + 1), || 0);
            let clones = copies[i] + 1;
            copies[i+1..i+count+1].iter_mut().for_each(|v| *v += clones);
            clones
        })
        .sum::<usize>()
}

#[test]
fn test_answers() {
    let input = std::fs::read_to_string("inputs/day4.txt").unwrap();
    assert_eq!(part1(&input), 21158);
    assert_eq!(part2(&input), 6050769);
}