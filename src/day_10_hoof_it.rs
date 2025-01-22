use anyhow::Context;
use itertools::iproduct;
use pathfinding::prelude::{bfs_reach, count_paths};

pub fn run(input: &str) -> aoc::Result<String> {
    let (ref map, w, h) = aoc::parse_grid(input, |ch| ch.to_digit(10).context("Invalid number"))?;
    let trailheads: Vec<_> = iproduct!(0..w, 0..h)
        .filter(|&(x, y)| map[y][x] == 0)
        .collect();

    let successors = |&(x, y): &(usize, usize)| {
        let curr_height = map[y][x];
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .map(move |(dx, dy)| (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy)))
            .filter_map(move |(nx, ny)| {
                let neighbor_height = *map.get(ny)?.get(nx)?;
                if neighbor_height == curr_height + 1 {
                    Some((nx, ny))
                } else {
                    None
                }
            })
    };

    let score_sum: usize = trailheads
        .iter()
        .map(|&trailhead| {
            bfs_reach(trailhead, successors)
                .filter(|&(x, y)| map[y][x] == 9)
                .count()
        })
        .sum();

    let trailends: Vec<_> = iproduct!(0..w, 0..h)
        .filter(|&(x, y)| map[y][x] == 9)
        .collect();
    let rating_sum: usize = trailheads
        .iter()
        .map(|&trailhead| {
            trailends
                .iter()
                .map(|&trailend| count_paths(trailhead, successors, |&p| p == trailend))
                .sum::<usize>()
        })
        .sum();

    Ok(format!("{score_sum} {rating_sum}"))
}

#[test]
fn small_sample_test() {
    let sample = "0123
1234
8765
9876
";
    assert_eq!(run(sample).unwrap(), "1 16")
}

#[test]
fn big_sample_test() {
    let sample = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";
    assert_eq!(run(sample).unwrap(), "36 81")
}
