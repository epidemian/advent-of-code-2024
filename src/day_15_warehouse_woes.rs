use anyhow::Context;
use itertools::iproduct;

pub fn run(input: &str) -> aoc::Answer {
    let (map, instructions) = input.split_once("\n\n").context("Invalid input")?;
    let (mut map, w, h) = aoc::parse_char_grid(map)?;
    let (mut x, mut y) = iproduct!(0..w, 0..h)
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
        let first_non_box = (1..).find_map(|i| {
            let tile = *map
                .get(y.wrapping_add_signed(i * dy))?
                .get(x.wrapping_add_signed(i * dx))?;
            (tile != 'O').then_some((tile, i))
        });
        if let Some(('.', n)) = first_non_box {
            map[y][x] = '.';
            (x, y) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
            map[y][x] = '@';
            for i in 1..n {
                map[y.wrapping_add_signed(i * dy)][x.wrapping_add_signed(i * dx)] = 'O';
            }
        }
        // println!("after {ins}:\n{}\n", aoc::grid_to_str(&map));
    }
    let gps_sum: usize = iproduct!(0..w, 0..h)
        .filter(|&(x, y)| map[y][x] == 'O')
        .map(|(x, y)| y * 100 + x)
        .sum();

    aoc::answer(gps_sum)
}

#[test]
fn sample_test() {
    let sample = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";
    assert_eq!(run(sample).unwrap(), "2028")
}
