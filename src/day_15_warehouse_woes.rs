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
        if can_move((x, y), (dx, dy), &map) {
            push_boxes((x, y), (dx, dy), &mut map);
            (x, y) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
        };
    }
    let gps_sum: usize = iproduct!(0..width, 0..height)
        .filter(|&(x, y)| map[y][x] == 'O' || map[y][x] == '[')
        .map(|(x, y)| y * 100 + x)
        .sum();
    Ok(gps_sum)
}

fn can_move((x, y): (usize, usize), dir: (isize, isize), map: &[Vec<char>]) -> bool {
    let ch = map[y][x];
    match ch {
        '@' | 'O' => can_move(add_signed((x, y), dir), dir, map),
        '#' => false,
        '[' | ']' => {
            if dir.0 != 0 {
                can_move(add_signed((x, y), dir), dir, map)
            } else {
                let lx = if ch == '[' { x } else { x - 1 };
                let new_l = add_signed((lx, y), dir);
                let new_r = add_signed((lx + 1, y), dir);
                can_move(new_l, dir, map) && can_move(new_r, dir, map)
            }
        }
        _ => true,
    }
}

fn push_boxes((x, y): (usize, usize), dir: (isize, isize), map: &mut [Vec<char>]) {
    let ch = map[y][x];
    match ch {
        '@' | 'O' => {
            let (new_x, new_y) = add_signed((x, y), dir);
            push_boxes((new_x, new_y), dir, map);
            map[new_y][new_x] = ch;
            map[y][x] = '.';
        }
        '[' | ']' => {
            let lx = if ch == '[' { x } else { x - 1 };
            let (new_lx, new_ly) = add_signed((lx, y), dir);
            let (new_rx, new_ry) = add_signed((lx + 1, y), dir);
            if dir.0 != 0 {
                let new_x = if dir.0 == 1 { new_rx } else { new_lx };
                push_boxes((new_x, y), dir, map);
            } else {
                push_boxes((new_lx, new_ly), dir, map);
                push_boxes((new_rx, new_ry), dir, map);
            }
            map[y][lx] = '.';
            map[y][lx + 1] = '.';
            map[new_ly][new_lx] = '[';
            map[new_ry][new_rx] = ']';
        }
        '#' => panic!("Tried to push boxes into wall!"),
        _ => {}
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
