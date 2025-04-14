use itertools::{Itertools, zip_eq};
use rustc_hash::FxHashMap as HashMap;

pub fn run(input: &str) -> aoc::Answer {
    let secrets = aoc::parse_numbers(input)?;
    let secret_sequences: Vec<Vec<u64>> = secrets
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
    let price_sequences: Vec<Vec<u8>> = secret_sequences
        .iter()
        .map(|secrets| secrets.iter().map(|s| (s % 10) as u8).collect())
        .collect();
    let diff_sequences: Vec<Vec<i8>> = price_sequences
        .iter()
        .map(|prices| {
            prices
                .iter()
                .tuple_windows()
                .map(|(&a, &b)| b as i8 - a as i8)
                .collect()
        })
        .collect();
    let mut total_bananas: HashMap<[i8; 4], u64> = HashMap::default();
    for (diff_seq, price_seq) in zip_eq(diff_sequences, price_sequences) {
        let mut monkey_bananas: HashMap<[i8; 4], u8> = HashMap::default();
        for (diffs, &price) in zip_eq(diff_seq.windows(4), &price_seq[4..]) {
            let diffs: [i8; 4] = diffs.try_into().unwrap();
            monkey_bananas.entry(diffs).or_insert(price);
        }
        for (diff, price) in monkey_bananas {
            *total_bananas.entry(diff).or_insert(0) += price as u64;
        }
    }
    let max_bananas = total_bananas.values().max().unwrap_or(&0);
    let sum: u64 = secret_sequences.iter().flat_map(|s| s.last()).sum();
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
