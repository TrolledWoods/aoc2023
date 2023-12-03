use std::collections::{hash_map::Entry, HashMap};

pub fn part1(input: &str) -> i64 {
    let lines: Vec<_> = input.lines().map(|v| v.trim().as_bytes()).collect();

    let mut sum = 0;
    for (line_i, line) in lines.iter().enumerate() {
        let mut number = 0;
        let mut number_start = None;
        for (c_i, c) in line.iter().copied().chain(Some(b'.')).enumerate() {
            if c >= b'0' && c <= b'9' {
                number = number * 10 + (c - b'0') as i64;
                number_start.get_or_insert(c_i);
            } else if let Some(number_start) = number_start.take() {
                let mut valid_number = false;
                for s in &lines[line_i.saturating_sub(1)..lines.len().min(line_i + 2)] {
                    for sub_c_i in number_start.saturating_sub(1)..c_i + 1 {
                        if s.get(sub_c_i).is_some_and(|v| !(*v == b'.' || (*v >= b'0' &&  *v <= b'9'))) {
                            valid_number = true;
                        }
                    }
                }

                if valid_number {
                    sum += number;
                }
                number = 0;
            }
        }
    }

    sum
}

pub fn part2(input: &str) -> i64 {
    let lines: Vec<_> = input.lines().map(|v| v.trim().as_bytes()).collect();
    let mut gears = HashMap::new();
    enum Gear {
        One(i64),
        Two(i64),
        Three,
    }

    let mut sum = 0;
    for (line_i, line) in lines.iter().enumerate() {
        let mut number = 0;
        let mut number_start = None;
        for (c_i, c) in line.iter().copied().chain(Some(b'.')).enumerate() {
            if c >= b'0' && c <= b'9' {
                number = number * 10 + (c - b'0') as i64;
                number_start.get_or_insert(c_i);
            } else if let Some(number_start) = number_start.take() {
                for sub_line_i in line_i.saturating_sub(1)..line_i + 2 {
                    if let Some(s) = lines.get(sub_line_i) {
                        for sub_c_i in number_start.saturating_sub(1)..c_i + 1 {
                            if s.get(sub_c_i).is_some_and(|v| *v == b'*') {
                                match gears.entry((s, sub_c_i)) {
                                    Entry::Vacant(entry) => {
                                        entry.insert(Gear::One(number));
                                    }
                                    Entry::Occupied(mut entry) => {
                                        match *entry.get() {
                                            Gear::One(prev) => {
                                                sum += prev * number;
                                                *entry.get_mut() = Gear::Two(prev * number);
                                            }
                                            Gear::Two(v) => {
                                                sum -= v;
                                                *entry.get_mut() = Gear::Three;
                                            }
                                            Gear::Three => {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                number = 0;
            }
        }
    }

    sum
}
    
#[test]
fn test_answers() {
    let input = std::fs::read_to_string("inputs/day3.txt").unwrap();
    assert_eq!(part1(&input), 531932);
    assert_eq!(part2(&input), 73646890);
}