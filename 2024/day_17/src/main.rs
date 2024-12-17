use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct Instruction {
    op: Opcode,
    arg: usize,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, Default)]
struct Machine {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    instructions: Vec<Instruction>,
    ip: usize,
}

impl Machine {
    fn resolve_combo(&self, v: usize) -> usize {
        match v {
            0 | 1 | 2 | 3 => v,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => unreachable!(),
        }
    }
}

fn parse_input() -> (Machine, Vec<usize>) {
    let reader = BufReader::new(File::open("data").unwrap());
    let mut machine = Machine::default();
    let mut program = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }

        let (left, right) = line.split_once(':').unwrap();
        match left {
            "Register A" => machine.reg_a = right.trim().parse::<usize>().unwrap(),
            "Register B" => machine.reg_b = right.trim().parse::<usize>().unwrap(),
            "Register C" => machine.reg_c = right.trim().parse::<usize>().unwrap(),
            "Program" => {
                program = right
                    .split(',')
                    .map(|e| e.trim().parse::<usize>().unwrap())
                    .collect();
                machine.instructions = parse_program(right);
            }
            _ => unreachable!(),
        }
    }
    (machine, program)
}

fn parse_program(s: &str) -> Vec<Instruction> {
    let split: Vec<_> = s
        .split(',')
        .map(|e| e.trim().parse::<usize>().unwrap())
        .collect();

    let mut result = Vec::new();
    let mut i = 0;
    while i < split.len() {
        let command = split[i];
        let arg = split[i + 1];
        i += 2;

        match command {
            0 => result.push(Instruction {
                op: Opcode::Adv,
                arg,
            }),
            1 => result.push(Instruction {
                op: Opcode::Bxl,
                arg,
            }),
            2 => result.push(Instruction {
                op: Opcode::Bst,
                arg,
            }),
            3 => result.push(Instruction {
                op: Opcode::Jnz,
                arg,
            }),
            4 => result.push(Instruction {
                op: Opcode::Bxc,
                arg,
            }),
            5 => result.push(Instruction {
                op: Opcode::Out,
                arg,
            }),
            6 => result.push(Instruction {
                op: Opcode::Bdv,
                arg,
            }),
            7 => result.push(Instruction {
                op: Opcode::Cdv,
                arg,
            }),
            _ => unreachable!(),
        }
    }

    result
}

fn part1(machine: &Machine) -> String {
    run(machine, 0)
        .into_iter()
        .map(|e| e.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn run(m: &Machine, stop_after: usize) -> Vec<usize> {
    let mut machine = m.clone();
    let mut output = Vec::new();

    loop {
        let ip = machine.ip / 2;
        if ip >= machine.instructions.len() {
            break;
        }

        let instr = machine.instructions[ip];

        match instr.op {
            Opcode::Adv => {
                let r = machine.reg_a
                    / 2usize.pow(machine.resolve_combo(instr.arg).try_into().unwrap());
                machine.reg_a = r;
            }
            Opcode::Bxl => {
                machine.reg_b ^= instr.arg;
            }
            Opcode::Bst => {
                machine.reg_b = machine.resolve_combo(instr.arg) % 8;
            }
            Opcode::Jnz => {
                if machine.reg_a != 0 {
                    machine.ip = instr.arg;
                    continue;
                }
            }
            Opcode::Bxc => {
                machine.reg_b ^= machine.reg_c;
            }
            Opcode::Out => {
                output.push(machine.resolve_combo(instr.arg) % 8);
                if stop_after == output.len() {
                    return output;
                }
            }
            Opcode::Bdv => {
                let r = machine.reg_a
                    / 2usize.pow(machine.resolve_combo(instr.arg).try_into().unwrap());
                machine.reg_b = r;
            }
            Opcode::Cdv => {
                let r = machine.reg_a
                    / 2usize.pow(machine.resolve_combo(instr.arg).try_into().unwrap());
                machine.reg_c = r;
            }
        }
        machine.ip += 2;
    }
    output
}

fn search(m: &Machine, program: &Vec<usize>, candidate: usize) -> Option<usize> {
    let mut m = m.clone();
    let mut program = program.clone();
    let target = program.pop().unwrap();

    for i in 0..8 {
        m.reg_a = candidate + i;
        if run(&m, 1) == vec![target] {
            if program.is_empty() {
                return Some(m.reg_a);
            }
            if let Some(num) = search(&m, &program, m.reg_a * 8) {
                return Some(num);
            }
        }
    }
    None
}

fn part2(m: &Machine, program: &Vec<usize>) -> usize {
    search(m, program, 0).unwrap()
}

fn main() {
    let (m, p) = parse_input();
    println!("part1: {}", part1(&m));
    println!("part2: {}", part2(&m, &p));
}
