use anyhow::bail;
use itertools::Itertools;
use pathfinding::prelude::maximal_cliques_collect;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

pub fn run(input: &str) -> aoc::Answer {
    let conns = parse_connections(input)?;
    let triplets_count = conns
        .iter()
        .filter(|(id, _)| id[0] == b't')
        .flat_map(|(&id, computers)| {
            computers
                .iter()
                .tuple_combinations()
                .filter(|&(a, b)| conns[a].contains(b))
                .map(move |(&a, &b)| [id, a, b].into_iter().sorted().collect_vec())
        })
        .unique()
        .count();
    let parties = maximal_cliques_collect(conns.keys().copied(), &mut |a, b| conns[a].contains(b));
    let max_party = parties.iter().max_by_key(|p| p.len()).unwrap();
    let password = max_party
        .iter()
        .flat_map(|id| std::str::from_utf8(id))
        .sorted()
        .join(",");
    aoc::answers(triplets_count, password)
}

type Id = [u8; 2];

fn parse_connections(input: &str) -> aoc::Result<HashMap<Id, HashSet<Id>>> {
    let mut conns = HashMap::<Id, HashSet<Id>>::default();
    for line in input.lines() {
        let &[a, b, b'-', c, d] = line.as_bytes() else {
            bail!("invalid input line '{line}'");
        };
        conns.entry([a, b]).or_default().insert([c, d]);
        conns.entry([c, d]).or_default().insert([a, b]);
    }
    Ok(conns)
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
