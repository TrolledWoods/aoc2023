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

    let mut seeds = input_lines
        .by_ref()
        .find_map(|v| v.strip_prefix("seeds: "))
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse::<u64>().unwrap())
        .array_chunks()
        .map(|[start, len]| start .. start + len)
        .collect::<Vec<_>>();

    // I cheat and assume the maps are in order :)
    let mut mapped_seeds = Vec::new();
    let mut unmapped_seeds = Vec::new();
    while input_lines.by_ref().any(|v| v.contains("map")) {
        for line in input_lines.by_ref().take_while(|v| !v.is_empty()) {
            let mut parts = line.split_whitespace().map(|v| v.parse::<u64>().unwrap());
            let target = parts.next().unwrap();
            let source_start = parts.next().unwrap();
            let len = parts.next().unwrap();
            let source = source_start .. source_start + len;

            for seed in seeds.drain(..) {
                let overlap = seed.start.max(source.start) .. seed.end.min(source.end);

                if overlap.is_empty() {
                    unmapped_seeds.push(seed);
                } else {
                    mapped_seeds.push(overlap.start - source.start + target .. overlap.end - source.start + target);

                    let left = seed.start .. overlap.start;
                    if !left.is_empty() {
                        unmapped_seeds.push(left);
                    }

                    let right = overlap.end .. seed.end;
                    if !right.is_empty() {
                        unmapped_seeds.push(right);
                    }
                }
            }

            // The remaining seed ranges that are not yet mapped by this mapper should
            // be swapped back in to be mapped with the next one
            std::mem::swap(&mut unmapped_seeds, &mut seeds);
        }

        seeds.extend(mapped_seeds.drain(..));
    }

    seeds.iter().map(|seed| seed.start).min().unwrap()
}

#[test]
fn test_answers() {
    let input = std::fs::read_to_string("inputs/day5.txt").unwrap();
    assert_eq!(part1(&input), 196167384);
    assert_eq!(part2(&input), 125742456);
}