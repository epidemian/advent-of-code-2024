use itertools::{chain, iproduct, Itertools};

pub fn run(input: &str) -> aoc::Result<String> {
    let (map, w, h) = aoc::parse_char_grid(input)?;
    let is_inside_map = |&(x, y): &_| 0 <= x && x < w as i64 && 0 <= y && y < h as i64;

    let antenna_groups = iproduct!(0..w, 0..h)
        .map(|(x, y)| (map[y][x], (x as i64, y as i64)))
        .filter(|&(ch, _)| ch != '.')
        .into_group_map();

    let antinode_count_p1 = antenna_groups
        .values()
        .flat_map(|antennas| {
            antennas
                .iter()
                .tuple_combinations()
                .flat_map(|((x1, y1), (x2, y2))| {
                    let (dx, dy) = (x1 - x2, y1 - y2);
                    [(x1 + dx, y1 + dy), (x2 - dx, y2 - dy)]
                })
        })
        .filter(is_inside_map)
        .unique()
        .count();

    let antinode_count_p2 = antenna_groups
        .values()
        .flat_map(|antennas| {
            antennas
                .iter()
                .tuple_combinations()
                .flat_map(|((x1, y1), (x2, y2))| {
                    let (dx, dy) = (x1 - x2, y1 - y2);
                    chain(
                        (0..)
                            .map(move |i| (x1 + dx * i, y1 + dy * i))
                            .take_while(is_inside_map),
                        (1..)
                            .map(move |i| (x1 - dx * i, y1 - dy * i))
                            .take_while(is_inside_map),
                    )
                })
        })
        .unique()
        .count();

    Ok(format!("{antinode_count_p1} {antinode_count_p2}"))
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
