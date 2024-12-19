use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> (Vec<String>, Vec<String>) {
    let reader = BufReader::new(File::open("data").unwrap());
    let mut towels = Vec::new();
    let mut targets = Vec::new();
    let mut towels_parsed = false;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            towels_parsed = true;
            continue;
        }
        if !towels_parsed {
            towels = line.split(',').map(|e| String::from(e.trim())).collect();
            continue;
        }
        targets.push(String::from(line));
    }
    (towels, targets)
}

fn part1(towels: &Vec<String>, targets: &Vec<String>) -> usize {
    let mut cache = HashMap::new();

    targets
        .iter()
        .filter(|t| possibilities(t, towels, &mut cache) != 0)
        .count()
}

fn part2(towels: &Vec<String>, targets: &Vec<String>) -> usize {
    let mut cache = HashMap::new();
    targets
        .iter()
        .map(|t| possibilities(t, towels, &mut cache))
        .sum()
}

fn possibilities(
    target: &String,
    towels: &Vec<String>,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if target.is_empty() {
        return 1;
    }

    if let Some(res) = cache.get(target) {
        return *res;
    }

    let mut result = 0;

    for towel in towels {
        if target.starts_with(towel) {
            let new_target = String::from(&target[towel.len()..]);
            result += possibilities(&new_target, towels, cache);
        }
    }
    cache.insert(target.to_string(), result);

    result
}

fn main() {
    let (towels, targets) = parse_input();
    println!("part1: {}", part1(&towels, &targets));
    println!("part2: {}", part2(&towels, &targets));
}
