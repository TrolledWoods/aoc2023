pub fn part_1(input: &str) -> i64 {
    let mut f = 0_u8;
    let mut l = 0_u8;
    let mut sum = 0;

    for c in input.bytes() {
        match c {
            b'1'..=b'9' => {
                if f == 0 { f = c - b'0'; }
                l = c - b'0';
            }
            b'\n' => {
                if f != 0 {
                    sum += (f * 10 + l) as i64;
                    f = 0;
                }
            }
            _ => {}
        }
    }

    if f != 0 {
        sum += (f * 10 + l) as i64;
    }

    sum
}

#[derive(Clone, Copy)]
#[repr(u8)]
enum StateMachine {
    None,
    O,
    On,
    T,
    Tw,
    Th,
    Thr,
    Thre,
    F,
    Fo,
    Fou,
    Fi,
    Fiv,
    S,
    Si,
    Se,
    Sev,
    Seve,
    E,
    Ei,
    Eig,
    Eigh,
    N,
    Ni,
    Nin,
}

pub fn part_2(input: &str) -> i64 {
    let mut f = 0_u8;
    let mut l = 0_u8;
    let mut sum = 0;

    fn add_digit(f: &mut u8, l: &mut u8, digit: u8) {
        if *f == 0 {
            *f = digit;
        }
        *l = digit;
    }

    let mut state = StateMachine::None;

    for c in input.bytes() {
        use StateMachine::*;
        match (state, c) {
            (O,   b'n') => state = On,
            (T,   b'w') => state = Tw,
            (T,   b'h') => state = Th,
            (Th,  b'r') => state = Thr,
            (Thr, b'e') => state = Thre,
            (F,   b'o') => state = Fo,
            (Fo,  b'u') => state = Fou,
            (F,   b'i') => state = Fi,
            (Fi,  b'v') => state = Fiv,
            (S,   b'i') => state = Si,
            (S,   b'e') => state = Se,
            (Se,  b'v') => state = Sev,
            (Sev, b'e') => state = Seve,
            (E,   b'i') => state = Ei,
            (Thre, b'i') => state = Ei,
            (Seve, b'i') => state = Ei,
            (Se,  b'i') => state = Ei,
            (Ei,  b'g') => state = Eig,
            (Eig, b'h') => state = Eigh,
            (N,   b'i') => state = Ni,
            (Nin, b'i') => state = Ni,
            (On,  b'i') => state = Ni,
            (Ni,  b'n') => state = Nin,

            (On,   b'e') => { add_digit(&mut f, &mut l, 1); state = E; },
            (Tw,   b'o') => { add_digit(&mut f, &mut l, 2); state = O; },
            (Thre, b'e') => { add_digit(&mut f, &mut l, 3); state = E; },
            (Fou,  b'r') => { add_digit(&mut f, &mut l, 4); state = None; },
            (Fiv,  b'e') => { add_digit(&mut f, &mut l, 5); state = E; },
            (Si,   b'x') => { add_digit(&mut f, &mut l, 6); state = None; },
            (Seve, b'n') => { add_digit(&mut f, &mut l, 7); state = N; },
            (Eigh, b't') => { add_digit(&mut f, &mut l, 8); state = T; },
            (Nin,  b'e') => { add_digit(&mut f, &mut l, 9); state = E; },

            (_, b'1') => { add_digit(&mut f, &mut l, 1); state = None; },
            (_, b'2') => { add_digit(&mut f, &mut l, 2); state = None; },
            (_, b'3') => { add_digit(&mut f, &mut l, 3); state = None; },
            (_, b'4') => { add_digit(&mut f, &mut l, 4); state = None; },
            (_, b'5') => { add_digit(&mut f, &mut l, 5); state = None; },
            (_, b'6') => { add_digit(&mut f, &mut l, 6); state = None; },
            (_, b'7') => { add_digit(&mut f, &mut l, 7); state = None; },
            (_, b'8') => { add_digit(&mut f, &mut l, 8); state = None; },
            (_, b'9') => { add_digit(&mut f, &mut l, 9); state = None; },

            (_, b'o') => state = O,
            (_, b't') => state = T,
            (_, b'f') => state = F,
            (_, b's') => state = S,
            (_, b'e') => state = E,
            (_, b'n') => state = N,

            (_, b'\n') => {
                if f != 0 {
                    let d = f * 10 + l;
                    sum += d as i64;
                }

                f = 0;
                l = 0;

                state = None;
            }

            _ => state = None,
        }
    }

    if f != 0 {
        let d = f * 10 + l;
        sum += d as i64;
    }

    sum
}

