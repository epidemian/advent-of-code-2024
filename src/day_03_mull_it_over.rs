use regex::Regex;

pub fn run(input: &str) -> aoc::Answer {
    aoc::answer(run_program(input, false)?, run_program(input, true)?)
}

fn run_program(input: &str, conditionals: bool) -> aoc::Result<u64> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut sum = 0;
    let mut mul_enabled = true;
    for capture in re.captures_iter(input) {
        match capture.get(0).unwrap().as_str() {
            "do()" => mul_enabled = true,
            "don't()" => mul_enabled = false,
            _ => {
                if !conditionals || mul_enabled {
                    let op1 = capture.get(1).unwrap().as_str().parse::<u64>()?;
                    let op2 = capture.get(2).unwrap().as_str().parse::<u64>()?;
                    sum += op1 * op2;
                }
            }
        }
    }
    Ok(sum)
}

#[test]
fn sample_1_test() {
    let sample = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(run_program(sample, false).unwrap(), 161)
}

#[test]
fn sample_2_test() {
    let sample = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(run_program(sample, true).unwrap(), 48)
}
