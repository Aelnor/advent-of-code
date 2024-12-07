use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> Vec<(usize, Vec<usize>)> {
    let mut result = Vec::new();

    let reader = BufReader::new(File::open("data").unwrap());

    for line in reader.lines() {
        let line = line.unwrap();

        let (sum, rest) = line.split_once(':').unwrap();
        result.push((
            sum.parse::<usize>().unwrap(),
            rest.trim()
                .split(' ')
                .map(|e| e.parse::<usize>().unwrap())
                .collect(),
        ));
    }
    result
}

fn part1(data: &Vec<(usize, Vec<usize>)>) -> usize {
    solve(data, &vec!['*', '+'])
}

fn part2(data: &Vec<(usize, Vec<usize>)>) -> usize {
    solve(data, &vec!['*', '+', '|'])
}

fn solve(data: &Vec<(usize, Vec<usize>)>, opers: &Vec<char>) -> usize {
    let mut result = 0;

    for (sum, nums) in data {
        if fixable(*sum, nums, opers) {
            result += sum;
        }
    }

    result
}

fn fixable(sum: usize, parts: &Vec<usize>, opers: &Vec<char>) -> bool {
    let permutations = opers.len().pow(parts.len() as u32 - 1);
    for i in 0..permutations {
        if calc(parts, i, opers) == sum {
            return true;
        }
    }
    false
}

fn calc(parts: &Vec<usize>, args: usize, opers: &Vec<char>) -> usize {
    let mut args = args;
    let mut result = parts[0];
    for i in 1..parts.len() {
        let num = args % opers.len();
        args /= opers.len();
        match opers[num] {
            '*' => result *= parts[i],
            '+' => result += parts[i],
            '|' => {
                result = (result.to_string() + &parts[i].to_string())
                    .parse::<usize>()
                    .unwrap()
            }
            _ => unreachable!(),
        }
    }
    result
}

fn main() {
    let data = parse_input();
    println!("part 1: {}", part1(&data));
    println!("part 2: {}", part2(&data));
}