pub fn part_2_naiive(input: &str) -> i64 {
    let mut f = 0_u8;
    let mut l = 0_u8;
    let mut sum = 0;

    fn add_digit(f: &mut u8, l: &mut u8, digit: u8) {
        if *f == 0 {
            *f = digit;
        }
        *l = digit;
    }

    for (i, c) in input.char_indices() {
        if let Some(digit) = c.to_digit(10) {
            add_digit(&mut f, &mut l, digit as u8);
        } else if c == '\n' {
            if f != 0 {
                let d = f * 10 + l;
                sum += d as i64;
            }

            f = 0;
            l = 0;
        } else if input[i..].starts_with("one") {
            add_digit(&mut f, &mut l, 1);
        } else if input[i..].starts_with("two") {
            add_digit(&mut f, &mut l, 2);
        } else if input[i..].starts_with("three") {
            add_digit(&mut f, &mut l, 3);
        } else if input[i..].starts_with("four") {
            add_digit(&mut f, &mut l, 4);
        } else if input[i..].starts_with("five") {
            add_digit(&mut f, &mut l, 5);
        } else if input[i..].starts_with("six") {
            add_digit(&mut f, &mut l, 6);
        } else if input[i..].starts_with("seven") {
            add_digit(&mut f, &mut l, 7);
        } else if input[i..].starts_with("eight") {
            add_digit(&mut f, &mut l, 8);
        } else if input[i..].starts_with("nine") {
            add_digit(&mut f, &mut l, 9);
        }
    }

    if f != 0 {
        let d = f * 10 + l;
        sum += d as i64;
    }

    sum
}

pub fn part_2_less_naiive(input: &str) -> i64 {
    let mut f = true;
    let mut l = 0_u8;
    let mut sum = 0;

    #[inline(always)]
    fn add_digit(sum: &mut i64, f: &mut bool, l: &mut u8, digit: u8) {
        if *f {
            *sum += (digit * 10) as i64;
            *f = false;
        }
        *l = digit;
    }

    for sect in input.as_bytes().array_windows::<5>() {
        match sect {
            [b'1', ..] | [b'o', b'n', b'e', _   , _   ] => add_digit(&mut sum, &mut f, &mut l, 1),
            [b'2', ..] | [b't', b'w', b'o', _   , _   ] => add_digit(&mut sum, &mut f, &mut l, 2),
            [b'3', ..] | [b't', b'h', b'r', b'e', b'e'] => add_digit(&mut sum, &mut f, &mut l, 3),
            [b'4', ..] | [b'f', b'o', b'u', b'r', _   ] => add_digit(&mut sum, &mut f, &mut l, 4),
            [b'5', ..] | [b'f', b'i', b'v', b'e', _   ] => add_digit(&mut sum, &mut f, &mut l, 5),
            [b'6', ..] | [b's', b'i', b'x', _   , _   ] => add_digit(&mut sum, &mut f, &mut l, 6),
            [b'7', ..] | [b's', b'e', b'v', b'e', b'n'] => add_digit(&mut sum, &mut f, &mut l, 7),
            [b'8', ..] | [b'e', b'i', b'g', b'h', b't'] => add_digit(&mut sum, &mut f, &mut l, 8),
            [b'9', ..] | [b'n', b'i', b'n', b'e', _   ] => add_digit(&mut sum, &mut f, &mut l, 9),
            [b'\n', ..] => {
                sum += l as i64;
                f = true;
                l = 0;
            }
            _ => {}
        }
    }

    let remainder = &input.as_bytes()[input.len().saturating_sub(4)..];
    for i in 0..remainder.len() {
        let sect = &remainder[i..];
        match sect {
            [b'1', ..] | [b'o', b'n', b'e', ..] => add_digit(&mut sum, &mut f, &mut l, 1),
            [b'2', ..] | [b't', b'w', b'o', ..] => add_digit(&mut sum, &mut f, &mut l, 2),
            [b'3', ..] | [b't', b'h', b'r', b'e', b'e', ..] => add_digit(&mut sum, &mut f, &mut l, 3),
            [b'4', ..] | [b'f', b'o', b'u', b'r', ..] => add_digit(&mut sum, &mut f, &mut l, 4),
            [b'5', ..] | [b'f', b'i', b'v', b'e', ..] => add_digit(&mut sum, &mut f, &mut l, 5),
            [b'6', ..] | [b's', b'i', b'x', ..] => add_digit(&mut sum, &mut f, &mut l, 6),
            [b'7', ..] | [b's', b'e', b'v', b'e', b'n', ..] => add_digit(&mut sum, &mut f, &mut l, 7),
            [b'8', ..] | [b'e', b'i', b'g', b'h', b't', ..] => add_digit(&mut sum, &mut f, &mut l, 8),
            [b'9', ..] | [b'n', b'i', b'n', b'e', ..] => add_digit(&mut sum, &mut f, &mut l, 9),
            [b'\n', ..] => {
                sum += l as i64;
                f = true;
                l = 0;
            }
            _ => {}
        }
    }

    sum += l as i64;

    sum
}

