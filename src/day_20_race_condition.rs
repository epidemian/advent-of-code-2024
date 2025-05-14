use anyhow::Context;
use itertools::iproduct;
use pathfinding::prelude::bfs;
use rustc_hash::FxHashMap as HashMap;

pub fn run(input: &str) -> aoc::Answer {
    let path = find_path(input)?;
    aoc::answers(count_cheats(&path, 2, 100), count_cheats(&path, 20, 100))
}

fn find_path(input: &str) -> aoc::Result<Vec<(usize, usize)>> {
    let (ref map, w, h) = aoc::parse_char_grid(input)?;
    let start = iproduct!(0..w, 0..h)
        .find(|&(x, y)| map[y][x] == 'S')
        .context("Start position not found")?;
    let successors = |&(x, y): &(usize, usize)| {
        [(1, 0), (0, 1), (-1, 0), (0, -1)]
            .into_iter()
            .map(move |(dx, dy)| (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy)))
            .filter(move |&(nx, ny)| nx < w && ny < h && map[ny][nx] != '#')
    };
    bfs(&start, successors, |&(x, y)| map[y][x] == 'E').context("path to the end not found")
}

fn count_cheats(path: &[(usize, usize)], max_cheat: isize, min_save_time: isize) -> usize {
    let times: HashMap<_, _> = path.iter().copied().zip(0..).collect();
    let mut cheat_count = 0;
    for (&(x, y), &t) in &times {
        for (dx, dy) in iproduct!(-max_cheat..=max_cheat, -max_cheat..=max_cheat) {
            let cheat_dist = dx.abs() + dy.abs();
            if cheat_dist > max_cheat {
                continue;
            }
            let cheat_end = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
            let Some(&cheat_end_t) = times.get(&cheat_end) else {
                continue;
            };
            let saved_time = cheat_end_t - t - cheat_dist;
            if saved_time >= min_save_time {
                cheat_count += 1
            }
        }
    }
    cheat_count
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
    let path = find_path(sample).unwrap();
    assert_eq!(count_cheats(&path, 2, 1), 44);
    assert_eq!(count_cheats(&path, 20, 50), 285);
}
