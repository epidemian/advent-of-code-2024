use itertools::{chain, iproduct, Itertools};

pub fn run(input: &str) -> aoc::Result<String> {
    let (map, w, h) = aoc::parse_char_grid(input)?;
    let is_inside_map = |(x, y)| 0 <= x && x < w as i64 && 0 <= y && y < h as i64;

    let antennas_by_freq = iproduct!(0..w, 0..h)
        .map(|(x, y)| (map[y][x], (x as i64, y as i64)))
        .filter(|&(ch, _)| ch != '.')
        .into_group_map();
    let antenna_groups = antennas_by_freq.into_values().collect_vec();

    let antinode_count_p1 = count_antinodes(&antenna_groups, |(x1, y1), (x2, y2)| {
        let (dx, dy) = (x1 - x2, y1 - y2);
        [(x1 + dx, y1 + dy), (x2 - dx, y2 - dy)]
            .into_iter()
            .filter(|&p| is_inside_map(p))
    });

    let antinode_count_p2 = count_antinodes(&antenna_groups, |(x1, y1), (x2, y2)| {
        let (dx, dy) = (x1 - x2, y1 - y2);
        chain(
            (0..)
                .map(move |i| (x1 + dx * i, y1 + dy * i))
                .take_while(|&p| is_inside_map(p)),
            (1..)
                .map(move |i| (x1 - dx * i, y1 - dy * i))
                .take_while(|&p| is_inside_map(p)),
        )
    });

    Ok(format!("{antinode_count_p1} {antinode_count_p2}"))
}

fn count_antinodes<I>(
    antenna_groups: &[Vec<(i64, i64)>],
    get_antinodes: impl Fn((i64, i64), (i64, i64)) -> I,
) -> usize
where
    I: Iterator<Item = (i64, i64)>,
{
    antenna_groups
        .iter()
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
