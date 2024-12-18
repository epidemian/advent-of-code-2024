use anyhow::Context;
use itertools::Itertools;

pub fn run(input: &str) -> anyhow::Result<String> {
    let pairs: Vec<_> = input.lines().map(parse_line).try_collect()?;
    let (mut left, mut right): (Vec<_>, Vec<_>) = pairs.into_iter().unzip();
    left.sort();
    right.sort();

    let total_distance: i64 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();
    let similarity_score: i64 = left
        .iter()
        .map(|l| l * right.iter().filter(|&r| r == l).count() as i64)
        .sum();

    Ok(format!("{total_distance} {similarity_score}"))
}

fn parse_line(line: &str) -> Result<(i64, i64), anyhow::Error> {
    let (l, r) = line.split_once("   ").context("expected two numbers")?;
    Ok((l.parse()?, r.parse()?))
}
