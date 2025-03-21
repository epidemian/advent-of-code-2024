use rustc_hash::FxHashMap as HashMap;

pub fn run(input: &str) -> aoc::Answer {
    aoc::answers(run_robots_chain(input, 2), run_robots_chain(input, 25))
}

fn run_robots_chain(input: &str, robot_count: u32) -> usize {
    let numeric_keypad = HashMap::from_iter([
        ('7', (0, 0)),
        ('8', (1, 0)),
        ('9', (2, 0)),
        ('4', (0, 1)),
        ('5', (1, 1)),
        ('6', (2, 1)),
        ('1', (0, 2)),
        ('2', (1, 2)),
        ('3', (2, 2)),
        (' ', (0, 3)),
        ('0', (1, 3)),
        ('A', (2, 3)),
    ]);
    let directional_keypad = HashMap::from_iter([
        (' ', (0, 0)),
        ('^', (1, 0)),
        ('A', (2, 0)),
        ('<', (0, 1)),
        ('v', (1, 1)),
        ('>', (2, 1)),
    ]);
    let mut cache = HashMap::default();
    let code_complexities = input.lines().map(|code| {
        let min_seq_len = calc_button_presses(code, &numeric_keypad)
            .iter()
            .map(|sequence| {
                get_shortest_final_sequence(sequence, &directional_keypad, robot_count, &mut cache)
            })
            .min()
            .unwrap();
        let code_num = code[0..3].parse::<usize>().unwrap();
        min_seq_len * code_num
    });
    code_complexities.sum()
}

fn get_shortest_final_sequence(
    sequence: &str,
    keypad: &HashMap<char, (i32, i32)>,
    robot_depth: u32,
    cache: &mut HashMap<(String, u32), usize>,
) -> usize {
    if robot_depth == 0 {
        return sequence.len();
    }
    if let Some(&min_len) = cache.get(&(sequence.to_string(), robot_depth)) {
        return min_len;
    }
    let mut total_len = 0;
    for sub_seq in sequence.split_inclusive('A') {
        total_len += calc_button_presses(sub_seq, keypad)
            .into_iter()
            .map(|next_robot_seq| {
                get_shortest_final_sequence(&next_robot_seq, keypad, robot_depth - 1, cache)
            })
            .min()
            .unwrap();
    }
    cache.insert((sequence.to_string(), robot_depth), total_len);
    total_len
}

fn calc_button_presses(seq: &str, keypad: &HashMap<char, (i32, i32)>) -> Vec<String> {
    let (mut x, mut y) = keypad[&'A'];
    let mut button_presses = vec![String::new()];
    let (bad_x, bad_y) = keypad[&' '];

    for ch in seq.chars() {
        let (end_x, end_y) = keypad[&ch];
        let (dx, dy) = (end_x - x, end_y - y);
        let h_moves = if dx < 0 { "<" } else { ">" }.repeat(dx.unsigned_abs() as usize);
        let v_moves = if dy < 0 { "^" } else { "v" }.repeat(dy.unsigned_abs() as usize);
        let h_first_path = String::new() + &h_moves + &v_moves + "A";
        let v_first_path = String::new() + &v_moves + &h_moves + "A";

        let ch_paths = if dx == 0 || dy == 0 {
            vec![h_first_path]
        } else if y == bad_y && end_x == bad_x {
            vec![v_first_path]
        } else if x == bad_x && end_y == bad_y {
            vec![h_first_path]
        } else {
            vec![h_first_path, v_first_path]
        };

        button_presses = button_presses
            .into_iter()
            .flat_map(|s| ch_paths.iter().map(move |ch_path| s.clone() + ch_path))
            .collect();
        (x, y) = (end_x, end_y);
    }
    button_presses
}

#[test]
fn sample_test() {
    let sample = "029A
980A
179A
456A
379A
";
    assert_eq!(run_robots_chain(sample, 2), 126384)
}
