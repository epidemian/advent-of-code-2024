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

    let [total_p1, total_p2] = [can_equal, can_equal_with_concat].map(|can_equal| {
        calibration_equations
            .iter()
            .filter(|&(val, ops)| can_equal(*val, ops[0], &ops[1..]))
            .map(|(val, _)| val)
            .sum::<u64>()
    });
    Ok(format!("{total_p1} {total_p2}"))
}

fn can_equal(value: u64, first_op: u64, more_ops: &[u64]) -> bool {
    match more_ops {
        [] => first_op == value,
        [second_op, rest @ ..] => {
            can_equal(value, first_op + second_op, rest)
                || can_equal(value, first_op * second_op, rest)
        }
    }
}

fn can_equal_with_concat(value: u64, first_op: u64, more_ops: &[u64]) -> bool {
    match more_ops {
        [] => first_op == value,
        [second_op, rest @ ..] => {
            can_equal_with_concat(value, first_op + second_op, rest)
                || can_equal_with_concat(value, first_op * second_op, rest)
                || can_equal_with_concat(value, concat(first_op, *second_op), rest)
        }
    }
}

fn concat(a: u64, b: u64) -> u64 {
    let decimal_digits = b.checked_ilog10().unwrap_or(0) + 1;
    a * 10_u64.pow(decimal_digits) + b
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

#[test]
fn concat_zero_test() {
    assert_eq!(run("420: 42 0").unwrap(), "0 420")
}
