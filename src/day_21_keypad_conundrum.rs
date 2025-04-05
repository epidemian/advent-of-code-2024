use regex::Regex;
use rustc_hash::FxHashMap as HashMap;

pub fn run(input: &str) -> aoc::Answer {
    aoc::answers(
        get_complexities_sum(input, 2),
        get_complexities_sum(input, 25),
    )
}

fn get_complexities_sum(input: &str, dir_robot_count: usize) -> usize {
    let num_pad = &build_keypad("789\n456\n123\n 0A");
    let dir_pad = &build_keypad(" ^A\n<v>");
    let keypad_chain = [vec![num_pad], vec![dir_pad; dir_robot_count]].concat();

    let code_re = Regex::new(r"([0-9]+)A").unwrap();
    let mut cache = HashMap::default();
    let code_complexities = code_re.captures_iter(input).map(|cap| {
        let (code, [code_num]) = cap.extract();
        let code_num: usize = code_num.parse().unwrap();
        let min_seq_len = shortest_final_sequence_len(code, &keypad_chain, &mut cache);
        min_seq_len * code_num
    });
    code_complexities.sum()
}

type Keypad = HashMap<char, (usize, usize)>;

fn build_keypad(s: &str) -> Keypad {
    s.lines()
        .enumerate()
        .flat_map(|(y, row)| row.chars().enumerate().map(move |(x, ch)| (ch, (x, y))))
        .collect()
}

fn shortest_final_sequence_len(
    sequence: &str,
    keypads: &[&Keypad],
    cache: &mut HashMap<(String, usize), usize>,
) -> usize {
    if keypads.is_empty() {
        return sequence.len();
    }
    if let Some(&min_len) = cache.get(&(sequence.to_string(), keypads.len())) {
        return min_len;
    }
    let sub_seq_lengths = sequence.split_inclusive('A').map(|sub_seq| {
        calc_button_presses(sub_seq, keypads[0])
            .into_iter()
            .map(|next_seq| shortest_final_sequence_len(&next_seq, &keypads[1..], cache))
            .min()
            .unwrap()
    });
    let total_len = sub_seq_lengths.sum();
    cache.insert((sequence.to_string(), keypads.len()), total_len);
    total_len
}

fn calc_button_presses(seq: &str, keypad: &Keypad) -> Vec<String> {
    let (mut x, mut y) = keypad[&'A'];
    let (bad_x, bad_y) = keypad[&' '];
    let mut button_presses = vec![String::new()];

    for ch in seq.chars() {
        let (end_x, end_y) = keypad[&ch];
        let h_moves = if end_x < x { "<" } else { ">" }.repeat(end_x.abs_diff(x));
        let v_moves = if end_y < y { "^" } else { "v" }.repeat(end_y.abs_diff(y));
        let h_first_path = [&h_moves, &v_moves, "A"].join("");
        let v_first_path = [&v_moves, &h_moves, "A"].join("");

        let ch_paths = if h_moves.is_empty() || v_moves.is_empty() {
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
fn empty_input_test() {
    assert_eq!(run("").unwrap(), "0 0");
}

#[test]
fn bad_inputs_test() {
    assert!(run("A").is_ok());
    assert!(run("\n").is_ok());
    assert!(run("xxx").is_ok());
}

#[test]
fn sample_test() {
    let sample = "029A
980A
179A
456A
379A
";
    assert_eq!(get_complexities_sum(sample, 2), 126384)
}
