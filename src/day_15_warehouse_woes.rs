use anyhow::Context;
use itertools::iproduct;
use pathfinding::prelude::bfs_reach;

pub fn run(input: &str) -> aoc::Answer {
    let (map, instructions) = input.split_once("\n\n").context("Invalid input")?;
    let (map, w, h) = aoc::parse_char_grid(map)?;
    let wide_map = widen_map(&map);
    aoc::answers(
        run_robot(map, instructions, w, h)?,
        run_robot(wide_map, instructions, w * 2, h)?,
    )
}

fn widen_map(map: &[Vec<char>]) -> Vec<Vec<char>> {
    let widen_char = |&ch| match ch {
        'O' => ['[', ']'],
        '@' => ['@', '.'],
        _ => [ch, ch],
    };
    map.iter()
        .map(|row| row.iter().flat_map(widen_char).collect())
        .collect()
}

fn run_robot(
    mut map: Vec<Vec<char>>,
    instructions: &str,
    width: usize,
    height: usize,
) -> aoc::Result<usize> {
    let (mut x, mut y) = iproduct!(0..width, 0..height)
        .find(|&(x, y)| map[y][x] == '@')
        .context("Robot not found")?;
    for ins in instructions.chars() {
        let (dx, dy) = match ins {
            '^' => (0, -1),
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (-1, 0),
            _ => continue,
        };
        let things_to_move: Vec<_> = bfs_reach((x, y), |&pos| {
            let (x2, y2) = add_signed(pos, (dx, dy));
            let ch = map.get(y2).and_then(|r| r.get(x2));
            match ch {
                None | Some('.') => vec![],
                Some('[') => vec![(x2, y2), (x2 + 1, y2)],
                Some(']') => vec![(x2 - 1, y2), (x2, y2)],
                Some(_) => vec![(x2, y2)],
            }
        })
        .map(|(x, y)| (map[y][x], x, y))
        .collect();
        let pushing_wall = things_to_move.iter().any(|&(ch, ..)| ch == '#');
        if !pushing_wall {
            for &(_, x, y) in &things_to_move {
                map[y][x] = '.';
            }
            for &(ch, x, y) in &things_to_move {
                let (new_x, new_y) = add_signed((x, y), (dx, dy));
                map[new_y][new_x] = ch;
            }
            (x, y) = add_signed((x, y), (dx, dy));
        };
    }
    let box_coords = iproduct!(0..width, 0..height)
        .filter(|&(x, y)| map[y][x] == 'O' || map[y][x] == '[')
        .map(|(x, y)| y * 100 + x);
    Ok(box_coords.sum())
}

fn add_signed((x, y): (usize, usize), (dx, dy): (isize, isize)) -> (usize, usize) {
    (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy))
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
