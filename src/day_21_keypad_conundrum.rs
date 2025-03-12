use rustc_hash::FxHashMap as HashMap;

pub fn run(input: &str) -> aoc::Answer {
    let codes = input.lines();
    let code_complexities_sum: u64 = codes
        .map(|code| {
            let sequences = get_numeric_keypad_presses(code)
                .into_iter()
                .flat_map(|seq| get_directional_keypad_presses(&seq))
                .flat_map(|seq| get_directional_keypad_presses(&seq));
            let min_sequence_len = sequences.map(|seq| seq.len() as u64).min().unwrap_or(0);
            let code_num = code[0..3].parse::<u64>().unwrap();
            min_sequence_len * code_num
        })
        .sum();
    aoc::answer(code_complexities_sum)
}

fn get_numeric_keypad_presses(seq: &str) -> Vec<String> {
    let keypad = HashMap::from_iter([
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
    get_button_presses(seq, keypad)
}

fn get_directional_keypad_presses(seq: &str) -> Vec<String> {
    let keypad = HashMap::from_iter([
        (' ', (0, 0)),
        ('^', (1, 0)),
        ('A', (2, 0)),
        ('<', (0, 1)),
        ('v', (1, 1)),
        ('>', (2, 1)),
    ]);
    get_button_presses(seq, keypad)
}

fn get_button_presses(seq: &str, keypad: HashMap<char, (i32, i32)>) -> Vec<String> {
    let (mut x, mut y) = keypad[&'A'];
    let mut button_presses = vec![String::new()];
    let (bad_x, bad_y) = keypad[&' '];

    for ch in seq.chars() {
        let add_moves = |button_presses: &mut [String], moves: &str| {
            button_presses.iter_mut().for_each(|s| s.push_str(moves))
        };
        let (end_x, end_y) = keypad[&ch];
        let (dx, dy) = (end_x - x, end_y - y);
        if dx != 0 && dy != 0 {
            if y == bad_y && end_x == bad_x {
                if dy < 0 {
                    add_moves(&mut button_presses, &"^".repeat(-dy as usize));
                } else {
                    add_moves(&mut button_presses, &"v".repeat(dy as usize));
                }
                if dx < 0 {
                    add_moves(&mut button_presses, &"<".repeat(-dx as usize));
                } else {
                    add_moves(&mut button_presses, &">".repeat(dx as usize));
                }
            } else if x == bad_x && end_y == bad_y {
                if dx < 0 {
                    add_moves(&mut button_presses, &"<".repeat(-dx as usize));
                } else {
                    add_moves(&mut button_presses, &">".repeat(dx as usize));
                }
                if dy < 0 {
                    add_moves(&mut button_presses, &"^".repeat(-dy as usize));
                } else {
                    add_moves(&mut button_presses, &"v".repeat(dy as usize));
                }
            } else {
                let mut button_presses_copy = button_presses.clone();

                if dx < 0 {
                    add_moves(&mut button_presses, &"<".repeat(-dx as usize));
                } else {
                    add_moves(&mut button_presses, &">".repeat(dx as usize));
                }
                if dy < 0 {
                    add_moves(&mut button_presses, &"^".repeat(-dy as usize));
                } else {
                    add_moves(&mut button_presses, &"v".repeat(dy as usize));
                }

                if dy < 0 {
                    add_moves(&mut button_presses_copy, &"^".repeat(-dy as usize));
                } else {
                    add_moves(&mut button_presses_copy, &"v".repeat(dy as usize));
                }
                if dx < 0 {
                    add_moves(&mut button_presses_copy, &"<".repeat(-dx as usize));
                } else {
                    add_moves(&mut button_presses_copy, &">".repeat(dx as usize));
                }
                button_presses.extend(button_presses_copy);
            }
        } else if dx != 0 {
            if dx < 0 {
                add_moves(&mut button_presses, &"<".repeat(-dx as usize));
            } else {
                add_moves(&mut button_presses, &">".repeat(dx as usize));
            }
        } else if dy != 0 {
            if dy < 0 {
                add_moves(&mut button_presses, &"^".repeat(-dy as usize));
            } else {
                add_moves(&mut button_presses, &"v".repeat(dy as usize));
            }
        }
        button_presses.iter_mut().for_each(|s| s.push('A'));
        (x, y) = (end_x, end_y);
    }
    button_presses
}

#[test]
fn numeric_keypad_test() {
    assert_eq!(
        get_numeric_keypad_presses("029A"),
        vec!["<A^A>^^AvvvA", "<A^A^^>AvvvA"]
    )
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
