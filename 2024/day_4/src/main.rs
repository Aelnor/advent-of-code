use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> Vec<Vec<char>> {
    let reader = BufReader::new(File::open("data").unwrap());

    reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect()
}

fn part1(ws: &Vec<Vec<char>>) -> usize {
    let mut result = 0;
    let directions = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    for i in 0..ws.len() {
        for j in 0..ws[i].len() {
            for d in &directions {
                if find(ws, i, j, *d, 0) {
                    result += 1;
                }
            }
        }
    }

    result
}

fn part2(ws: &Vec<Vec<char>>) -> usize {
    let mut result = 0;
    for i in 0..ws.len() - 2 {
        for j in 0..ws[i].len() - 2 {
            let mut count = 0;

            if find(ws, i, j, (1, 1), 1) {
                count += 1;
            }
            if find(ws, i + 2, j, (-1, 1), 1) {
                count += 1;
            }
            if find(ws, i, j + 2, (1, -1), 1) {
                count += 1;
            }
            if find(ws, i + 2, j + 2, (-1, -1), 1) {
                count += 1;
            }

            assert!(count <= 2);
            if count == 2 {
                result += 1;
            }
        }
    }
    result
}

fn find(ws: &Vec<Vec<char>>, i: usize, j: usize, direction: (i32, i32), step: usize) -> bool {
    if ws[i][j] != letter(step) {
        return false;
    }

    if step == 3 {
        return true;
    }

    let (di, dj) = direction;

    let new_i = add_i32_to_usize(i, di);
    let new_j = add_i32_to_usize(j, dj);

    if new_i.is_none() || new_j.is_none() {
        return false;
    }

    let new_i = new_i.unwrap();
    let new_j = new_j.unwrap();

    if new_j >= ws.len() || new_i >= ws[0].len() {
        return false;
    }

    find(ws, new_i, new_j, direction, step + 1)
}

fn letter(i: usize) -> char {
    match i {
        0 => 'X',
        1 => 'M',
        2 => 'A',
        3 => 'S',
        _ => unreachable!(),
    }
}

fn add_i32_to_usize(base: usize, i: i32) -> Option<usize> {
    if i > 0 {
        return base.checked_add(i as usize);
    }

    return base.checked_sub((-i) as usize);
}

fn main() {
    let ws = parse_input();
    println!("part1: {}", part1(&ws));
    println!("part2: {}", part2(&ws));
}
