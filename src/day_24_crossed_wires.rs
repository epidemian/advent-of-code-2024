use anyhow::{Context, bail};
use itertools::Itertools;
use rustc_hash::FxHashMap as HashMap;

pub fn run(input: &str) -> aoc::Answer {
    let (inputs, gates) = parse_wires(input)?;
    aoc::answers(
        get_numeric_output(&inputs, &gates),
        get_swapped_wires(&gates),
    )
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Op {
    And,
    Or,
    Xor,
}
use Op::*;

type WireMap<'a, T> = HashMap<&'a str, T>;
type Gate<'a> = (Op, &'a str, &'a str);

fn parse_wires(input: &str) -> aoc::Result<(WireMap<bool>, WireMap<Gate>)> {
    let (inputs, gates) = input
        .split_once("\n\n")
        .context("section separator not found")?;

    let inputs = inputs.lines().map(|line| -> aoc::Result<_> {
        let (name, value) = line.split_once(": ").context("invalid line")?;
        Ok((name, value == "1"))
    });

    let gates = gates.lines().map(|line| {
        Ok(match line.split(' ').collect_tuple() {
            Some((a, "AND", b, "->", out)) => (out, (And, a, b)),
            Some((a, "OR", b, "->", out)) => (out, (Or, a, b)),
            Some((a, "XOR", b, "->", out)) => (out, (Xor, a, b)),
            _ => bail!("invalid gate line '{line}'"),
        })
    });

    Ok((inputs.try_collect()?, gates.try_collect()?))
}

fn get_numeric_output(inputs: &WireMap<bool>, gates: &WireMap<Gate>) -> u64 {
    gates
        .keys()
        .filter(|name| name.starts_with('z'))
        .filter_map(|name| {
            let value = get_value(name, inputs, gates) as u64;
            let bit: u32 = name[1..].parse().ok()?;
            Some(value << bit)
        })
        .sum()
}

fn get_value(name: &str, inputs: &WireMap<bool>, gates: &WireMap<Gate>) -> bool {
    if let Some(&val) = inputs.get(name) {
        return val;
    };
    let Some((op, a, b)) = gates.get(name) else {
        return false;
    };
    let [a, b] = [a, b].map(|n| get_value(n, inputs, gates));
    match op {
        And => a & b,
        Or => a | b,
        Xor => a ^ b,
    }
}

/// Detects swapped wires on a 45-bit full adder circuit
///
/// This circuit can be broken down into 45 1-bit adders. The first of which only consists of an AND
/// and an OR gate, and should look like this:
///
///     ┌───┐       ┌───┐   ┌───┐
///     │x00├──┬────┤XOR├───┤z00│
///     └───┘  │ ┌──┤   │   └───┘
///            │ │  └───┘
///     ┌───┐  │ │  ┌───┐
///     │y00├────┴──┤AND├─────┐
///     └───┘  └────┤   │     │
///                 └───┘    z00
///                         carry
///
/// The other 44 1-bit adders also take the carry bit from the previous adder. They are made of two
/// XOR, two AND, and an OR gate. For example, the x10 and y10 adder should look like this:
///
///       z09
///      carry────────────────┐
///                           │
///     ┌───┐       ┌───┐     │  ┌───┐     ┌───┐
///     │x10├──┬────┤XOR├───┬────┤XOR├─────┤z10│
///     └───┘  │ ┌──┤   │   │ ├──┤   │     └───┘
///            │ │  └───┘   │ │  └───┘
///     ┌───┐  │ │  ┌───┐   │ │  ┌───┐
///     │y10├────┴──┤AND├─┐ │ └──┤AND├─┐
///     └───┘  └────┤   │ │ └────┤   │ │ ┌───┐
///                 └───┘ │      └───┘ └─┤OR ├──┐
///                       └──────────────┤   │  │
///                                      └───┘ z10
///                                           carry
///
/// Diagrams made with ASCIIFlow (https://asciiflow.com)
///
/// Note: This function is not general by any means. It only works for the special case of this
/// specific 45-bit full adder circuit. And it probably doesn't even detect all possible wire swaps,
/// but it works for the given input file.
fn get_swapped_wires(gates: &WireMap<Gate>) -> String {
    let mut gate_outputs: WireMap<Vec<&str>> = WireMap::default();
    for (&name, &(_op, a, b)) in gates {
        gate_outputs.entry(name).or_default();
        gate_outputs.entry(a).or_default().push(name);
        gate_outputs.entry(b).or_default().push(name);
    }
    let is_input = |name: &str| name.starts_with(['x', 'y']);

    let mut bad_wires = Vec::new();
    for (&name, &(op, a, b)) in gates {
        let out_gates = gate_outputs[name]
            .iter()
            .map(|&c| gates[c].0)
            .sorted()
            .collect_vec();
        let ok_wiring = match op {
            And => out_gates == [Or] || out_gates == [And, Xor] && (a, b) == ("y00", "x00"),
            Or => out_gates == [And, Xor] || name == "z45",
            Xor => out_gates == [And, Xor] && is_input(a) && is_input(b) || name.starts_with('z'),
        };
        if !ok_wiring {
            bad_wires.push(name);
        }
    }

    bad_wires.iter().sorted().join(",")
}

#[test]
fn empty_input_test() {
    assert_eq!(
        run("").unwrap_err().to_string(),
        "section separator not found"
    );
    assert_eq!(run("\n\n").unwrap(), "0 ");
}

#[test]
fn bad_inputs_test() {
    assert_eq!(
        run("bad input\n\n").unwrap_err().to_string(),
        "invalid line"
    );
    assert_eq!(
        run("x0: 1\n\nbad gate").unwrap_err().to_string(),
        "invalid gate line 'bad gate'"
    );
    assert_eq!(
        run("x0: 1\n\nx0 NARF x0 -> z0").unwrap_err().to_string(),
        "invalid gate line 'x0 NARF x0 -> z0'"
    );
}

#[test]
fn bad_output_name_test() {
    assert_eq!(run("x0: 1\n\nx0 AND x0 -> zXXX").unwrap(), "0 zXXX");
}

#[test]
fn unconnected_wire_test() {
    assert_eq!(run("x0: 1\n\nx0 AND foo -> z0").unwrap(), "0 z0");
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
    let (inputs, gates) = parse_wires(sample).unwrap();
    assert_eq!(get_numeric_output(&inputs, &gates), 4);
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
    let (inputs, gates) = parse_wires(sample).unwrap();
    assert_eq!(get_numeric_output(&inputs, &gates), 2024);
}
