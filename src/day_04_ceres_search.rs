use itertools::iproduct;
use std::array;

pub fn run(input: &str) -> aoc::Answer {
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
                        let ch_x = x.wrapping_add_signed(dx * i as isize);
                        let ch_y = y.wrapping_add_signed(dy * i as isize);
                        grid.get(ch_y)?.get(ch_x).copied()
                    });
                    strip == [Some('X'), Some('M'), Some('A'), Some('S')]
                })
                .count()
        })
        .sum();

    let x_mas_count = iproduct!(0..w.saturating_sub(2), 0..h.saturating_sub(2))
        .filter(|(x, y)| {
            let diag_1 = array::from_fn(|i| grid[y + i][x + i]);
            let diag_2 = array::from_fn(|i| grid[y + i][x + 2 - i]);
            let is_mas = |diag| diag == ['M', 'A', 'S'] || diag == ['S', 'A', 'M'];
            is_mas(diag_1) && is_mas(diag_2)
        })
        .count();

    aoc::answer(xmas_count, x_mas_count)
}

#[test]
fn small_inputs_test() {
    assert_eq!(run("").unwrap(), "0 0");
    assert_eq!(run("SMA").unwrap(), "0 0");
    assert_eq!(run("S\nM\nA").unwrap(), "0 0");
}

#[test]
fn sample_test() {
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
