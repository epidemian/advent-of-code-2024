use itertools::Itertools;

pub fn run(input: &str) -> aoc::Result<String> {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(parse_line)
        .process_results(|iter| iter.unzip())?;
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

fn parse_line(line: &str) -> aoc::Result<(i64, i64)> {
    let numbers = aoc::parse_numbers(line)?;
    let [l, r] = numbers[..].try_into()?;
    Ok((l, r))
}
