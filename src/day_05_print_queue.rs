use anyhow::Context;
use itertools::Itertools;

pub fn run(input: &str) -> aoc::Result<String> {
    let (rules_part, updates_part) = input.split_once("\n\n").context("invalid input")?;
    let rules: Vec<(u32, u32)> = rules_part
        .lines()
        .map(|line| {
            aoc::parse_numbers(line)?
                .into_iter()
                .collect_tuple()
                .context("expected two numbers")
        })
        .try_collect()?;
    let updates: Vec<Vec<u32>> = updates_part.lines().map(aoc::parse_numbers).try_collect()?;

    let (correct_updates, mut incorrect_updates): (Vec<_>, Vec<_>) = updates
        .into_iter()
        .partition(|pages| find_broken_rule(pages, &rules).is_none());
    let middle_page_sum_p1 = add_middle_pages(&correct_updates);

    for pages in &mut incorrect_updates {
        while let Some((i, j)) = find_broken_rule(pages, &rules) {
            pages.swap(i, j);
        }
    }
    let middle_page_sum_p2 = add_middle_pages(&incorrect_updates);

    Ok(format!("{middle_page_sum_p1} {middle_page_sum_p2}"))
}

fn find_broken_rule(pages: &[u32], rules: &[(u32, u32)]) -> Option<(usize, usize)> {
    rules.iter().find_map(|(before, after)| {
        let before_index = pages.iter().position(|page| page == before)?;
        let after_index = pages.iter().position(|page| page == after)?;
        (before_index > after_index).then_some((before_index, after_index))
    })
}

fn add_middle_pages(updates: &[Vec<u32>]) -> u32 {
    updates.iter().map(|pages| pages[pages.len() / 2]).sum()
}

#[test]
fn sample_test() {
    let sample = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
    assert_eq!(run(sample).unwrap(), "143 123")
}
