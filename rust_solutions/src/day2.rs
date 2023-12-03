pub fn part_1(input: &str) -> i64 {
    let mut sum = 0;

    let mut game_id = 0;
    let mut n = 0;
    for c in input.bytes() {
        match c {
            b'0'..=b'9' => n = n * 10 + (c - b'0') as i64,
            b'r' => { game_id *= (n <= 12) as i64; n = 0; },
            b'g' => { game_id *= (n <= 13) as i64; n = 0; },
            b'b' => { game_id *= (n <= 14) as i64; n = 0; },
            b':' => {
                sum += game_id;
                game_id = n;
                n = 0;
            }
            _ => {}
        }
    }

    sum += game_id;

    sum
}

pub fn part_2(input: &str) -> i64 {
    let mut sum = 0;

    let mut min_colors = [0; 3];
    let mut n = 0;
    for c in input.bytes() {
        match c {
            b'0'..=b'9' => n = n * 10 + (c - b'0') as i64,
            b'r' => { min_colors[0] = i64::max(min_colors[0], n); n = 0; },
            b'g' => { min_colors[1] = i64::max(min_colors[1], n); n = 0; },
            b'b' => { min_colors[2] = i64::max(min_colors[2], n); n = 0; },
            b':' => {
                sum += min_colors.iter().product::<i64>();
                n = 0;
                min_colors.fill(0);
            }
            _ => {}
        }
    }

    sum += min_colors.iter().product::<i64>();

    sum
}

#[test]
fn test_answers() {
    let input = std::fs::read_to_string("inputs/day2.txt").unwrap();
    let part1_answer = part_1(&input);
    assert_eq!(part1_answer, 2563);
    let part2_answer = part_2(&input);
    assert_eq!(part2_answer, 70768);
}