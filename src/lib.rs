use std::{result, str::FromStr};

pub type Result<T> = anyhow::Result<T>;

pub fn parse_numbers<T: FromStr>(s: &str) -> result::Result<Vec<T>, T::Err> {
    s.split(|ch: char| !ch.is_ascii_digit() && ch != '-')
        .filter(|s| !s.is_empty())
        .map(str::parse)
        .collect()
}
