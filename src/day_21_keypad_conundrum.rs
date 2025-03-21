use itertools::Itertools;
use rustc_hash::FxHashMap as HashMap;

pub fn run(input: &str) -> aoc::Answer {
    let numeric_keypad = &HashMap::from_iter([
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
    let directional_keypad = &HashMap::from_iter([
        (' ', (0, 0)),
        ('^', (1, 0)),
        ('A', (2, 0)),
        ('<', (0, 1)),
        ('v', (1, 1)),
        ('>', (2, 1)),
    ]);
    let p1_keypad_chain = [numeric_keypad, directional_keypad, directional_keypad];
    let mut p2_keypad_chain = vec![numeric_keypad];
    p2_keypad_chain.extend(vec![directional_keypad; 24]);

    // aoc::answers(
    //     run_keypad_chain(input, &p1_keypad_chain),
    //     run_keypad_chain(input, &p2_keypad_chain),
    // )
    aoc::answer(run_keypad_chain(input, &p1_keypad_chain))
}

fn run_keypad_chain(input: &str, keypad_chain: &[&HashMap<char, (i32, i32)>]) -> u64 {
    let code_complexities_sum: u64 = input
        .lines()
        .map(|code| {
            // println!("processing {code}");
            let sequences = keypad_chain
                .iter()
                .fold(vec![code.to_string()], |seqs, keypad| {
                    let mut next_sequences = seqs
                        .into_iter()
                        .flat_map(|seq| get_button_presses(&seq, keypad))
                        .collect_vec();
                    let min_seq_len = next_sequences
                        .iter()
                        .map(|seq| seq.len())
                        .min()
                        .unwrap_or(0);
                    next_sequences.retain(|seq| seq.len() == min_seq_len);
                    // println!("#seqs: {}", next_sequences.len());
                    // println!("min seq: {min_seq_len}");

                    next_sequences
                });
            // println!();
            let min_sequence_len = sequences
                .iter()
                .map(|seq| seq.len() as u64)
                .min()
                .unwrap_or(0);
            let code_num = code[0..3].parse::<u64>().unwrap();
            min_sequence_len * code_num
        })
        .sum();
    code_complexities_sum
}

fn get_button_presses(seq: &str, keypad: &HashMap<char, (i32, i32)>) -> Vec<String> {
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
    assert_eq!(run(sample).unwrap(), "126384")
}
