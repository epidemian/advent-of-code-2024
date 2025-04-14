use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

pub fn run(input: &str) -> aoc::Answer {
    let secrets = aoc::parse_numbers(input)?;
    let mut secrets_sum = 0_u64;
    let mut total_bananas = HashMap::default();
    for mut secret in secrets {
        // Store 4 price diffs as 4 bytes in a u32. Using a [u8; 4] is slower for some reason.
        let mut diffs = 0_u32;
        let mut diffs_seen = HashSet::default();
        for i in 0..2000 {
            let prev_secret = secret;
            secret = rand(secret);
            let diff = 10 + secret % 10 - prev_secret % 10;
            diffs = diffs << 8 | diff;
            if i >= 3 && diffs_seen.insert(diffs) {
                *total_bananas.entry(diffs).or_insert(0) += secret % 10;
            }
        }
        secrets_sum += secret as u64;
    }
    let max_bananas = total_bananas.values().max().unwrap_or(&0);
    aoc::answers(secrets_sum, max_bananas)
}

fn rand(mut s: u32) -> u32 {
    let mask = 0xFFFFFF;
    s = ((s << 6) ^ s) & mask;
    s = ((s >> 5) ^ s) & mask;
    s = ((s << 11) ^ s) & mask;
    s
}

#[test]
fn part_1_sample_test() {
    let sample = "1\n10\n100\n2024";
    assert_eq!(run(sample).unwrap(), "37327623 24")
}

#[test]
fn part_2_sample_test() {
    let sample = "1\n2\n3\n2024";
    assert_eq!(run(sample).unwrap(), "37990510 23")
}
