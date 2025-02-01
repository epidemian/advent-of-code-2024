use anyhow::Context;
use itertools::iproduct;
use pathfinding::directed::dijkstra::dijkstra;

pub fn run(input: &str) -> aoc::Answer {
    let (maze, w, h) = aoc::parse_char_grid(input)?;
    let start_pos = iproduct!(0..w, 0..h)
        .find(|&(x, y)| maze[y][x] == 'S')
        .context("Start not found")?;
    let start_dir = (1, 0);
    let (_, best_score) = dijkstra(
        &(start_pos, start_dir),
        |&((x, y), (dx, dy))| {
            let forwards = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
            [
                ((forwards, (dx, dy)), 1),   // Advance
                (((x, y), (-dy, dx)), 1000), // Rotate right
                (((x, y), (dy, -dx)), 1000), // Rotate left
            ]
            .into_iter()
            .filter(|&(((x, y), _), _)| maze[y][x] != '#')
        },
        |&((x, y), _)| maze[y][x] == 'E',
    )
    .context("Path to end not found")?;

    aoc::answer(best_score)
}

#[test]
fn sample_test() {
    let sample = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";
    assert_eq!(run(sample).unwrap(), "7036")
}
