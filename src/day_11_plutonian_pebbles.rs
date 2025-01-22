use std::collections::HashMap;

pub fn run(input: &str) -> aoc::Result<String> {
    let stones: Vec<u64> = aoc::parse_numbers(input)?;
    let p1_count = count_stones(&stones, 25);
    let p2_count = count_stones(&stones, 75);
    Ok(format!("{p1_count} {p2_count}"))
}

fn count_stones(stones: &[u64], blink_count: u64) -> u64 {
    let mut stone_counts: HashMap<u64, u64> = stones.iter().map(|&s| (s, 1)).collect();
    for _ in 0..blink_count {
        let mut new_counts = HashMap::new();
        for (&stone, &count) in stone_counts.iter() {
            let mut add_stones = |stone| *new_counts.entry(stone).or_insert(0) += count;
            if stone == 0 {
                add_stones(1);
            } else if let Some((l, r)) = split_digits(stone) {
                add_stones(l);
                add_stones(r);
            } else {
                add_stones(stone * 2024);
            }
        }
        stone_counts = new_counts;
    }
    stone_counts.into_values().sum()
}

fn split_digits(n: u64) -> Option<(u64, u64)> {
    let digits = n.checked_ilog10()? + 1;
    if digits % 2 != 0 {
        return None;
    }
    let d = 10u64.pow(digits / 2);
    Some((n / d, n % d))
}

#[test]
fn sample_test() {
    assert_eq!(count_stones(&[125, 17], 25), 55312)
}

#[test]
fn split_digits_test() {
    assert_eq!(split_digits(0), None);
    assert_eq!(split_digits(5), None);
    assert_eq!(split_digits(123), None);
    assert_eq!(split_digits(42), Some((4, 2)));
    assert_eq!(split_digits(4321), Some((43, 21)));
}
