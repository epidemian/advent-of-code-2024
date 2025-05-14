use anyhow::Context;
use itertools::Itertools;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

pub fn run(input: &str) -> aoc::Answer {
    let (towels, designs) = input.split_once("\n\n").context("invalid input")?;
    let towels: HashSet<_> = towels.split(", ").collect();
    let designs = designs.lines().collect_vec();
    let max_towel_len = towels.iter().map(|t| t.len()).max().unwrap_or(0);
    let arrangement_counts = designs
        .into_iter()
        .map(|design| count_arrangements(design, &towels, max_towel_len, &mut HashMap::default()))
        .collect_vec();
    aoc::answers(
        arrangement_counts.iter().filter(|&&c| c > 0).count(),
        arrangement_counts.iter().sum::<u64>(),
    )
}

fn count_arrangements<'a>(
    design: &'a str,
    towels: &HashSet<&str>,
    max_towel_len: usize,
    cache: &mut HashMap<&'a str, u64>,
) -> u64 {
    if design.is_empty() {
        return 1;
    }
    if let Some(&n) = cache.get(design) {
        return n;
    }
    let count = (1..=max_towel_len.min(design.len()))
        .filter(|&i| towels.contains(&design[0..i]))
        .map(|i| count_arrangements(&design[i..], towels, max_towel_len, cache))
        .sum();
    cache.insert(design, count);
    count
}

#[test]
fn sample_test() {
    let sample = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";
    assert_eq!(run(sample).unwrap(), "6 16")
}
