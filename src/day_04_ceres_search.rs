use itertools::iproduct;
use std::array;

pub fn run(input: &str) -> aoc::Result<String> {
    let (grid, w, h) = aoc::parse_char_grid(input)?;

    let xmas_count: usize = iproduct!(0..w, 0..h)
        .map(|(x, y)| {
            #[rustfmt::skip]
            let dirs = [
                // Right, left, down, up.
                (1, 0), (-1, 0), (0, 1), (0, -1),
                // Diagonals.
                (1, 1), (-1, -1), (1, -1), (-1, 1)
            ];
            dirs.into_iter()
                .filter(|(dx, dy)| {
                    let strip = array::from_fn(|i| {
                        let sx = x.wrapping_add_signed(dx * i as isize);
                        let sy = y.wrapping_add_signed(dy * i as isize);
                        grid.get(sy)?.get(sx).copied()
                    });
                    strip == [Some('X'), Some('M'), Some('A'), Some('S')]
                })
                .count()
        })
        .sum();

    let x_mas_count = iproduct!(0..w.saturating_sub(2), 0..h.saturating_sub(2))
        .filter(|(x, y)| {
            let [center, top_l, top_r, bottom_r, bottom_l] =
                [(1, 1), (0, 0), (0, 2), (2, 2), (2, 0)].map(|(dx, dy)| grid[y + dy][x + dx]);
            center == 'A'
                && matches!((top_l, bottom_r), ('M', 'S') | ('S', 'M'))
                && matches!((top_r, bottom_l), ('M', 'S') | ('S', 'M'))
        })
        .count();

    Ok(format!("{xmas_count} {x_mas_count}"))
}

#[test]
fn empty() {
    assert_eq!(run("").unwrap(), "0 0")
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