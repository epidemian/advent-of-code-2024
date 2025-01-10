use anyhow::Context;
use itertools::Itertools;

pub fn run(input: &str) -> aoc::Result<String> {
    let (rules_part, updates_part) = input.split_once("\n\n").context("invalid input")?;
    let rules: Vec<[u32; 2]> = rules_part
        .lines()
        .map(|line| {
            let numbers = aoc::parse_numbers(line)?;
            numbers[..].try_into().context("expected two numbers")
        })
        .try_collect()?;
    let updates: Vec<Vec<u32>> = updates_part.lines().map(aoc::parse_numbers).try_collect()?;

    let (correct_updates, mut incorrect_updates): (Vec<_>, Vec<_>) =
        updates.into_iter().partition(|pages| {
            rules.iter().all(|[before, after]| {
                let Some(before_index) = pages.iter().position(|page| page == before) else {
                    return true;
                };
                let Some(after_index) = pages.iter().position(|page| page == after) else {
                    return true;
                };
                before_index < after_index
            })
        });
    let middle_page_sum_p1: u32 = correct_updates
        .iter()
        .map(|pages| pages[pages.len() / 2])
        .sum();

    for pages in &mut incorrect_updates {
        let update_rules: Vec<_> = rules
            .iter()
            .filter(|[before, after]| pages.contains(before) && pages.contains(after))
            .copied()
            .collect();
        loop {
            let offending_rule = update_rules.iter().find_map(|[before, after]| {
                let before_index = pages.iter().position(|page| page == before)?;
                let after_index = pages.iter().position(|page| page == after)?;
                (before_index > after_index).then_some((before_index, after_index))
            });
            let Some((before_index, after_index)) = offending_rule else {
                break;
            };
            pages.swap(before_index, after_index);
        }
    }
    let middle_page_sum_p2: u32 = incorrect_updates
        .iter()
        .map(|pages| pages[pages.len() / 2])
        .sum();

    Ok(format!("{middle_page_sum_p1} {middle_page_sum_p2}"))
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
