use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> (Vec<Vec<usize>>, Vec<(usize, usize)>) {
    let mut page_runs = Vec::new();
    let mut rules = Vec::new();

    let mut rules_processed = false;

    let reader = BufReader::new(File::open("data").unwrap());
    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            rules_processed = true;
            continue;
        }

        if !rules_processed {
            let Some((l, r)) = line.split_once('|') else {
                unreachable!()
            };
            rules.push((l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap()));
            continue;
        }

        page_runs.push(
            line.split(',')
                .map(|e| e.trim().parse::<usize>().unwrap())
                .collect(),
        );
    }
    (page_runs, rules)
}

fn part1(runs: &Vec<Vec<usize>>, rules: &Vec<(usize, usize)>) -> usize {
    let mut result = 0;

    for run in runs {
        let (correct, _) = is_correct(run, rules);
        if correct {
            result += run[run.len() / 2];
        }
    }

    result
}

fn part2(runs: &Vec<Vec<usize>>, rules: &Vec<(usize, usize)>) -> usize {
    let mut result = 0;

    for run in runs {
        let (mut correct, mut indexes) = is_correct(run, rules);
        if correct {
            continue;
        }

        let mut run = run.clone();
        while !correct {
            run.swap(indexes.0, indexes.1);
            (correct, indexes) = is_correct(&run, rules);
        }
        result += run[run.len() / 2];
    }

    result
}

fn is_correct(run: &Vec<usize>, rules: &Vec<(usize, usize)>) -> (bool, (usize, usize)) {
    for rule in rules {
        let left = run.iter().position(|e| *e == rule.0);
        let right = run.iter().position(|e| *e == rule.1);

        if left.is_none() || right.is_none() {
            continue;
        }

        let left = left.unwrap();
        let right = right.unwrap();

        if right < left {
            return (false, (left, right));
        }
    }
    (true, (0, 0))
}

fn main() {
    let (runs, rules) = parse_input();
    println!("part 1: {}", part1(&runs, &rules));
    println!("part 1: {}", part2(&runs, &rules));
}
