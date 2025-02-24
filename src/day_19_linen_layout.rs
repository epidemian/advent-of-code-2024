use anyhow::Context;
use itertools::Itertools;
use rustc_hash::FxHashMap as HashMap;

pub fn run(input: &str) -> aoc::Answer {
    let (towels, designs) = input.split_once("\n\n").context("Invalid input")?;
    let towels = towels.split(", ").collect_vec();
    let designs = designs.lines().collect_vec();
    let mut cache = HashMap::default();
    let arrangement_counts = designs
        .into_iter()
        .map(|design| count_arrangements(design, &towels, &mut cache))
        .collect_vec();
    aoc::answers(
        arrangement_counts.iter().filter(|&&c| c > 0).count(),
        arrangement_counts.iter().sum::<u64>(),
    )
}

fn count_arrangements<'a>(
    design: &'a str,
    towels: &[&str],
    cache: &mut HashMap<&'a str, u64>,
) -> u64 {
    if design.is_empty() {
        return 1;
    }
    if let Some(&n) = cache.get(design) {
        return n;
    }
    let count = towels
        .iter()
        .map(|t| {
            let Some(remaining_design) = design.strip_prefix(t) else {
                return 0;
            };
            count_arrangements(remaining_design, towels, cache)
        })
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
