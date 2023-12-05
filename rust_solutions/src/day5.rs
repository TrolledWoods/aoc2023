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
            let parts = line.split_whitespace().map(|v| v.parse::<u64>().unwrap()).collect::<Vec<_>>();
            let source = parts[1];
            let target = parts[0];
            let len = parts[2];

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
            let parts = line.split_whitespace().map(|v| v.parse::<u64>().unwrap()).collect::<Vec<_>>();
            let source_a = parts[1];
            let target_a = parts[0];
            let len = parts[2];
            let source_b = source_a + len - 1;
            let target_b = target_a + len - 1;

            for idx in 0..initial_seeds.len() {
                let seed = &mut initial_seeds[idx];
                let [seed_a, seed_b] = *seed;
                if seed_b < seed_a {
                    // Don't do anything, this seed is invalid (or was removed or something)!
                } else if seed_b < source_a || seed_a > source_b {
                    *seed = [seed_a, seed_b];
                } else if seed_a < source_a && seed_b <= source_b {
                    *seed = [seed_a, source_a - 1];
                    target_seeds.push([target_a, seed_b - source_a + target_a]);
                } else if seed_a >= source_a && seed_b > source_b {
                    *seed = [source_b + 1, seed_b];
                    target_seeds.push([seed_a - source_a + target_a, target_b]);
                } else if seed_a < source_a && seed_b > source_b {
                    target_seeds.push([target_a, target_b]);
                    *seed = [source_b + 1, seed_b];
                    initial_seeds.push([seed_a, source_a - 1]);
                } else {
                    *seed = [u64::MAX, 0];
                    target_seeds.push([seed_a - source_a + target_a, seed_b - source_a + target_a]);
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