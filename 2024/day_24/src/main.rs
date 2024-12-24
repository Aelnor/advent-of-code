use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone, Debug)]
enum Operation {
    Xor,
    And,
    Or,
}

#[derive(Clone, Debug)]
struct Instruction {
    wire1: String,
    wire2: String,
    operation: Operation,
    output: String,
}

fn parse_input() -> (Vec<Instruction>, HashMap<String, u8>) {
    let mut instructions = Vec::new();
    let mut state = HashMap::new();

    let reader = BufReader::new(File::open("data").unwrap());

    let mut state_parsed = false;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            state_parsed = true;
            continue;
        }

        if !state_parsed {
            let (name, value) = line.split_once(':').unwrap();
            state.insert(name.to_string(), value.trim().parse::<u8>().unwrap());
            continue;
        }
        let re = Regex::new(r"(\w+) (\w+) (\w+) -> (\w+)").unwrap();
        let caps = re.captures(&line).unwrap();
        let op = match &caps[2] {
            "XOR" => Operation::Xor,
            "OR" => Operation::Or,
            "AND" => Operation::And,
            _ => unreachable!(),
        };
        instructions.push(Instruction {
            wire1: caps[1].to_string(),
            wire2: caps[3].to_string(),
            operation: op,
            output: caps[4].to_string(),
        });
    }

    (instructions, state)
}

fn execute(instructions: &Vec<Instruction>, state: &mut HashMap<String, u8>) -> usize {
    let mut done = false;
    while !done {
        done = true;
        for i in instructions {
            let arg1 = state.get(&i.wire1);
            let arg2 = state.get(&i.wire2);
            if arg1.is_none() || arg2.is_none() {
                done = false;
                continue;
            }
            if state.contains_key(&i.output) {
                continue;
            }
            let arg1 = arg1.unwrap();
            let arg2 = arg2.unwrap();
            let res = match i.operation {
                Operation::Or => arg1 | arg2,
                Operation::Xor => arg1 ^ arg2,
                Operation::And => arg1 & arg2,
            };
            state.insert(i.output.clone(), res);
        }
    }

    let mut result = 0;
    for (k, v) in state {
        let re = Regex::new(r"z(\d{2})").unwrap();
        if let Some(caps) = re.captures(&k) {
            let n = caps[1].parse::<u8>().unwrap();
            if *v != 0 {
                result |= 1 << n;
            }
        }
    }
    result
}

fn part2(instructions: &Vec<Instruction>) {
    // this is not automated solution.
    // I generated a graphwiz file with the scheme with this code:
    /*
    for i in instructions {
        let inst = match i.operation {
            Operation::And => "AND",
            Operation::Xor => "XOR",
            Operation::Or => "OR",
        };
        println!("{} -> {} [label={inst}]", i.wire1, i.output);
        println!("{} -> {} [label={inst}]", i.wire2, i.output);
    }*/

    // then I looked for errors bit by bit
    // Every time an error is found I examined the scheme to find an issue.
    // The errors turned out to be very local, so, the approach was kinda fast
    for i in 0..45 {
        let mut state = HashMap::new();
        let n = format!("{:02}", i);
        println!("trying {n}");

        for j in 0..45 {
            let val = if j == i { 1 } else { 0 };
            state.insert(format!("x{:02}", j), val);
            state.insert(format!("y{:02}", j), val);
        }
        let expected = (1 << i) * 2;
        let got = execute(instructions, &mut state);
        if expected != got {
            println!("i: {i}, exp: {expected}, got: {got}");
            for (k, v) in state {
                if v != 0 {
                    println!("{k}: {v}");
                }
            }

            break;
        }
    }
}

fn main() {
    let (instructions, mut state) = parse_input();
    println!("Part 1: {}", execute(&instructions, &mut state));
    part2(&instructions);
}
