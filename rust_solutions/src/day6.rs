use std::{ops::{Range, MulAssign}, str::FromStr, fmt::Debug};
use num_bigint::BigUint;
use num_traits::{Num, Pow};
use num_integer::Roots;

pub fn part1_bigint(input: &str) -> BigUint {
    part1::<BigUint>(input)
}

pub fn part2_bigint(input: &str) -> BigUint {
    part2::<BigUint>(input)
}

pub fn part1<T: FromStr + Clone + Num + Ord + Roots + Pow<u32, Output = T> + From<u32> + MulAssign>(input: &str) -> T where <T as FromStr>::Err: Debug {
    let mut lines = input.lines().map(|v| v.trim());
    let times = lines.by_ref().find_map(|l| l.strip_prefix("Time:")).unwrap().split_whitespace().map(|v| v.parse::<T>().unwrap()).collect::<Vec<_>>();
    let distances = lines.by_ref().find_map(|l| l.strip_prefix("Distance:")).unwrap().split_whitespace().map(|v| v.parse::<T>().unwrap()).collect::<Vec<_>>();

    let mut prod = T::from(1_u32);
    for (time, distance) in times.into_iter().zip(distances) {
        let range = get_optimum_range(time, distance);
        prod *= range.end - range.start
    }
    prod
}

pub fn part2<T: FromStr + Clone + Num + Ord + Roots + Pow<u32, Output = T> + From<u32>>(input: &str) -> T where <T as FromStr>::Err: Debug {
    let mut lines = input.lines().map(|v| v.trim());
    let time = lines.by_ref().find_map(|l| l.strip_prefix("Time:")).unwrap().split_whitespace().collect::<String>().parse::<T>().unwrap();
    let distance = lines.by_ref().find_map(|l| l.strip_prefix("Distance:")).unwrap().split_whitespace().collect::<String>().parse::<T>().unwrap();
    let range = get_optimum_range(time, distance);
    range.end - range.start
}

fn get_optimum_range<T: Clone + Num + Ord + Roots + Pow<u32, Output = T> + From<u32>>(time: T, distance: T) -> Range<T> {
    let pow = time.clone().pow(2);
    let dist = distance * T::from(4_u32);
    if pow >= dist {
        let v = pow - dist;
        let s = v.sqrt();
        let is_exact = (s.clone().pow(2) == v) as u32;

        // let lower = (time.saturating_sub(s) + 1 + is_exact) / 2;
        // let upper = (time + s - is_exact).min(time * 2) / 2;
        let lower = if &time >= &s {
            (time.clone() - s.clone() + T::from(1 + is_exact)) / T::from(2)
        } else {
            0_u32.into()
        };
        let upper = (time.clone() + s - T::from(is_exact)).min(time * T::from(2)) / T::from(2);

        lower .. upper + T::from(1)
    } else {
        0_u32.into()..0_u32.into()
    }
}

#[test]
fn test_optimum_ranges() {
    fn test_single_range<T: Clone + Num + Ord + Roots + Pow<u32, Output = T> + From<u32>>(time: T, distance: T) {
        let wins = |button_time: T| button_time.clone() * (time.clone() - button_time) > distance.clone();
        let optimum = get_optimum_range(time.clone(), distance.clone());

        if !optimum.is_empty() {
            assert!(wins(optimum.start.clone()));
            assert!(!wins(optimum.start - T::from(1)));
            assert!(wins(optimum.end.clone() - T::from(1)));
            assert!(!wins(optimum.end));
        } else {
            assert!(optimum.start.clone() * (time - optimum.start) <= distance);
        }
    }

    test_single_range(48_u64, 390_u64);
    test_single_range(98_u64, 1103_u64);
    test_single_range(90_u64, 1112_u64);
    test_single_range(83_u64, 1360_u64);
    test_single_range(BigUint::from(u64::MAX / 2), BigUint::from(u64::MAX / 3));
    test_single_range(48989083_u64, 390110311121360_u64);

    for time in 1..1000_u32 {
        for distance in 1..1000 {
            test_single_range(time, distance);
        }
    }
}

#[test]
fn test_answers() {
    let input = std::fs::read_to_string("inputs/day6.txt").unwrap();
    assert_eq!(part1(&input), 4568778);
    assert_eq!(part2(&input), 28973936);
}