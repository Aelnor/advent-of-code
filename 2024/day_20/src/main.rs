use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone, Default)]
struct Point {
    row: usize,
    col: usize,
}

fn parse_input() -> Vec<Vec<char>> {
    let reader = BufReader::new(File::open("data").unwrap());

    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

fn find_start_and_finish(map: &Vec<Vec<char>>) -> (Point, Point) {
    let mut start = Point::default();
    let mut finish = Point::default();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'S' {
                start.row = i;
                start.col = j;
                continue;
            }
            if map[i][j] == 'E' {
                finish.row = i;
                finish.col = j;
            }
        }
    }
    (start, finish)
}

fn find_cheats(
    map: &Vec<Vec<char>>,
    distances: &Vec<Vec<usize>>,
    row: usize,
    col: usize,
    cheats: &mut HashMap<usize, usize>,
    radius: usize,
) {
    for i in (row as isize - radius as isize)..=(row as isize + radius as isize) {
        if i < 0 || i as usize >= map.len() {
            continue;
        }

        for j in (col as isize - radius as isize)..=(col as isize + radius as isize) {
            if j < 0 || j as usize >= map[i as usize].len() {
                continue;
            }
            if distances[i as usize][j as usize] == std::usize::MAX {
                continue;
            }
            let d = distance((row as isize, col as isize), (i, j));
            if d > radius {
                continue;
            }

            let gain = distances[i as usize][j as usize] as isize
                - distances[row][col] as isize
                - d as isize;

            if gain <= 0 {
                continue;
            }

            *cheats.entry(gain as usize).or_insert(0) += 1;
        }
    }
}

fn distance(p1: (isize, isize), p2: (isize, isize)) -> usize {
    let (x1, y1) = p1;
    let (x2, y2) = p2;

    ((x2 - x1).abs() + (y2 - y1).abs()) as usize
}

fn solve(map: &Vec<Vec<char>>, steps: usize) -> usize {
    let (start, finish) = find_start_and_finish(map);
    let mut step = 0;
    let mut col = start.col;
    let mut row = start.row;
    let mut distances = vec![vec![std::usize::MAX; map[0].len()]; map.len()];
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    loop {
        distances[row][col] = step;
        for (drow, dcol) in directions {
            let new_row = add_i32_to_usize(row, drow).unwrap();
            let new_col = add_i32_to_usize(col, dcol).unwrap();
            if map[new_row][new_col] != '#' && distances[new_row][new_col] == std::usize::MAX {
                col = new_col;
                row = new_row;
                break;
            }
        }
        step += 1;
        if col == finish.col && row == finish.row {
            distances[row][col] = step;
            break;
        }
    }

    let mut cheats = HashMap::new();
    col = start.col;
    row = start.row;

    loop {
        find_cheats(map, &distances, row, col, &mut cheats, steps);

        for (drow, dcol) in directions {
            let new_row = add_i32_to_usize(row, drow).unwrap();
            let new_col = add_i32_to_usize(col, dcol).unwrap();
            if map[new_row][new_col] != '#' && distances[new_row][new_col] > distances[row][col] {
                col = new_col;
                row = new_row;
                break;
            }
        }
        if col == finish.col && row == finish.row {
            break;
        }
    }

    cheats
        .into_iter()
        .filter(|(k, _)| *k >= 100)
        .map(|(_, v)| v)
        .sum()
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
    println!("Part 1: {}", solve(&map, 2));
    println!("Part 2: {}", solve(&map, 20));
}
