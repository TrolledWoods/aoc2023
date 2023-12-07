pub fn part1(input: &str) -> usize {
    const NUM_CARDS: usize = 8 + 5;

    let mut hands = Vec::with_capacity(input.len() / 8);
    for (cards, bet) in input.lines().filter_map(|v| v.split_once(' ')) {
        let mut card_counts = [0_u8; NUM_CARDS];
        let mut card_value = 0;
        for &card in &cards.as_bytes()[..5] {
            let card = match card {
                c @ b'2'..=b'9' => (c - b'2') as u32,
                b'T' => 8,
                b'J' => 9,
                b'Q' => 10,
                b'K' => 11,
                b'A' => 12,
                c => panic!("Invalid character {}", c as char),
            };

            card_counts[card as usize] += 1;
            card_value = card_value * 16 + card;
        }

        let (max, next_max) = get_max_two(card_counts.into_iter());
        let kind = (max) as u32 * 2 + (next_max == 2) as u32;
        let bet = bet.parse::<u32>().unwrap();
        hands.push(((kind << (4 * 5)) + card_value, bet));
    }

    hands.sort_unstable_by_key(|v| v.0);
    hands.iter().enumerate().map(|(i, &(_, bet))| (i + 1) * bet as usize).sum()
}

pub fn part2(input: &str) -> usize {
    const NUM_CARDS: usize = 8 + 4;

    let mut hands = Vec::with_capacity(input.len() / 8);
    for (cards, bet) in input.lines().filter_map(|v| v.split_once(' ')) {
        let mut n_jokers = 0;
        let mut card_counts = [0_u8; NUM_CARDS];
        let mut card_value = 0;
        for &card in &cards.as_bytes()[..5] {
            let card = match card {
                b'J' => 12,
                c @ b'2'..=b'9' => (c - b'2') as u32,
                b'T' => 8,
                b'Q' => 9,
                b'K' => 10,
                b'A' => 11,
                c => panic!("Invalid character {}", c as char),
            };

            if card == 12 {
                n_jokers += 1;
                card_value *= 16;
            } else {
                card_counts[card as usize] += 1;
                card_value = card_value * 16 + card + 1;
            }
        }

        let (max, next_max) = get_max_two(card_counts.into_iter());
        let kind = (max + n_jokers) as u32 * 2 + (next_max == 2) as u32;
        let bet = bet.parse::<u32>().unwrap();
        hands.push(((kind << (4 * 5)) + card_value, bet));
    }

    hands.sort_unstable_by_key(|v| v.0);
    hands.iter().enumerate().map(|(i, &(_, bet))| (i + 1) * bet as usize).sum()
}

#[test]
fn test_answers() {
    let input = std::fs::read_to_string("inputs/day7.txt").unwrap();
    assert_eq!(part1(&input), 248836197);
    assert_eq!(part2(&input), 251195607);
}

fn get_max_two<T: Ord + Default>(values: impl Iterator<Item = T>) -> (T, T) {
    let mut max = T::default();
    let mut next_max = T::default();

    for value in values {
        if value > max {
            next_max = std::mem::replace(&mut max, value);
        } else if value > next_max {
            next_max = value;
        }
    }

    (max, next_max)
}