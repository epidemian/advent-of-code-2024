use itertools::iproduct;
use std::iter::zip;

pub fn run(input: &str) -> aoc::Answer {
    let (locks, keys): (Vec<_>, Vec<_>) = input.split("\n\n").partition(|s| s.starts_with("#####"));
    let fitting_pairs = iproduct!(locks, keys).filter(|(lock, key)| {
        !zip(lock.chars(), key.chars()).any(|(c1, c2)| c1 == '#' && c2 == '#')
    });
    aoc::answer(fitting_pairs.count())
}

#[test]
fn sample_test() {
    let sample = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";
    assert_eq!(run(sample).unwrap(), "3")
}
