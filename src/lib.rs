use itertools::Itertools;
use std::{fmt::Display, result, str::FromStr};

pub type Result<T> = anyhow::Result<T>;
pub type Answer = Result<String>;

pub fn answers(p1: impl Display, p2: impl Display) -> Answer {
    Ok(format!("{p1} {p2}"))
}

pub fn answer(ans: impl Display) -> Answer {
    Ok(format!("{ans}"))
}

pub fn parse_numbers<T: FromStr>(s: &str) -> result::Result<Vec<T>, T::Err> {
    s.split(|ch: char| !ch.is_ascii_digit() && ch != '-')
        .filter(|s| !s.is_empty())
        .map(str::parse)
        .collect()
}

pub fn parse_char_grid(input: &str) -> Result<(Vec<Vec<char>>, usize, usize)> {
    parse_grid(input, Ok)
}

pub fn parse_grid<T>(
    input: &str,
    parse_char: impl Fn(char) -> Result<T>,
) -> Result<(Vec<Vec<T>>, usize, usize)> {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|l| l.chars().map(&parse_char).try_collect())
        .try_collect()?;

    let height = grid.len();
    let width = grid.first().map_or(0, |r| r.len());
    for row in grid.iter() {
        anyhow::ensure!(row.len() == width, "rows must be all the same length");
    }

    Ok((grid, width, height))
}

pub fn grid_to_str(grid: &[Vec<char>]) -> String {
    let mut s = String::new();
    for row in grid {
        s.extend(row);
        s.push('\n');
    }
    s
}
