use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone, Eq, Debug, PartialEq, Default)]
struct Machine {
    a: (usize, usize),
    b: (usize, usize),
    prize: (usize, usize),
}

fn parse_input() -> Vec<Machine> {
    let mut result = Vec::new();
    let mut line_number = 0;
    let mut current_machine = Machine::default();

    let reader = BufReader::new(File::open("data").unwrap());
    for line in reader.lines() {
        let line = line.unwrap();

        match line_number {
            0 => {
                let re = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
                let caps = re.captures(&line).unwrap();
                current_machine.a.0 = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
                current_machine.a.1 = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            }
            1 => {
                let re = Regex::new(r"^Button B: X\+(\d+), Y\+(\d+)$").unwrap();
                let caps = re.captures(&line).unwrap();
                current_machine.b.0 = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
                current_machine.b.1 = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            }
            2 => {
                let re = Regex::new(r"^Prize: X=(\d+), Y=(\d+)$").unwrap();
                let caps = re.captures(&line).unwrap();
                current_machine.prize.0 = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
                current_machine.prize.1 = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
                result.push(current_machine);
                current_machine = Machine::default();
            }
            3 => {}
            _ => unreachable!(),
        }
        line_number = (line_number + 1) % 4;
    }

    result
}

fn part(machines: &Vec<Machine>, with_limit: bool) -> usize {
    let mut result = 0;
    for m in machines {
        if let Some((a, b)) = solve(m) {
            if !with_limit || (a < 101 && b < 101) {
                result += a * 3 + b;
            }
        }
    }
    result.try_into().unwrap()
}

fn solve(machine: &Machine) -> Option<(isize, isize)> {
    // The system is:
    // (xdistancea)a + (xdistanceb)b = prize.0
    // (ydistancea)a + (ydistanceb)b = prize.1
    let d = (machine.a.0 * machine.b.1) as isize - (machine.a.1 * machine.b.0) as isize;
    if d == 0 {
        return None;
    }
    let a = ((machine.prize.0 * machine.b.1) as f64 - (machine.prize.1 * machine.b.0) as f64)
        / d as f64;
    let b = ((machine.prize.1 * machine.a.0) as f64 - (machine.prize.0 * machine.a.1) as f64)
        / d as f64;

    if !is_integer(a) || !is_integer(b) {
        return None;
    }

    Some((a.floor() as isize, b.floor() as isize))
}

fn is_integer(x: f64) -> bool {
    // Check if the fractional part is effectively zero
    x.fract().abs() < f64::EPSILON
}

fn main() {
    let mut machines = parse_input();
    println!("part 1: {}", part(&machines, true));
    for i in 0..machines.len() {
        machines[i].prize.0 += 10000000000000;
        machines[i].prize.1 += 10000000000000;
    }
    println!("part 2: {}", part(&machines, false));
}
