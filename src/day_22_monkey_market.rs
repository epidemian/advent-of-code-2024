use itertools::{Itertools, zip_eq};
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

pub fn run(input: &str) -> aoc::Answer {
    let initial_secrets = aoc::parse_numbers(input)?;
    let secrets: Vec<Vec<u64>> = initial_secrets
        .into_iter()
        .map(|secret| {
            (0..=2000)
                .scan(secret, |s, _| {
                    let res = Some(*s);
                    *s = rand(*s);
                    res
                })
                .collect()
        })
        .collect();
    let prices: Vec<Vec<u8>> = secrets
        .iter()
        .map(|monkey_secrets| monkey_secrets.iter().map(|s| (s % 10) as u8).collect())
        .collect();
    let diffs: Vec<Vec<u8>> = prices
        .iter()
        .map(|monkey_prices| {
            monkey_prices
                .iter()
                .tuple_windows()
                .map(|(&a, &b)| 10 + b - a) // Avoid negative diff
                .collect()
        })
        .collect();
    let mut total_bananas = HashMap::default();
    for (monkey_diffs, monkey_prices) in zip_eq(diffs, prices) {
        let mut diffs_seen = HashSet::default();
        for (diffs, &price) in zip_eq(monkey_diffs.windows(4), &monkey_prices[4..]) {
            let diffs: [u8; 4] = diffs.try_into().unwrap();
            if diffs_seen.insert(diffs) {
                *total_bananas.entry(diffs).or_insert(0) += price as u32;
            }
        }
    }
    let sum: u64 = secrets.iter().flat_map(|s| s.last()).sum();
    let max_bananas = total_bananas.values().max().unwrap_or(&0);
    aoc::answers(sum, max_bananas)
}

fn rand(mut s: u64) -> u64 {
    let mask = 0xFFFFFF;
    s = ((s << 6) ^ s) & mask;
    s = ((s >> 5) ^ s) & mask;
    s = ((s << 11) ^ s) & mask;
    s
}

#[test]
fn part_1_sample_test() {
    let sample = "1
10
100
2024
";
    assert_eq!(run(sample).unwrap(), "37327623 24")
}

#[test]
fn part_2_sample_test() {
    let sample = "1
2
3
2024
";
    assert_eq!(run(sample).unwrap(), "37990510 23")
}
