use anyhow::Context;
use itertools::Itertools;
use regex::Regex;

pub fn run(input: &str) -> aoc::Answer {
    let (towels, designs) = input.split_once("\n\n").context("Invalid input")?;
    let re_pattern = format!("^({})*$", towels.split(", ").join("|"));
    let re = Regex::new(&re_pattern).unwrap();
    let possible_designs_count = designs.lines().filter(|d| re.is_match(d)).count();
    aoc::answer(possible_designs_count)
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
    assert_eq!(run(sample).unwrap(), "6")
}
