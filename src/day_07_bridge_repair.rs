use anyhow::ensure;
use itertools::Itertools;

pub fn run(input: &str) -> aoc::Result<String> {
    let calibration_equations: Vec<(u64, Vec<u64>)> = input
        .lines()
        .map(|line| {
            let numbers = aoc::parse_numbers(line)?;
            ensure!(numbers.len() >= 2, "invalid calibration equation: {line}");
            Ok((numbers[0], numbers[1..].to_vec()))
        })
        .try_collect()?;
    let total_p1: u64 = calibration_equations
        .iter()
        .filter(|&(test_value, ops)| can_equal_p1(*test_value, ops[0], &ops[1..]))
        .map(|(val, _)| val)
        .sum();
    let total_p2: u64 = calibration_equations
        .iter()
        .filter(|&(test_value, ops)| can_equal_p2(*test_value, ops[0], &ops[1..]))
        .map(|(val, _)| val)
        .sum();
    Ok(format!("{total_p1} {total_p2}"))
}

fn can_equal_p1(test_value: u64, first_op: u64, rest: &[u64]) -> bool {
    match rest {
        [] => first_op == test_value,
        [second_op, rest @ ..] => {
            can_equal_p1(test_value, first_op + second_op, rest)
                || can_equal_p1(test_value, first_op * second_op, rest)
        }
    }
}

fn can_equal_p2(test_value: u64, first_op: u64, rest: &[u64]) -> bool {
    match rest {
        [] => first_op == test_value,
        [second_op, rest @ ..] => {
            can_equal_p2(test_value, first_op + second_op, rest)
                || can_equal_p2(test_value, first_op * second_op, rest)
                || can_equal_p2(test_value, concat(first_op, *second_op), rest)
        }
    }
}

fn concat(a: u64, b: u64) -> u64 {
    format!("{a}{b}").parse().unwrap()
}

#[test]
fn sample_test() {
    let sample = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
    assert_eq!(run(sample).unwrap(), "3749 11387")
}
