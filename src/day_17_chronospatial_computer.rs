use anyhow::{Context, ensure};
use itertools::Itertools;
use pathfinding::prelude::bfs;

pub fn run(input: &str) -> aoc::Answer {
    let (a, b, c, program) = parse_program(input)?;
    let output = run_program(a, b, c, &program).iter().join(",");

    // Our input program is "2,4,1,7,7,5,1,7,0,3,4,1,5,5,3,0", which translates to:
    //
    //     2,4: b = a % 8
    //     1,7: b ^= 7
    //     7,5: c = a >> b
    //     1,7: b ^= 7
    //     0,3: a >>= 3
    //     4,1: b ^= c
    //     5,5: output(b % 8)
    //     3,0: if a != 0 goto 0
    //
    // This is a loop that consumes the 3 least significant bits (an octet) of A and outputs a
    // single value on each iteration, util A becomes zero. So the number of outputs depends on the
    // number of initial bits on A. To output the 16 numbers on our program, A must have at least 16
    // octets (48 bits). Since the last output only depends on the most significant octet of A, we
    // start the search there and go "backwards" towards the least significant octets searching for
    // A inputs that produce the expected output for each octet.
    //
    // This solution is not general, but works for the given input and the part 2 sample, which is
    // also a loop that "consumes" octets from register A.
    let (min_a, _) = bfs(
        &(0, program.len()),
        |&(a, i)| {
            let i = i - 1;
            (0..8)
                .map(|oct| a | oct << (i * 3))
                .filter(|&a| run_program(a, b, c, &program).get(i) == Some(&program[i]))
                .map(|a| (a, i))
                .collect_vec()
        },
        |&(_a, i)| i == 0,
    )
    .and_then(|path| path.last().copied())
    .context("Could not find a value A to make the program a quine")?;

    ensure!(run_program(min_a, b, c, &program) == program);

    aoc::answers(output, min_a)
}

fn run_program(mut a: u64, mut b: u64, mut c: u64, program: &[u64]) -> Vec<u64> {
    let mut ip = 0;
    let mut outputs = vec![];
    while ip + 1 < program.len() {
        let opcode = program[ip];
        let operand = program[ip + 1];
        let combo_op = match operand {
            4 => a,
            5 => b,
            6 => c,
            _ => operand,
        };
        match opcode {
            0 => a >>= combo_op,
            1 => b ^= operand,
            2 => b = combo_op % 8,
            3 => {
                if a != 0 {
                    ip = operand as usize;
                    continue;
                }
            }
            4 => b ^= c,
            5 => outputs.push(combo_op % 8),
            6 => b = a >> combo_op,
            7 => c = a >> combo_op,
            _ => {}
        }
        ip += 2;
    }
    outputs
}

fn parse_program(input: &str) -> aoc::Result<(u64, u64, u64, Vec<u64>)> {
    let (registers, program) = input.split_once("\n\n").context("Invalid input")?;
    let registers = aoc::parse_numbers(registers)?;
    let [a, b, c] = registers[..].try_into().context("Expected 3 registers")?;
    let program = aoc::parse_numbers(program)?;
    Ok((a, b, c, program))
}

#[test]
fn part_1_sample_test() {
    let sample = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";
    let (a, b, c, prog) = parse_program(sample).unwrap();
    assert_eq!(run_program(a, b, c, &prog), [4, 6, 3, 5, 6, 3, 5, 2, 1, 0])
}

#[test]
fn part_2_sample_test() {
    let sample = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";
    assert_eq!(run(sample).unwrap(), "5,7,3,0 117440")
}
