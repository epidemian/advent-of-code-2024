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
    bfs(&start, successors, |&(x, y)| map[y][x] == 'E').context("Path to the end not found")
}

fn count_cheats(path: &[(usize, usize)], cheat_time: isize, min_save_time: isize) -> i32 {
    let distances: HashMap<_, _> = path.iter().copied().zip(0..).collect();
    let mut cheat_count = 0;
    for &(x, y) in path {
        for (dx, dy) in iproduct!(-cheat_time..=cheat_time, -cheat_time..=cheat_time) {
            let (end_x, end_y) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
            let d = dx.abs() + dy.abs();
            if d <= cheat_time && distances.contains_key(&(end_x, end_y)) {
                let saved_time = distances[&(end_x, end_y)] - distances[&(x, y)] - d;
                if saved_time >= min_save_time {
                    cheat_count += 1
                }
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
