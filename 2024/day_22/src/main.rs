use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> Vec<u64> {
    BufReader::new(File::open("data").unwrap())
        .lines()
        .map(|l| l.unwrap().parse::<u64>().unwrap())
        .collect()
}

fn part1(numbers: &Vec<u64>) -> u64 {
    let mut sum = 0;
    for n in numbers {
        let mut n = *n;
        for _ in 0..2000 {
            n = calculate_secret_number(n);
        }
        sum += n;
    }
    sum
}

fn part2(numbers: &Vec<u64>) -> u64 {
    let mut bananas = HashMap::new();
    for n in numbers {
        let mut n = *n;
        let mut prev_n = n;
        let mut seen = HashSet::new();
        let mut vec = VecDeque::new();
        for _ in 0..2000 {
            n = calculate_secret_number(n);
            let d = n % 10;
            let change = (prev_n as i64 % 10) - (d as i64);
            prev_n = n;
            vec.push_back(change);
            if vec.len() > 4 {
                vec.pop_front();
            }
            if vec.len() < 4 {
                continue;
            }
            if seen.insert(vec.clone()) {
                *bananas.entry(vec.clone()).or_insert(0) += d;
            }
        }
    }
    *bananas.values().max().unwrap() as u64
}

fn calculate_secret_number(n: u64) -> u64 {
    let mut n = n;
    n = prune(mix(n * 64, n));
    n = prune(mix(n / 32, n));
    n = prune(mix(n * 2048, n));

    n
}

fn mix(a: u64, b: u64) -> u64 {
    a ^ b
}

fn prune(a: u64) -> u64 {
    a % 16777216
}

fn main() {
    let numbers = parse_input();
    println!("Part 1: {}", part1(&numbers));
    println!("Part 2: {}", part2(&numbers));
}
