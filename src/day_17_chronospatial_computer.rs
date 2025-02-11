use anyhow::{bail, Context};
use itertools::Itertools;

pub fn run(input: &str) -> aoc::Answer {
    let (mut a, mut b, mut c, program) = parse_program(input)?;
    let mut ip = 0;
    let mut outputs = vec![];
    while ip < program.len() {
        let opcode = program[ip];
        let operand = program[ip + 1];
        let combo_op = || match operand {
            0..=3 => operand,
            4 => a,
            5 => b,
            6 => c,
            _ => unreachable!("invalid operand value {operand}"),
        };
        match opcode {
            0 => a >>= combo_op(),
            1 => b ^= operand,
            2 => b = combo_op() % 8,
            3 => {
                if a != 0 {
                    ip = operand as usize;
                    continue;
                }
            }
            4 => b ^= c,
            5 => outputs.push(combo_op() % 8),
            7 => c = a >> combo_op(),
            _ => {
                bail!("Invalid opcode {opcode}")
            }
        }
        ip += 2;
    }

    aoc::answer(outputs.iter().join(","))
}

fn parse_program(input: &str) -> aoc::Result<(u64, u64, u64, Vec<u64>)> {
    let (registers, program) = input.split_once("\n\n").context("Invalid input")?;
    let registers = aoc::parse_numbers(registers)?;
    let [a, b, c] = registers[..].try_into().context("Expected 3 registers")?;
    let program = aoc::parse_numbers(program)?;
    Ok((a, b, c, program))
}

#[test]
fn sample_test() {
    let sample = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";
    assert_eq!(run(sample).unwrap(), "4,6,3,5,6,3,5,2,1,0")
}
