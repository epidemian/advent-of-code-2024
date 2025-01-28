use itertools::{iproduct, Itertools};

pub fn run(input: &str) -> aoc::Answer {
    let claw_machines: Vec<_> = input.split("\n\n").map(parse_claw_machine).try_collect()?;
    let total_tokens: u64 = claw_machines.iter().filter_map(min_tokens).sum();
    aoc::answer(total_tokens, "")
}

type Point = (u64, u64);

fn parse_claw_machine(s: &str) -> aoc::Result<(Point, Point, Point)> {
    let numbers = aoc::parse_numbers(s)?;
    let [ax, ay, bx, by, price_x, price_y] = numbers[..].try_into()?;
    Ok(((ax, ay), (bx, by), (price_x, price_y)))
}

fn min_tokens(&((ax, ay), (bx, by), (price_x, price_y)): &(Point, Point, Point)) -> Option<u64> {
    iproduct!(0..=100, 0..=100)
        .filter(|(a, b)| (a * ax + b * bx, a * ay + b * by) == (price_x, price_y))
        .map(|(a, b)| a * 3 + b)
        .min()
}

#[test]
fn sample_test() {
    let sample = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";
    assert_eq!(run(sample).unwrap(), "480 ")
}
