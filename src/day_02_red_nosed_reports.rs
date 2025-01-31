use itertools::Itertools;

pub fn run(input: &str) -> aoc::Answer {
    let reports: Vec<_> = input.lines().map(aoc::parse_numbers).try_collect()?;
    aoc::answers(
        reports.iter().filter(|r| is_safe(r)).count(),
        reports.iter().filter(|r| is_safe_with_dampener(r)).count(),
    )
}

fn is_safe(report: &[i64]) -> bool {
    let diffs = || report.iter().tuple_windows().map(|(a, b)| a - b);
    diffs().map(|d| d.signum()).all_equal() && diffs().all(|d| 1 <= d.abs() && d.abs() <= 3)
}

fn is_safe_with_dampener(report: &[i64]) -> bool {
    (0..report.len()).any(|i| {
        let mut new_report = report.to_vec();
        new_report.remove(i);
        is_safe(&new_report)
    })
}

#[test]
fn sample_test() {
    let sample = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
    assert_eq!(run(sample).unwrap(), "2 4")
}
