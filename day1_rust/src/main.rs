fn part_1(input: &str) -> i64 {
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

fn part_2(input: &str) -> i64 {
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

fn part_2_naiive(input: &str) -> i64 {
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

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut total_duration = 0.0;
    for _ in 0..100 {
        let start_time = std::time::Instant::now();
        let answer = part_1(&input);
        let duration = start_time.elapsed();

        println!("Part 1 answer: {}", answer);
        println!("Took {:.6} s", duration.as_secs_f64());
        total_duration += duration.as_secs_f64();
    }

    let mut total_duration2 = 0.0;
    for _ in 0..100 {
        let start_time = std::time::Instant::now();
        let answer = part_2(&input);
        let duration = start_time.elapsed();

        println!("Part 2 answer: {}", answer);
        println!("Took {:.6} s", duration.as_secs_f64());
        total_duration2 += duration.as_secs_f64();
    }

    println!("Part 1 Average: {:.6} s", total_duration / 100.0);
    println!("Part 2 Average: {:.6} s", total_duration2 / 100.0);
}
