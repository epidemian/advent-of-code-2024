pub fn run(input: &str) -> aoc::Answer {
    let secrets = aoc::parse_numbers(input)?;
    let sum: u64 = secrets
        .into_iter()
        .map(|secret| (0..2000).fold(secret, |s, _| rand(s)))
        .sum();
    aoc::answer(sum)
}

fn rand(mut s: u64) -> u64 {
    let mask = 0xFFFFFF;
    s = ((s * 64) ^ s) & mask;
    s = ((s / 32) ^ s) & mask;
    s = ((s * 2048) ^ s) & mask;
    s
}

#[test]
fn sample_test() {
    let sample = "1
10
100
2024
";
    assert_eq!(run(sample).unwrap(), "37327623")
}
