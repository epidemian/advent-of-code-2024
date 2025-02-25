use anyhow::Context;
use itertools::iproduct;
use pathfinding::prelude::bfs;
use rustc_hash::FxHashMap as HashMap;

pub fn run(input: &str) -> aoc::Answer {
    let (ref map, w, h) = aoc::parse_char_grid(input)?;
    let start = iproduct!(0..w, 0..h)
        .find(|&(x, y)| map[y][x] == 'S')
        .context("Start position not found")?;
    let path = bfs(
        &start,
        |&(x, y)| {
            [(1, 0), (0, 1), (-1, 0), (0, -1)]
                .into_iter()
                .map(move |(dx, dy)| (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy)))
                .filter(move |&(nx, ny)| nx < w && ny < h && map[ny][nx] != '#')
        },
        |&(x, y)| map[y][x] == 'E',
    )
    .context("Path to the end not found")?;
    let distances: HashMap<_, _> = path.iter().copied().zip(0..).collect();
    let mut cheat_count = 0;
    for &(x, y) in &path {
        for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let (end_x, end_y) = (x.wrapping_add_signed(dx * 2), y.wrapping_add_signed(dy * 2));
            if end_x < w && end_y < h && map[end_y][end_x] != '#' {
                let diff = distances[&(end_x, end_y)] - distances[&(x, y)] - 2;
                if diff >= 100 {
                    cheat_count += 1
                }
            }
        }
    }

    aoc::answer(cheat_count)
}

#[test]
fn sample_test() {
    let sample = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";
    assert_eq!(run(sample).unwrap(), "0")
}
