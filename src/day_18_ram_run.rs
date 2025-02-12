use anyhow::Context;
use itertools::Itertools;
use pathfinding::prelude::dijkstra;
use rustc_hash::FxHashSet as HashSet;

pub fn run(input: &str) -> aoc::Answer {
    let falling_bytes = parse_byte_coordinates(input)?;
    let step_count = find_path(&falling_bytes[0..1024], 70).context("Path to exit not found")?;
    let (bx, by) = find_first_blocking_byte(&falling_bytes, 70)
        .context("No byte that blocks path to exit found")?;
    aoc::answers(step_count, format!("{bx},{by}"))
}

fn parse_byte_coordinates(input: &str) -> aoc::Result<Vec<(u32, u32)>> {
    let parse_coordinates = |l| {
        aoc::parse_numbers(l)?
            .into_iter()
            .collect_tuple()
            .context("Expected two numbers")
    };
    input.lines().map(parse_coordinates).try_collect()
}

fn find_path(fallen_bytes: &[(u32, u32)], memory_size: u32) -> Option<u32> {
    let corrupted_positions: HashSet<_> = fallen_bytes.iter().copied().collect();
    let (_path, cost) = dijkstra(
        &(0u32, 0u32),
        |&(x, y)| {
            [(1, 0), (0, 1), (-1, 0), (0, -1)]
                .into_iter()
                .map(move |(dx, dy)| (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy)))
                .filter(|&(x, y)| {
                    x <= memory_size && y <= memory_size && !corrupted_positions.contains(&(x, y))
                })
                .map(|pos| (pos, 1))
        },
        |&pos| pos == (memory_size, memory_size),
    )?;
    Some(cost)
}

fn find_first_blocking_byte(bytes: &[(u32, u32)], memory_size: u32) -> Option<(u32, u32)> {
    let indices = (0..bytes.len()).collect_vec();
    let idx = indices.partition_point(|&i| find_path(&bytes[0..=i], memory_size).is_some());
    bytes.get(idx).copied()
}

#[test]
fn sample_test() {
    let sample = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";
    let falling_bytes = parse_byte_coordinates(sample).unwrap();
    assert_eq!(find_path(&falling_bytes[0..12], 6), Some(22));
    assert_eq!(find_first_blocking_byte(&falling_bytes, 6), Some((6, 1)));
}
