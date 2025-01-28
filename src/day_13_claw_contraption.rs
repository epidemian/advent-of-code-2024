use itertools::Itertools;

pub fn run(input: &str) -> aoc::Answer {
    let claw_machines: Vec<_> = input.split("\n\n").map(parse_claw_machine).try_collect()?;
    let p1_total_tokens: u64 = claw_machines
        .iter()
        .filter_map(|&(a, b, p)| min_tokens(a, b, p))
        .sum();
    let p2_total_tokens: u64 = claw_machines
        .iter()
        .filter_map(|&(a, b, (px, py))| {
            min_tokens(a, b, (px + 10000000000000.0, py + 10000000000000.0))
        })
        .sum();
    aoc::answer(p1_total_tokens, p2_total_tokens)
}

type Point = (f64, f64);

fn parse_claw_machine(s: &str) -> aoc::Result<(Point, Point, Point)> {
    let numbers = aoc::parse_numbers(s)?;
    let [ax, ay, bx, by, price_x, price_y] = numbers[..].try_into()?;
    Ok(((ax, ay), (bx, by), (price_x, price_y)))
}

fn min_tokens((ax, ay): Point, (bx, by): Point, (px, py): Point) -> Option<u64> {
    // We want to find the number of button presses, `a` and `b`, solving these equations:
    // a*ax + b*bx = px
    // a*ay + b*by = py
    // So:
    // a = (px - b*bx)/ax
    // b = (py - px*ay/ax) / (by - bx*ay/ax)
    let b = (py - px * ay / ax) / (by - bx * ay / ax);
    let a = (px - b * bx) / ax;
    // Since we used floating-point math, the results are approximate.
    Some(close_to_integer(a)? * 3 + close_to_integer(b)?)
}

fn close_to_integer(a: f64) -> Option<u64> {
    let close_enough = (a - a.round()).abs() < 0.001;
    (close_enough).then(|| a.round() as u64)
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
    assert_eq!(run(sample).unwrap(), "480 875318608908")
}
