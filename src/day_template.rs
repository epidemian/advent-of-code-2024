pub fn run(input: &str) -> aoc::Answer {
    aoc::answer("to", "do")
}

#[test]
fn sample_test() {
    let sample = "";
    assert_eq!(run(sample).unwrap(), "to do")
}
