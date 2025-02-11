use anyhow::Context;
use itertools::iproduct;
use pathfinding::prelude::{build_path, dijkstra_partial};
use rustc_hash::FxHashSet as HashSet;

pub fn run(input: &str) -> aoc::Answer {
    let (maze, w, h) = aoc::parse_char_grid(input)?;
    let start_pos = iproduct!(0..w, 0..h)
        .find(|&(x, y)| maze[y][x] == 'S')
        .context("Start not found")?;
    let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let start = (start_pos, 0);
    let successors = |&((x, y), d): &((usize, usize), usize)| {
        let add = |(dx, dy)| (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));
        let r = (d + 1) % 4;
        let l = (d + 3) % 4;
        let go_forward = (add(dirs[d]), d);
        let go_right = (add(dirs[r]), r);
        let go_left = (add(dirs[l]), l);
        [(go_forward, 1), (go_right, 1001), (go_left, 1001)]
            .into_iter()
            .filter(|&(((x, y), _), _)| x < w && y < h && maze[y][x] != '#')
    };
    let (parents, end) = dijkstra_partial(&start, successors, |&((x, y), _)| maze[y][x] == 'E');
    let end = end.context("Path to end not found")?;
    let best_score = parents[&end].1;

    // Reconstruct all possible best paths by looking for nodes that connect to the best path and
    // have the same cost at the point of connection.
    let mut best_paths_nodes = HashSet::from_iter(build_path(&end, &parents));
    loop {
        let join_node = parents.iter().find(|(node, (_parent, node_cost))| {
            !best_paths_nodes.contains(node)
                && successors(node).any(|(succ_node, succ_cost)| {
                    // `node` joins a best path with same cost, so it's also part of a best path.
                    best_paths_nodes.contains(&succ_node)
                        && succ_node != start
                        && node_cost + succ_cost == parents[&succ_node].1
                })
        });
        let Some((node, _)) = join_node else { break };
        best_paths_nodes.extend(build_path(node, &parents));
    }
    let best_paths_tiles: HashSet<_> = best_paths_nodes.iter().map(|&(pos, _)| pos).collect();

    aoc::answers(best_score, best_paths_tiles.len())
}

#[test]
fn bad_inputs_test() {
    assert_eq!(run("").unwrap_err().to_string(), "Start not found");
    assert_eq!(run("S").unwrap_err().to_string(), "Path to end not found");
    assert_eq!(run("S#E").unwrap_err().to_string(), "Path to end not found");
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
