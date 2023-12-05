use clap::{Parser, ValueEnum};
use rand::prelude::*;

#[derive(Debug, Clone, ValueEnum)]
enum Day {
    Day1,
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Which advent of code day to use
    #[arg(value_enum)]
    day: Day,

    /// The seed to use for the output generation
    #[arg(default_value_t = 0)]
    seed: u64,

    /// The size of the output
    #[arg(short, long, default_value_t = 1000)]
    size: usize,

    #[arg(short, long, default_value_t = String::from("a.txt"))]
    output: String,
}

fn main() {
    let mut args = Args::parse();
    while args.seed == 0 {
        args.seed = rand::thread_rng().gen();
    }

    let generated = match args.day {
        Day::Day1 => {
            generate_day_1(args.size, &mut SmallRng::seed_from_u64(args.seed))
        }
    };

    std::fs::write(&args.output, generated.input).expect("Failed to write output file");

    println!("--seed {} --size {}", args.seed, args.size);
    println!("Expected part 1 answer: {}", generated.answer_part1);
    println!("Expected part 2 answer: {}", generated.answer_part2);
}

struct GeneratedResult {
    input: Vec<u8>,
    answer_part1: String,
    answer_part2: String,
}

fn generate_day_1(size: usize, rand: &mut impl Rng) -> GeneratedResult {
    let mut input = Vec::new();

    for _ in 0..size {
        let line_length = rand.gen_range(4..=20);
        let n_strings = rand.gen_range(1..=10);

        let start = input.len();
        input.extend(std::iter::repeat_with(|| rand.gen_range(b'a'..=b'z')).take(line_length));
        let new_bit = &mut input[start..];

        let mut i = 0;
        while i < n_strings {
            let (name, _) = NAME_TO_DIGITS[rand.gen_range(0..NAME_TO_DIGITS.len())];
            if name.len() > line_length {
                continue;
            }

            let pos = rand.gen_range(0..=line_length - name.len());
            new_bit[pos..pos + name.len()].copy_from_slice(name);

            i += 1;
        }

        input.push(b'\n');
    }

    let part1 = solve_day1(&input, NAME_TO_DIGITS_PART1);
    let part2 = solve_day1(&input, NAME_TO_DIGITS);

    GeneratedResult {
        input,
        answer_part1: format!("{}", part1),
        answer_part2: format!("{}", part2),
    }
}

const NAME_TO_DIGITS_PART1: &[(&[u8], u8)] = &[
    (b"1", 1),
    (b"2", 2),
    (b"3", 3),
    (b"4", 4),
    (b"5", 5),
    (b"6", 6),
    (b"7", 7),
    (b"8", 8),
    (b"9", 9),
];

const NAME_TO_DIGITS: &[(&[u8], u8)] = &[
    (b"1",     1),
    (b"one",   1),
    (b"2",     2),
    (b"two",   2),
    (b"3",     3),
    (b"three", 3),
    (b"4",     4),
    (b"four",  4),
    (b"5",     5),
    (b"five",  5),
    (b"6",     6),
    (b"six",   6),
    (b"7",     7),
    (b"seven", 7),
    (b"8",     8),
    (b"eight", 8),
    (b"9",     9),
    (b"nine",  9),
];

fn solve_day1(input: &[u8], name_to_digits: &[(&[u8], u8)]) -> i64 {
    let mut f = 0_u8;
    let mut l = 0_u8;
    let mut sum = 0;

    for (i, &c) in input.iter().enumerate() {
        for &(name, digit) in name_to_digits {
            if input[i..].starts_with(name) {
                if f == 0 {
                    f = digit;
                }
                l = digit;
            }
        }

        if c == b'\n' {
            if f != 0 {
                let d = f * 10 + l;
                sum += d as i64;
            }

            f = 0;
            l = 0;
        }
    }

    if f != 0 {
        let d = f * 10 + l;
        sum += d as i64;
    }

    sum
}