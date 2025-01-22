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
        let mut new_stone_counts = HashMap::new();
        let mut add_stones = |stone, count| *new_stone_counts.entry(stone).or_insert(0) += count;
        for (&stone, &count) in stone_counts.iter() {
            if stone == 0 {
                add_stones(1, count);
            } else {
                let stone_str = stone.to_string();
                if stone_str.len() % 2 == 0 {
                    let (l, r) = stone_str.split_at(stone_str.len() / 2);
                    let (l, r) = (l.parse().unwrap(), r.parse().unwrap());
                    add_stones(l, count);
                    add_stones(r, count);
                } else {
                    add_stones(stone * 2024, count);
                }
            }
        }
        stone_counts = new_stone_counts;
    }
    stone_counts.into_values().sum()
}

#[test]
fn sample_test() {
    assert_eq!(count_stones(&[125, 17], 25), 55312)
}
