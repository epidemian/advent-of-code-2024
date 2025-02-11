use anyhow::Context;
use itertools::Itertools;

pub fn run(input: &str) -> aoc::Answer {
    let (a, b, c, program) = parse_program(input)?;
    let p1_ans = run_program(a, b, c, &program).iter().join(",");
    let p2_ans = (0..1_000_000)
        .find(|&a| run_program(a, b, c, &program) == program)
        .context("Could not find a value for A")?;
    aoc::answers(p1_ans, p2_ans)
}

fn run_program(mut a: u64, mut b: u64, mut c: u64, program: &[u64]) -> Vec<u64> {
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
                unreachable!("Invalid opcode {opcode}")
            }
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
