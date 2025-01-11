use std::collections::HashSet;

use anyhow::Context;
use itertools::iproduct;

pub fn run(input: &str) -> aoc::Result<String> {
    let (map, w, h) = aoc::parse_char_grid(input)?;

    let guard_start_pos = iproduct!(0..w, 0..h)
        .find(|&(x, y)| map[y][x] == '^')
        .context("guard not found")?;

    // Part 1
    let (mut x, mut y) = guard_start_pos;
    let (mut dx, mut dy) = (0, -1);
    let mut visited_positions = HashSet::from([(x, y)]);

    loop {
        let (new_x, new_y) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
        match map.get(new_y).and_then(|row| row.get(new_x)) {
            None => break, // Guard goes out of the map.
            Some('#') => {
                (dx, dy) = (-dy, dx); // Rotate right.
            }
            Some(_) => {
                (x, y) = (new_x, new_y);
                visited_positions.insert((x, y));
            }
        }
    }
    let visited_positions_count = visited_positions.len();

    // Part 2
    let obstacle_positions_count = visited_positions
        .into_iter()
        .filter(|&(obstacle_x, obstacle_y)| {
            if (obstacle_x, obstacle_y) == guard_start_pos {
                return false;
            }
            let mut map = map.clone();
            map[obstacle_y][obstacle_x] = '#';
            let (mut x, mut y) = guard_start_pos;
            let (mut dx, mut dy) = (0, -1);
            let mut visited_positions = vec![(x, y)];

            loop {
                let (new_x, new_y) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
                match map.get(new_y).and_then(|row| row.get(new_x)) {
                    None => return false, // Guard goes out of the map.
                    Some('#') => {
                        (dx, dy) = (-dy, dx); // Rotate right.
                    }
                    Some(_) => {
                        (x, y) = (new_x, new_y);
                        visited_positions.push((x, y));
                        if visited_positions.len() > w * h {
                            return true;
                        }
                    }
                }
            }
        })
        .count();

    Ok(format!(
        "{visited_positions_count} {obstacle_positions_count}"
    ))
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
