use anyhow::{Context, bail};
use itertools::Itertools;
use rustc_hash::FxHashMap as HashMap;

pub fn run(input: &str) -> aoc::Answer {
    let wires = parse_wires(input)?;
    aoc::answers(get_numeric_output(&wires), get_swapped_wires(&wires))
}

#[derive(PartialEq, PartialOrd)]
enum Wire<'a> {
    Input(bool),
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    Xor(&'a str, &'a str),
}
use Wire::*;

fn parse_wires(input: &str) -> aoc::Result<HashMap<&str, Wire>> {
    let (inputs, gates) = input
        .split_once("\n\n")
        .context("section separator not found")?;

    let inputs = inputs.lines().map(|line| {
        let (name, value) = line.split_once(": ").context("invalid line")?;
        Ok((name, Input(value == "1")))
    });

    let gates = gates.lines().map(|line| {
        Ok(match line.split(' ').collect_tuple() {
            Some((a, "AND", b, "->", out)) => (out, And(a, b)),
            Some((a, "OR", b, "->", out)) => (out, Or(a, b)),
            Some((a, "XOR", b, "->", out)) => (out, Xor(a, b)),
            _ => bail!("invalid gate line '{line}'"),
        })
    });

    inputs.chain(gates).try_collect()
}

fn get_numeric_output(wires: &HashMap<&str, Wire>) -> u64 {
    wires
        .keys()
        .filter(|name| name.starts_with('z'))
        .map(|name| {
            let value = get_value(name, wires) as u64;
            let bit: u32 = name[1..].parse().unwrap();
            value << bit
        })
        .sum()
}

fn get_value(name: &str, wires: &HashMap<&str, Wire>) -> bool {
    match wires[name] {
        Input(val) => val,
        And(a, b) => get_value(a, wires) & get_value(b, wires),
        Or(a, b) => get_value(a, wires) | get_value(b, wires),
        Xor(a, b) => get_value(a, wires) ^ get_value(b, wires),
    }
}

// Note: This only works for the special case of the wiring being a full adder of 45-bit numbers
// with the specific shape of the input.
// TODO: Try to refactor this mess.
fn get_swapped_wires(wires: &HashMap<&str, Wire>) -> String {
    let sorted_wires = |a, b| {
        let (a, b) = if wires[a] < wires[b] { (a, b) } else { (b, a) };
        (a, b, (&wires[a], &wires[b]))
    };

    let mut bad_wires = Vec::new();
    for (&name, wire) in wires {
        if name.starts_with('z') {
            let is_good_z = matches!(wire, Xor(..)) || (matches!(wire, Or(..)) && name == "z45");
            if !is_good_z {
                bad_wires.push(name);
                continue;
            }
        }
        match *wire {
            Xor(a, b) => {
                let (a, b, inputs) = sorted_wires(a, b);
                if matches!(inputs, (Input(..), Input(..))) {
                    continue;
                }
                if !matches!(inputs, (Or(..), Xor(..))) {
                    if matches!(inputs, (And(..), Xor(..))) && name == "z01" {
                        continue;
                    }
                    bad_wires.push(if matches!(inputs.0, Or(..)) { b } else { a });
                }
            }
            And(a, b) => {
                let (a, b, inputs) = sorted_wires(a, b);
                if matches!(inputs, (Input(..), Input(..))) {
                    continue;
                }
                if !matches!(inputs, (Or(..), Xor(..))) {
                    if matches!(inputs, (And("y00", "x00"), Xor(..))) {
                        continue;
                    }
                    bad_wires.push(if matches!(inputs.0, Or(..)) { b } else { a });
                }
            }
            Or(a, b) => {
                let (a, b, inputs) = sorted_wires(a, b);
                if !matches!(inputs, (And(..), And(..))) {
                    bad_wires.push(if matches!(inputs.0, And(..)) { b } else { a });
                }
            }
            Input(_) => {}
        }
    }
    bad_wires.iter().unique().sorted().join(",")
}

#[test]
fn small_sample_test() {
    let sample = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
";
    let wires = parse_wires(sample).unwrap();
    assert_eq!(get_numeric_output(&wires), 4);
}

#[test]
fn large_sample_test() {
    let sample = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
";
    let wires = parse_wires(sample).unwrap();
    assert_eq!(get_numeric_output(&wires), 2024);
}
