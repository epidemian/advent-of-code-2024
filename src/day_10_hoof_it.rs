use anyhow::Context;
use itertools::{iproduct, Itertools};
use pathfinding::prelude::{bfs_reach, count_paths};

pub fn run(input: &str) -> aoc::Answer {
    let (ref map, w, h) = aoc::parse_grid(input, |ch| ch.to_digit(10).context("Invalid number"))?;
    let trailheads = iproduct!(0..w, 0..h)
        .filter(|&(x, y)| map[y][x] == 0)
        .collect_vec();

    let successors = |&(x, y): &(usize, usize)| {
        let curr_height = map[y][x];
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(move |(dx, dy)| {
                let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
                let neighbor_height = *map.get(ny)?.get(nx)?;
                (neighbor_height == curr_height + 1).then_some((nx, ny))
            })
    };

    let mut score_sum = 0;
    let mut rating_sum = 0;
    for trailhead in trailheads {
        let ends = bfs_reach(trailhead, successors)
            .filter(|&(x, y)| map[y][x] == 9)
            .collect_vec();
        let score = ends.len();
        let rating: usize = ends
            .iter()
            .map(|end| count_paths(trailhead, successors, |p| p == end))
            .sum();
        score_sum += score;
        rating_sum += rating;
    }

    aoc::answers(score_sum, rating_sum)
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
