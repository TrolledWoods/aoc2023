use std::collections::hash_map::Entry;
use ahash::AHashMap;

pub fn part1(input: &str) -> i64 {
    let stride = input.find('\n').unwrap_or(input.len() * 2) + 1;

    let mut number = 0;
    let mut number_start = None;
    let mut sum = 0;
    for (byte_i, c) in input.bytes().chain(Some(b'\n')).enumerate() {
        match c {
            b'0'..=b'9' => {
                number = number * 10 + (c - b'0') as i64;
                number_start.get_or_insert(byte_i);
            }
            _ => {
                if let Some(number_start) = number_start.take() {
                    let number_len = byte_i - number_start;
                    let valid_number = 
                        [number_start.checked_sub(stride), Some(number_start), Some(number_start + stride)]
                        .into_iter()
                        .flatten()
                        .flat_map(|idx| (idx.saturating_sub(1) .. idx + number_len + 1))
                        .map(|i| input.as_bytes().get(i).copied().unwrap_or(b' '))
                        .any(|v| !(v == b'.' || v.is_ascii_whitespace() || (v >= b'0' &&  v <= b'9')));

                    if valid_number {
                        sum += number;
                    }
                    number = 0;
                }
            }
        }
    }

    sum
}

pub fn part2(input: &str) -> i64 {
    let mut gears = AHashMap::new();
    enum Gear {
        One(i64),
        Two(i64),
        Three,
    }

    let stride = input.find('\n').unwrap_or(input.len() * 2) + 1;

    let mut number = 0;
    let mut number_start = None;
    let mut sum = 0;
    for (byte_i, c) in input.bytes().chain(Some(b'\n')).enumerate() {
        match c {
            b'0'..=b'9' => {
                number = number * 10 + (c - b'0') as i64;
                number_start.get_or_insert(byte_i);
            }
            _ => {
                if let Some(number_start) = number_start.take() {
                    let number_len = byte_i - number_start;
                    let stars = [number_start.checked_sub(stride), Some(number_start), Some(number_start + stride)]
                        .into_iter()
                        .flatten()
                        .flat_map(|idx| (idx.saturating_sub(1) .. idx + number_len + 1))
                        .filter(|&i| input.as_bytes().get(i) == Some(&b'*'));

                    for star in stars {
                        match gears.entry(star) {
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

                    number = 0;
                }
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