use anyhow::Context;
use itertools::iproduct;
use pathfinding::prelude::bfs_reach;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::hash::Hash;

// Alternative day 16 solution, implementing an ad-hoc Dijkstra algorithm, which works similar to
// `dijkstra_partial` from the `pathfinding` crate, but keeping track of multiple shortest paths.
#[allow(dead_code)]
pub fn run(input: &str) -> aoc::Answer {
    let (maze, w, h) = aoc::parse_char_grid(input)?;
    let start_pos = iproduct!(0..w, 0..h)
        .find(|&(x, y)| maze[y][x] == 'S')
        .context("start not found")?;
    let dirs = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let start = (start_pos, 0);
    let successors = |((x, y), d): ((usize, usize), usize)| {
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
    let (parents, end) = dijkstra_multi_path(start, successors, |((x, y), _)| maze[y][x] == 'E');
    let end = end.context("path to end not found")?;
    let best_score = parents[&end].1;

    // Get all best paths' tiles from `parents` using BFS starting from the end.
    let best_paths_nodes = bfs_reach(&end, |node| {
        parents.get(node).into_iter().flat_map(|(ps, _)| ps)
    });
    let best_paths_tiles: HashSet<_> = best_paths_nodes.map(|&(pos, _)| pos).collect();

    aoc::answers(best_score, best_paths_tiles.len())
}

#[allow(clippy::type_complexity)]
fn dijkstra_multi_path<N, I>(
    start: N,
    successors: impl Fn(N) -> I,
    is_goal: impl Fn(N) -> bool,
) -> (HashMap<N, (Vec<N>, usize)>, Option<N>)
where
    N: Copy + Hash + Ord,
    I: IntoIterator<Item = (N, usize)>,
{
    let mut to_visit = BinaryHeap::new();
    let mut parents = HashMap::default();

    // Use Reverse to make `to_visit` a min-heap where the node with the minimum cost is first.
    to_visit.push(Reverse((0, start)));

    while let Some(Reverse((node_cost, node))) = to_visit.pop() {
        if is_goal(node) {
            return (parents, Some(node));
        }

        for (succ, move_cost) in successors(node) {
            let succ_cost = node_cost + move_cost;
            match parents.get_mut(&succ) {
                None => {
                    parents.insert(succ, (vec![node], succ_cost));
                    to_visit.push(Reverse((succ_cost, succ)));
                }
                Some((existing_parents, existing_cost)) => {
                    if succ_cost == *existing_cost {
                        existing_parents.push(node);
                    }
                    if succ_cost < *existing_cost {
                        (*existing_parents, *existing_cost) = (vec![node], succ_cost);
                        to_visit.push(Reverse((succ_cost, succ)));
                    }
                }
            }
        }
    }
    (parents, None)
}

#[test]
fn bad_inputs_test() {
    assert_eq!(run("").unwrap_err().to_string(), "start not found");
    assert_eq!(run("S").unwrap_err().to_string(), "path to end not found");
    assert_eq!(run("S#E").unwrap_err().to_string(), "path to end not found");
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
