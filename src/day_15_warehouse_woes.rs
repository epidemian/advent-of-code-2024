use anyhow::Context;
use itertools::iproduct;

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
        let mut moves = vec![];
        if collect_moves((x, y), (dx, dy), &map, &mut moves) {
            for &(_, x, y) in &moves {
                map[y][x] = '.';
            }
            for &(ch, x, y) in &moves {
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

fn collect_moves(
    (x, y): (usize, usize),
    dir: (isize, isize),
    map: &[Vec<char>],
    moves: &mut Vec<(char, usize, usize)>,
) -> bool {
    let ch = map[y][x];
    match ch {
        '@' | 'O' => {
            moves.push((ch, x, y));
            collect_moves(add_signed((x, y), dir), dir, map, moves)
        }
        '#' => false,
        '[' | ']' => {
            if dir.0 != 0 {
                moves.push((ch, x, y));
                collect_moves(add_signed((x, y), dir), dir, map, moves)
            } else {
                let lx = if ch == '[' { x } else { x - 1 };
                moves.push(('[', lx, y));
                moves.push((']', lx + 1, y));
                let new_l = add_signed((lx, y), dir);
                let new_r = add_signed((lx + 1, y), dir);
                collect_moves(new_l, dir, map, moves) && collect_moves(new_r, dir, map, moves)
            }
        }
        _ => true,
    }
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
