use itertools::{chain, iproduct, Itertools};
use std::collections::HashMap;

pub fn run(input: &str) -> aoc::Result<String> {
    let (map, w, h) = aoc::parse_char_grid(input)?;
    let is_inside_map = |&(x, y): &_| 0 <= x && x < w as i64 && 0 <= y && y < h as i64;

    let antenna_groups = iproduct!(0..w, 0..h)
        .map(|(x, y)| (map[y][x], (x as i64, y as i64)))
        .filter(|&(ch, _)| ch != '.')
        .into_group_map();

    let antinode_count_p1 = count_unique_antinodes(&antenna_groups, |(x1, y1), (x2, y2)| {
        let (dx, dy) = (x1 - x2, y1 - y2);
        [(x1 + dx, y1 + dy), (x2 - dx, y2 - dy)]
            .into_iter()
            .filter(is_inside_map)
    });

    let antinode_count_p2 = count_unique_antinodes(&antenna_groups, |(x1, y1), (x2, y2)| {
        let (dx, dy) = (x1 - x2, y1 - y2);
        chain(
            (0..)
                .map(move |i| (x1 + dx * i, y1 + dy * i))
                .take_while(is_inside_map),
            (1..)
                .map(move |i| (x1 - dx * i, y1 - dy * i))
                .take_while(is_inside_map),
        )
    });

    Ok(format!("{antinode_count_p1} {antinode_count_p2}"))
}

type Point = (i64, i64);

fn count_unique_antinodes<I>(
    antenna_groups: &HashMap<char, Vec<(i64, i64)>>,
    get_antinodes: impl Fn(Point, Point) -> I,
) -> usize
where
    I: Iterator<Item = Point>,
{
    antenna_groups
        .values()
        .flat_map(|antennas| {
            antennas
                .iter()
                .tuple_combinations()
                .flat_map(|(&a, &b)| get_antinodes(a, b))
        })
        .unique()
        .count()
}

#[test]
fn sample_test() {
    let sample = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
    assert_eq!(run(sample).unwrap(), "14 34")
}
