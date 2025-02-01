use anyhow::Context;
use itertools::iproduct;
use pathfinding::prelude::bfs_reach;
use rustc_hash::FxHashMap as HashMap;

pub fn run(input: &str) -> aoc::Answer {
    let (map, instructions) = input.split_once("\n\n").context("Invalid input")?;
    let (map, w, h) = aoc::parse_char_grid(map)?;
    let map = iproduct!(0..w, 0..h)
        .map(|(x, y)| ((x as isize, y as isize), map[y][x]))
        .filter(|&(_, ch)| ch != '.')
        .collect();
    let wide_map = widen_map(&map);
    aoc::answers(
        run_robot(map, instructions)?,
        run_robot(wide_map, instructions)?,
    )
}

type Map = HashMap<(isize, isize), char>;

fn widen_map(map: &Map) -> Map {
    let widen_thing = |(&(x, y), &ch)| {
        let new_things = match ch {
            'O' => vec!['[', ']'],
            '@' => vec!['@'],
            _ => vec![ch, ch],
        };
        (0..)
            .zip(new_things)
            .map(move |(i, thing)| ((x * 2 + i, y), thing))
    };
    map.iter().flat_map(widen_thing).collect()
}

fn run_robot(mut map: Map, instructions: &str) -> aoc::Result<isize> {
    let (&(mut x, mut y), _) = map
        .iter()
        .find(|&(_, &ch)| ch == '@')
        .context("Robot not found")?;
    for ins in instructions.chars() {
        let (dx, dy) = match ins {
            '^' => (0, -1),
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (-1, 0),
            _ => continue,
        };
        let things_to_move: Vec<_> = bfs_reach((x, y), |&(x, y)| {
            let (x2, y2) = (x + dx, y + dy);
            match map.get(&(x2, y2)) {
                None => vec![],
                Some('[') => vec![(x2, y2), (x2 + 1, y2)],
                Some(']') => vec![(x2 - 1, y2), (x2, y2)],
                Some(_) => vec![(x2, y2)],
            }
        })
        .map(|pos| (map[&pos], pos))
        .collect();
        let pushing_wall = things_to_move.iter().any(|&(ch, ..)| ch == '#');
        if !pushing_wall {
            for &(_, pos) in &things_to_move {
                map.remove(&pos);
            }
            for &(ch, (x, y)) in &things_to_move {
                let (new_x, new_y) = (x + dx, y + dy);
                map.insert((new_x, new_y), ch);
            }
            (x, y) = (x + dx, y + dy);
        };
    }
    let boxes = map.iter().filter(|&(_, &ch)| ch == 'O' || ch == '[');
    Ok(boxes.map(|((x, y), _)| y * 100 + x).sum())
}

#[test]
fn sample_test() {
    let sample = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";
    assert_eq!(run(sample).unwrap(), "10092 9021")
}

#[test]
fn bad_map_test() {
    let input = "#####
#...#
#.@O.
#...#
#####

>>>>>";
    assert!(run(input).is_ok())
}
