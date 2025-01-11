use anyhow::Context;
use itertools::Itertools;
use std::collections::HashSet;

pub fn run(input: &str) -> aoc::Result<String> {
    let (rules_part, updates_part) = input.split_once("\n\n").context("invalid input")?;
    let rules: HashSet<(u32, u32)> = rules_part
        .lines()
        .map(|line| {
            aoc::parse_numbers(line)?
                .into_iter()
                .collect_tuple()
                .context("expected two numbers")
        })
        .try_collect()?;
    let mut updates: Vec<Vec<u32>> = updates_part.lines().map(aoc::parse_numbers).try_collect()?;

    let mut sums = [0, 0];
    for pages in &mut updates {
        let mut needed_sorting = false;
        while fix_single_broken_rule(pages, &rules) {
            needed_sorting = true;
        }
        let middle_page = pages.get(pages.len() / 2);
        sums[needed_sorting as usize] += middle_page.unwrap_or(&0);
    }

    Ok(format!("{} {}", sums[0], sums[1]))
}

fn fix_single_broken_rule(pages: &mut [u32], ordering_rules: &HashSet<(u32, u32)>) -> bool {
    for (page_idx, &page) in pages.iter().enumerate() {
        for (page_after_idx, &page_after) in pages.iter().enumerate().skip(page_idx + 1) {
            if ordering_rules.contains(&(page_after, page)) {
                // An ordering rule is broken because `page_after` should go before `page`.
                // Fix that ordering rule by swapping those two pages.
                pages.swap(page_idx, page_after_idx);
                return true;
            }
        }
    }
    false
}

#[test]
fn empty_update_line_test() {
    let sample = "1|2\n\n ";
    assert_eq!(run(sample).unwrap(), "0 0")
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
