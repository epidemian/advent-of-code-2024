use anyhow::Context;
use itertools::iproduct;
use pathfinding::prelude::{build_path, dijkstra_partial};
use rustc_hash::FxHashSet as HashSet;

pub fn run(input: &str) -> aoc::Answer {
    let (maze, w, h) = aoc::parse_char_grid(input)?;
    let (start_x, start_y) = iproduct!(0..w, 0..h)
        .find(|&(x, y)| maze[y][x] == 'S')
        .context("Start not found")?;
    let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let start = (start_x, start_y, 0);
    let successors = |&(x, y, d): &(usize, usize, usize)| {
        let (dx, dy) = dirs[d];
        let forwards = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy), d);
        let rot_r = (x, y, (d + 1) % 4);
        let rot_l = (x, y, (d + 3) % 4);
        [(forwards, 1), (rot_r, 1000), (rot_l, 1000)]
            .into_iter()
            .filter(|&((x, y, _), _)| maze[y][x] != '#')
    };
    let (parents, end) = dijkstra_partial(&start, successors, |&(x, y, _)| maze[y][x] == 'E');
    let end = end.context("Path to end not found")?;
    let best_score = parents[&end].1;

    let mut best_paths_nodes = HashSet::from_iter(build_path(&end, &parents));
    loop {
        let join_node = parents.iter().find_map(|(node, (_parent, node_cost))| {
            if best_paths_nodes.contains(node) {
                return None;
            }
            for (succ_node, succ_cost) in successors(node) {
                if best_paths_nodes.contains(&succ_node) && succ_node != start {
                    let best_path_cost = parents[&succ_node].1;
                    if node_cost + succ_cost == best_path_cost {
                        // `node` joins the best path and it's also part of a best path.
                        return Some(node);
                    }
                }
            }
            None
        });
        if let Some(node) = join_node {
            best_paths_nodes.extend(build_path(node, &parents));
        } else {
            break;
        }
    }
    let best_paths_tiles: HashSet<_> = best_paths_nodes.iter().map(|&(x, y, ..)| (x, y)).collect();

    aoc::answers(best_score, best_paths_tiles.len())
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
