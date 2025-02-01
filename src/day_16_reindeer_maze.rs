use anyhow::Context;
use itertools::{iproduct, Itertools};
use pathfinding::{directed::dijkstra::dijkstra, prelude::dijkstra_reach};

pub fn run(input: &str) -> aoc::Answer {
    let (maze, w, h) = aoc::parse_char_grid(input)?;
    let start_pos = iproduct!(0..w, 0..h)
        .find(|&(x, y)| maze[y][x] == 'S')
        .context("Start not found")?;
    let start = (start_pos, (1, 0));
    let successors = |&((x, y), (dx, dy)): &((usize, usize), (isize, isize))| {
        let forwards = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
        [
            ((forwards, (dx, dy)), 1),   // Advance
            (((x, y), (-dy, dx)), 1000), // Rotate right
            (((x, y), (dy, -dx)), 1000), // Rotate left
        ]
        .into_iter()
        .filter(|&(((x, y), _), _)| maze[y][x] != '#')
    };
    let (_, best_score) = dijkstra(&start, successors, |&((x, y), _)| maze[y][x] == 'E')
        .context("Path to end not found")?;

    let possible_tiles: Vec<_> = dijkstra_reach(&start, |node, _cost| successors(node))
        .take_while(|node| node.total_cost <= best_score)
        .collect();

    let mut count = 0;
    let total = possible_tiles.len();
    let best_path_tile_count = possible_tiles
        .into_iter()
        .filter(|node| {
            let (_, cost) =
                dijkstra(&node.node, successors, |&((x, y), _)| maze[y][x] == 'E').unwrap();
            count += 1;
            if count % 100 == 0 {
                eprintln!("{count}/{total} processed");
            }
            cost + node.total_cost == best_score
        })
        .map(|node| node.node.0)
        .unique()
        .count();
    aoc::answers(best_score, best_path_tile_count)
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
    assert_eq!(run(sample).unwrap(), "7036 45")
}
