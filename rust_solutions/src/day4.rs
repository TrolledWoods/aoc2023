use std::collections::VecDeque;

pub fn part1(input: &str) -> i64 {
    let mut input_set = 0_u128;
    // For now we assume the inputs are not above 128, which is invalid!
    // Something like this should be added as a fallback probably....
    //
    // let mut large_input_set = HashSet::default();
    // let mut is_large = false;

    let mut n = 0;
    let mut count = 0;
    let mut sum = 0;
    let mut win_section = true;
    for c in input.bytes().chain(Some(b'\n')) {
        if c.is_ascii_digit() {
            n = n*10 + (c-b'0') as u32;
        } else {
            if n != 0 {
                if win_section {
                    input_set |= 1 << n;
                } else {
                    if n < 128 && (input_set & (1 << n)) > 0 {
                        count += 1;
                    }
                }

                n = 0;
            }

            if c == b':' {
                win_section = true;
            } else if c == b'|' {
                win_section = false;
            } else if c == b'\n' {
                if count > 0 {
                    sum += 2_i64.pow(count - 1);
                }

                count = 0;
                input_set = 0;
            }
        }
    }

    sum
}

pub fn part2(input: &str) -> usize {
    let mut copies = VecDeque::new();
    let mut input_set = 0_u128;
    // For now we assume the inputs are not above 128, which is invalid!
    // let mut large_input_set = HashSet::default();
    // let mut is_large = false;

    let mut n = 0;
    let mut count = 0;
    let mut sum = 0;
    let mut win_section = true;
    for c in input.bytes().chain(Some(b'\n')) {
        if c.is_ascii_digit() {
            n = n*10 + (c-b'0') as u32;
        } else {
            if win_section {
                if n != 0 {
                    input_set |= 1 << n;
                }
            } else {
                if n != 0 && n < 128 && (input_set & (1 << n)) > 0 {
                    count += 1;
                }
            }

            if c == b':' {
                win_section = true;
            } else if c == b'|' {
                win_section = false;
            } else if c == b'\n' {
                let n_cards = copies.pop_front().unwrap_or(0) + 1;
                sum += n_cards;

                copies.resize(copies.len().max(count), 0);
                copies.range_mut(..count).for_each(|v| *v += n_cards);

                count = 0;
                input_set = 0;
            }

            n = 0;
        }
    }

    sum
}

#[test]
fn test_answers() {
    let input = std::fs::read_to_string("inputs/day4.txt").unwrap();
    assert_eq!(part1(&input), 21158);
    assert_eq!(part2(&input), 6050769);
}