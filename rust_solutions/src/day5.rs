use rayon::prelude::*;

pub fn part2_brute(input: &str) -> u64 {
    let mut input_lines = input.lines().map(|v| v.trim());

    let initial_seeds = input_lines
        .by_ref()
        .find_map(|v| v.strip_prefix("seeds: "))
        .unwrap()
        .split_whitespace()
        .filter_map(|v| v.parse::<u64>().ok())
        .array_chunks()
        .collect::<Vec<_>>();

    struct MapEntry {
        target: u64,
        source: u64,
        end: u64,
    }

    let mut maps = Vec::new();
    let mut ranges = Vec::new();
    // I cheat and assume the maps are in order :)
    while input_lines.by_ref().any(|v| v.contains("map")) {
        let start = ranges.len();
        let mut has_zero_map = false;
        for line in input_lines.by_ref().take_while(|v| !v.is_empty()) {
            let mut parts = line.split_whitespace().map(|v| v.parse::<u64>().unwrap());
            let target = parts.next().unwrap();
            let source = parts.next().unwrap();
            let len = parts.next().unwrap();
            if source == 0 {
                has_zero_map = true;
            }
            ranges.push(MapEntry { target, source, end: source + len });
        }
        if !has_zero_map {
            ranges.push(MapEntry { target: 0, source: 0, end: 0 });
        }
        ranges[start..].sort_by_key(|e| e.source);
        maps.push(start..ranges.len());
    }

    let maps = maps.into_iter().map(|range| &ranges[range.start .. range.end]).collect::<Vec<_>>();

    initial_seeds
        .par_iter()
        .flat_map(|&[a, b]| (a..a+b))
        .map(|mut seed| {
            for &map in &maps {
                let idx = match map.binary_search_by_key(&seed, |v| v.source) {
                    Ok(idx) => idx,
                    Err(idx) => idx - 1,
                };

                if seed < map[idx].end {
                    seed = seed - map[idx].source + map[idx].target;
                }
            }

            seed
        })
        .min()
        .unwrap()
}

pub fn part1(input: &str) -> u64 {
    let mut input_lines = input.lines().map(|v| v.trim());

    enum SeedState {
        Original,
        Mapped,
    }

    let mut initial_seeds = input_lines
        .by_ref()
        .find_map(|v| v.strip_prefix("seeds: "))
        .unwrap()
        .split_whitespace()
        .filter_map(|v| v.parse::<u64>().ok())
        .map(|v| (SeedState::Original, v))
        .collect::<Vec<_>>();

    // I cheat and assume the maps are in order :)
    while input_lines.by_ref().any(|v| v.contains("map")) {
        for line in input_lines.by_ref().take_while(|v| !v.is_empty()) {
            let mut parts = line.split_whitespace().map(|v| v.parse::<u64>().unwrap());
            let target = parts.next().unwrap();
            let source = parts.next().unwrap();
            let len = parts.next().unwrap();

            for (state, seed) in initial_seeds.iter_mut() {
                if !matches!(state, SeedState::Mapped) && source <= *seed && *seed < source + len {
                    *seed = *seed - source + target;
                    *state = SeedState::Mapped;
                }
            }
        }

        for (state, _) in initial_seeds.iter_mut() {
            *state = SeedState::Original;
        }
    }

    initial_seeds.into_iter().map(|(_, v)| v).min().unwrap()
}

pub fn part2(input: &str) -> u64 {
    let mut input_lines = input.lines().map(|v| v.trim());

    let mut initial_seeds = input_lines
        .by_ref()
        .find_map(|v| v.strip_prefix("seeds: "))
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse::<u64>().unwrap())
        .array_chunks()
        .map(|[a, b]| [a, a + b - 1])
        .collect::<Vec<_>>();

    // I cheat and assume the maps are in order :)
    let mut target_seeds = Vec::new();
    while input_lines.by_ref().any(|v| v.contains("map")) {
        for line in input_lines.by_ref().take_while(|v| !v.is_empty()) {
            let mut parts = line.split_whitespace().map(|v| v.parse::<u64>().unwrap());
            let target_a = parts.next().unwrap();
            let source_a = parts.next().unwrap();
            let len = parts.next().unwrap();
            let source_b = source_a + len - 1;

            for idx in 0..initial_seeds.len() {
                let seed = &mut initial_seeds[idx];
                let [seed_a, seed_b] = *seed;

                // Compute the range of overlap between the seed range and the map source range
                let a = seed_a.max(source_a);
                let b = seed_b.min(source_b);

                // Is the overlapping non-empty?
                if a <= b {
                    // Map the overlapping range
                    target_seeds.push([a - source_a + target_a, b - source_a + target_a]);

                    *seed = [u64::MAX, 0];
                    // Add the remaining left and right ranges back to the un-mapped seeds list, if
                    // they're non-empty.
                    if seed_a < a {
                        initial_seeds.push([seed_a, a - 1]);
                    }
                    if b < seed_b {
                        initial_seeds.push([b + 1, seed_b]);
                    }
                }
            }

            initial_seeds.retain(|[a, b]| b >= a);
        }

        initial_seeds.extend(target_seeds.drain(..));
    }

    initial_seeds.into_iter().map(|[a, _]| a).min().unwrap()
}

#[test]
fn test_answers() {
    let input = std::fs::read_to_string("inputs/day5.txt").unwrap();
    assert_eq!(part1(&input), 196167384);
    assert_eq!(part2(&input), 125742456);
}