pub fn part_1_dandi(input: &str) -> i64 {
    input.lines()
        .map(|line| {
            let first = line.chars().find_map(|v| v.to_digit(10)).unwrap_or(0);
            let last  = line.chars().rev().find_map(|v| v.to_digit(10)).unwrap_or(0);
            (first * 10 + last) as i64
        })
        .sum::<i64>()
}

pub fn part_1_dandi_ascii(input: &str) -> i64 {
    input.as_bytes()
        .split(|v| *v == b'\n')
        .map(|line| {
            let first = line.iter().find(|v| **v >= b'1' && **v <= b'9').map(|v| v - b'0').unwrap_or(0);
            let last  = line.iter().rev().find(|v| **v >= b'1' && **v <= b'9').map(|v| v - b'0').unwrap_or(0);
            (first * 10 + last) as i64
        })
        .sum::<i64>()
}

pub fn part_2_dandi(input: &str) -> i64 {
    fn find_digit(sect: &[u8]) -> Option<u8> {
        match sect {
            [b'1', ..] | [b'o', b'n', b'e', ..] => Some(1),
            [b'2', ..] | [b't', b'w', b'o', ..] => Some(2),
            [b'3', ..] | [b't', b'h', b'r', b'e', b'e', ..] => Some(3),
            [b'4', ..] | [b'f', b'o', b'u', b'r', ..] => Some(4),
            [b'5', ..] | [b'f', b'i', b'v', b'e', ..] => Some(5),
            [b'6', ..] | [b's', b'i', b'x', ..] => Some(6),
            [b'7', ..] | [b's', b'e', b'v', b'e', b'n', ..] => Some(7),
            [b'8', ..] | [b'e', b'i', b'g', b'h', b't', ..] => Some(8),
            [b'9', ..] | [b'n', b'i', b'n', b'e', ..] => Some(9),
            _ => None,
        }
    }

    let mut sum = 0;
    for line in input.as_bytes().split(|v| *v == b'\n') {
        let first = (0..line.len()).map(|v| &line[v..]).find_map(find_digit).unwrap_or(0);
        let last  = (0..line.len()).map(|v| &line[v..]).rev().find_map(find_digit).unwrap_or(0);
        sum += (first * 10 + last) as i64;
    }
    sum
}

pub fn part_2_dandi_naiive(input: &str) -> i64 {
    #[inline]
    fn find_digit(sect: &[u8]) -> Option<u8> {
        if sect[0] >= b'1' && sect[0] <= b'9' {
            Some(sect[0] - b'0')
        } else if sect.starts_with(b"one") {
            Some(1)
        } else if sect.starts_with(b"two") {
            Some(2)
        } else if sect.starts_with(b"three") {
            Some(3)
        } else if sect.starts_with(b"four") {
            Some(4)
        } else if sect.starts_with(b"five") {
            Some(5)
        } else if sect.starts_with(b"six") {
            Some(6)
        } else if sect.starts_with(b"seven") {
            Some(7)
        } else if sect.starts_with(b"eight") {
            Some(8)
        } else if sect.starts_with(b"nine") {
            Some(9)
        } else {
            None
        }
    }

    let mut sum = 0;
    for line in input.as_bytes().split(|v| *v == b'\n') {
        let first = (0..line.len()).map(|v| &line[v..]).find_map(find_digit).unwrap_or(0);
        let last  = (0..line.len()).map(|v| &line[v..]).rev().find_map(find_digit).unwrap_or(0);
        sum += (first * 10 + last) as i64;
    }
    sum
}

#[test]
fn test_answers() {
    let input = std::fs::read_to_string("inputs/day1.txt").unwrap();
    assert_eq!(part_1(&input), 53921);
    assert_eq!(part_1_dandi(&input), 53921);
    assert_eq!(part_2(&input), 54676);
    assert_eq!(part_2_naiive(&input), 54676);
    assert_eq!(part_2_less_naiive(&input), 54676);
    assert_eq!(part_2_dandi(&input), 54676);
    assert_eq!(part_2_dandi_naiive(&input), 54676);
}