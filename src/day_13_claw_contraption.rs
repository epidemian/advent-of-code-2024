use itertools::Itertools;

pub fn run(input: &str) -> aoc::Answer {
    let claw_machines: Vec<_> = input.split("\n\n").map(parse_claw_machine).try_collect()?;
    let p1_total_tokens: u64 = claw_machines
        .iter()
        .map(|&(a, b, p)| min_tokens(a, b, p))
        .sum();
    let p2_total_tokens: u64 = claw_machines
        .iter()
        .map(|&(a, b, (px, py))| min_tokens(a, b, (px + 10000000000000, py + 10000000000000)))
        .sum();
    aoc::answer(p1_total_tokens, p2_total_tokens)
}

type Point = (i64, i64);

fn parse_claw_machine(s: &str) -> aoc::Result<(Point, Point, Point)> {
    let numbers = aoc::parse_numbers(s)?;
    let [ax, ay, bx, by, price_x, price_y] = numbers[..].try_into()?;
    Ok(((ax, ay), (bx, by), (price_x, price_y)))
}

fn min_tokens((ax, ay): Point, (bx, by): Point, (px, py): Point) -> u64 {
    // We want to find the number of button presses, `a` and `b`, solving these equations:
    // a*ax + b*bx = px
    // a*ay + b*by = py
    // So:
    // a = (px - b * bx) / ax
    // b = (py * ax - px * ay) / (by * ax - bx * ay)
    let (b, b_rem) = div_mod(py * ax - px * ay, by * ax - bx * ay);
    let (a, a_rem) = div_mod(px - b * bx, ax);
    // Key presses must be integers. Ignore non-integer solutions.
    if b_rem != 0 || a_rem != 0 {
        return 0;
    }
    a as u64 * 3 + b as u64
}

fn div_mod(a: i64, b: i64) -> (i64, i64) {
    (a / b, a % b)
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
