pub fn part1(input: &str) -> u64 {
    solve(input, false)
}

pub fn part2(input: &str) -> u64 {
    solve(input, true)
}

pub fn solve(input: &str, do_jokers: bool) -> u64 {
    const NUM_CARDS: usize = 14;

    let mut hands = Vec::new();
    for (cards, bet) in input.lines().filter_map(|v| v.split_once(' ')) {
        let bet = bet.parse::<u32>().unwrap();

        let mut card_counts = [0_u8; NUM_CARDS];
        let mut cards_score = 0;
        for &card in &cards.as_bytes()[..5] {
            let card = match card {
                b'J' if do_jokers => 0,
                c @ b'2'..=b'9' => (c - b'2' + 1) as u32,
                b'T' => 9,
                b'J' => 10,
                b'Q' => 11,
                b'K' => 12,
                b'A' => 13,
                c => panic!("Invalid character {}", c as char),
            };

            card_counts[card as usize] += 1;
            cards_score = (cards_score << 4) + card;
        }

        let n_jokers = card_counts[0];
        let (max, next_max) = card_counts[1..].iter().fold(
            (0, 0),
            |(max, next_max), &v| if v > max {
                (v, max)
            } else if v > next_max {
                (max, v)
            } else {
                (max, next_max)
            }
        );

        let hand_score = (max + n_jokers) as u32 * 2 + (next_max == 2) as u32;
        hands.push(((hand_score << 4 * 5) + cards_score, bet));
    }

    hands.sort_unstable_by_key(|&(card, _)| card);
    hands.iter().zip(1_u64..).map(|(&(_, bet), rank)| bet as u64 * rank).sum()
}

#[test]
fn test_answers() {
    let input = std::fs::read_to_string("inputs/day7.txt").unwrap();
    assert_eq!(part1(&input), 248836197);
    assert_eq!(part2(&input), 251195607);
}