use itertools::{iproduct, Itertools};
use std::collections::HashMap;

pub fn run(input: &str) -> aoc::Result<String> {
    let (map, w, h) = aoc::parse_char_grid(input)?;

    let mut antennas: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    for (x, y) in iproduct!(0..w, 0..h) {
        let ch = map[y][x];
        if ch != '.' {
            antennas.entry(ch).or_default().push((x as i64, y as i64));
        }
    }

    let antinode_count_p1 = antennas
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
        .filter(|&(x, y)| 0 <= x && x < w as i64 && 0 <= y && y < h as i64)
        .unique()
        .count();

    let antinode_count_p2 = antennas
        .values()
        .flat_map(|antennas| {
            antennas
                .iter()
                .tuple_combinations()
                .flat_map(|((x1, y1), (x2, y2))| {
                    let (dx, dy) = (x1 - x2, y1 - y2);
                    let antinode_line = (0..)
                        .map(move |i| (x1 + dx * i, y1 + dy * i))
                        .take_while(|&(x, y)| 0 <= x && x < w as i64 && 0 <= y && y < h as i64);
                    let antinode_opposite_line = (1..)
                        .map(move |i| (x1 - dx * i, y1 - dy * i))
                        .take_while(|&(x, y)| 0 <= x && x < w as i64 && 0 <= y && y < h as i64);
                    antinode_line.chain(antinode_opposite_line)
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
