use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> Vec<Vec<usize>> {
    let reader = BufReader::new(File::open("data").unwrap());
    let mut result = Vec::new();

    for line in reader.lines() {
        result.push(
            line.unwrap()
                .chars()
                .map(|e| e.to_digit(10).unwrap() as usize)
                .collect(),
        );
    }

    result
}

fn part1(map: &Vec<Vec<usize>>) -> usize {
    let mut result = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 0 {
                result += calc_score(map, i, j, false);
            }
        }
    }
    result
}

fn part2(map: &Vec<Vec<usize>>) -> usize {
    let mut result = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 0 {
                result += calc_score(map, i, j, true);
            }
        }
    }
    result
}

fn calc_score(map: &Vec<Vec<usize>>, i: usize, j: usize, part2: bool) -> usize {
    let mut reached_peaks = HashSet::new();
    let mut part2_score = 0;
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let mut states = VecDeque::new();
    states.push_back((i, j));
    while let Some((i, j)) = states.pop_front() {
        if map[i][j] == 9 {
            reached_peaks.insert((i, j));
            part2_score += 1;
        }
        for (di, dj) in &directions {
            let new_i = add_i32_to_usize(i, *di);
            let new_j = add_i32_to_usize(j, *dj);

            if new_i.is_none() || new_j.is_none() {
                continue;
            }
            let new_i = new_i.unwrap();
            let new_j = new_j.unwrap();
            if new_i >= map.len() || new_j >= map[0].len() {
                continue;
            }

            if map[new_i][new_j] == map[i][j] + 1 {
                states.push_back((new_i, new_j));
            }
        }
    }

    if part2 {
        part2_score
    } else {
        reached_peaks.len()
    }
}

fn add_i32_to_usize(u: usize, i: i32) -> Option<usize> {
    if i < 0 {
        u.checked_sub((-i) as usize)
    } else {
        u.checked_add(i as usize)
    }
}

fn main() {
    let map = parse_input();
    println!("part 1: {}", part1(&map));
    println!("part 2: {}", part2(&map));
}
