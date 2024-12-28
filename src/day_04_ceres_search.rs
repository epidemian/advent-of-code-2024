use itertools::iproduct;

pub fn run(input: &str) -> aoc::Result<String> {
    let (grid, w, h) = aoc::parse_char_grid(input)?;
    let xmas_count: u64 = iproduct!(0..w, 0..h)
        .map(|(x, y)| {
            let dirs = [
                [(0, 0), (1, 0), (2, 0), (3, 0)],
                [(0, 0), (0, 1), (0, 2), (0, 3)],
                [(0, 0), (1, 1), (2, 2), (3, 3)],
                [(3, 0), (2, 1), (1, 2), (0, 3)],
            ];
            let mut count = 0;
            for d in dirs {
                let chars = d.map(|(dx, dy)| grid.get(y + dy)?.get(x + dx).copied());
                if chars == [Some('X'), Some('M'), Some('A'), Some('S')] {
                    count += 1;
                }
                if chars == [Some('S'), Some('A'), Some('M'), Some('X')] {
                    count += 1;
                }
            }
            count
        })
        .sum();

    let x_mas_count = iproduct!(0..w - 2, 0..h - 2)
        .filter(|(x, y)| {
            let cross = [(0, 0), (1, 1), (2, 2), (0, 2), (2, 0)];
            let cross_chars = cross.map(|(dx, dy)| grid[y + dy][x + dx]);
            matches!(
                cross_chars,
                ['M', 'A', 'S', 'M', 'S']
                    | ['M', 'A', 'S', 'S', 'M']
                    | ['S', 'A', 'M', 'M', 'S']
                    | ['S', 'A', 'M', 'S', 'M']
            )
        })
        .count();

    Ok(format!("{xmas_count} {x_mas_count}"))
}

#[test]
fn sample() {
    let sample = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
    assert_eq!(run(sample).unwrap(), "18 9")
}
