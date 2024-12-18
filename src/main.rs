use anyhow::Context;

fn main() -> Result<(), anyhow::Error> {
    let mut left: Vec<i64> = vec![];
    let mut right: Vec<i64> = vec![];
    const INPUT: &str = include_str!("../inputs/01.txt");
    for line in INPUT.lines() {
        let (l, r) = line.split_once("   ").context("expected two numbers")?;
        left.push(l.parse()?);
        right.push(r.parse()?);
    }
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

    println!("{total_distance} {similarity_score}");
    Ok(())
}
