const EXAMPLE_INPUT: &str = r"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    // let input = EXAMPLE_INPUT;

    let timer = std::time::Instant::now();
    let part1_answer = part_1(&input);
    println!("Part 1 took {:6}s", timer.elapsed().as_secs_f64());

    let timer = std::time::Instant::now();
    let part2_answer = part_2(&input);
    println!("Part 2 took {:6}s", timer.elapsed().as_secs_f64());

    assert_eq!(part1_answer, 2563);
    assert_eq!(part2_answer, 70768);
}

fn part_1(input: &str) -> i64 {
    let mut sum = 0;

    let mut is_possible_game = true;
    let mut game_id = -1;
    let mut n = 0;
    for c in input.bytes() {
        match c {
            b'0'..=b'9' => n = n * 10 + (c - b'0') as i64,
            b'r' => { if n > 12 { is_possible_game = false; } n = 0; },
            b'g' => { if n > 13 { is_possible_game = false; } n = 0; },
            b'b' => { if n > 14 { is_possible_game = false; } n = 0; },
            b':' => {
                if is_possible_game && game_id >= 0 {
                    sum += game_id;
                }
                game_id = n;
                n = 0;
                is_possible_game = true;
            }
            _ => {}
        }
    }

    if is_possible_game && game_id >= 0 {
        sum += game_id;
    }

    sum
}

fn part_2(input: &str) -> i64 {
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