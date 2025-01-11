use anyhow::Context;
use itertools::iproduct;
use std::collections::HashSet;

pub fn run(input: &str) -> aoc::Result<String> {
    let (map, w, h) = aoc::parse_char_grid(input)?;

    let start_pos = iproduct!(0..w, 0..h)
        .find(|&(x, y)| map[y][x] == '^')
        .context("guard not found")?;

    let guard_path = guard_walk(&map, start_pos).context("guard should exit the map on part 1")?;
    let guard_positions: HashSet<_> = guard_path.into_iter().collect();
    let guard_positions_count = guard_positions.len();

    let obstacles_count = guard_positions
        .into_iter()
        .filter(|&(obstacle_x, obstacle_y)| {
            let mut map = map.clone();
            map[obstacle_y][obstacle_x] = '#';
            guard_walk(&map, start_pos).is_none()
        })
        .count();

    Ok(format!("{guard_positions_count} {obstacles_count}"))
}

fn guard_walk(map: &[Vec<char>], start_pos: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    let map_size = map.len() * map[0].len();
    let (mut x, mut y) = start_pos;
    let (mut dx, mut dy) = (0, -1);
    let mut guard_path = vec![(x, y)];
    loop {
        let (new_x, new_y) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
        match map.get(new_y).and_then(|row| row.get(new_x)) {
            None => return Some(guard_path),   // Guard goes out of the map.
            Some('#') => (dx, dy) = (-dy, dx), // Rotate right.
            Some(_) => {
                (x, y) = (new_x, new_y);
                guard_path.push((x, y));
                if guard_path.len() > map_size {
                    // Guard has walked too much. She's stuck in a loop!
                    return None;
                }
            }
        }
    }
}

#[test]
fn sample_test() {
    let sample = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
    assert_eq!(run(sample).unwrap(), "41 6")
}
