use anyhow::Context;
use itertools::Itertools;
use pathfinding::prelude::maximal_cliques_collect;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

pub fn run(input: &str) -> aoc::Answer {
    let conns = parse_connections(input)?;
    let triplets_count = conns
        .iter()
        .filter(|(id, _)| id.starts_with('t'))
        .flat_map(|(&id, computers)| {
            computers
                .iter()
                .tuple_combinations()
                .filter(|&(a, b)| conns[a].contains(b))
                .map(move |(&a, &b)| [id, a, b].into_iter().sorted().collect_vec())
        })
        .unique()
        .count();
    let cliques = maximal_cliques_collect(conns.keys().copied(), &mut |a, b| conns[a].contains(b));
    let max_clique = cliques.iter().max_by_key(|c| c.len()).unwrap();
    let password = max_clique.iter().sorted().join(",");
    aoc::answers(triplets_count, password)
}

fn parse_connections(input: &str) -> aoc::Result<HashMap<&str, HashSet<&str>>> {
    let mut conns = HashMap::<&str, HashSet<&str>>::default();
    for line in input.lines() {
        let (a, b) = line.split_once('-').context("invalid input line")?;
        conns.entry(a).or_default().insert(b);
        conns.entry(b).or_default().insert(a);
    }
    Ok(conns)
}

#[test]
fn empty_input_test() {
    assert_eq!(run("").unwrap(), "0 ")
}

#[test]
fn sample_test() {
    let sample = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";
    assert_eq!(run(sample).unwrap(), "7 co,de,ka,ta")
}
