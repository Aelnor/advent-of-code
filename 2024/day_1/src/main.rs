use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> (Vec<u32>, Vec<u32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    let file = File::open("data").unwrap();
    let reader = BufReader::new(file);

    reader.lines().for_each(|l| {
        let l = l.unwrap();
        let split: Vec<_> = l.split("   ").map(|e| e.trim()).collect();
        left.push(split[0].parse::<u32>().unwrap());
        right.push(split[1].parse::<u32>().unwrap());
    });

    (left, right)
}

fn part1(left: &Vec<u32>, right: &Vec<u32>) -> u32 {
    let mut l = left.clone();
    let mut r = right.clone();

    l.sort();
    r.sort();

    let mut result = 0;

    for i in 0..left.len() {
        result += if l[i] > r[i] {
            l[i] - r[i]
        } else {
            r[i] - l[i]
        }
    }

    result
}

fn part2(left: &Vec<u32>, right: &Vec<u32>) -> u32 {
    let mut result = 0;
    let mut right_freq = HashMap::new();

    for loc in right {
        *right_freq.entry(loc).or_insert(0) += 1;
    }

    for loc in left {
        if let Some(right_number) = right_freq.get(&loc) {
            result += loc * right_number;
        }
    }

    result
}

fn main() {
    let (left, right) = parse_input();
    println!("part 1: {}", part1(&left, &right));
    println!("part 2: {}", part2(&left, &right));
}
