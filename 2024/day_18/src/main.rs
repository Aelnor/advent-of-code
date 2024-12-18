use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> Vec<(usize, usize)> {
    let reader = BufReader::new(File::open("data").unwrap());
    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let (col, row) = line.split_once(',').unwrap();
        result.push((col.parse::<usize>().unwrap(), row.parse::<usize>().unwrap()));
    }
    result
}

fn make_map(coords: &Vec<(usize, usize)>, size: usize, steps: usize) -> Vec<Vec<char>> {
    let mut map = vec![vec!['.'; size]; size];
    for i in 0..steps {
        let (col, row) = coords[i];
        map[row][col] = '#';
    }
    map
}

fn find_path(map: &Vec<Vec<char>>) -> usize {
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut state = VecDeque::new();
    state.push_back((0, 0, 0));

    while let Some((row, col, distance)) = state.pop_front() {
        if visited[row][col] {
            continue;
        }

        visited[row][col] = true;
        if row == map.len() - 1 && col == map[0].len() - 1 {
            return distance;
        }
        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (drow, dcol) in directions {
            let new_row = add_i32_to_usize(row, drow);
            let new_col = add_i32_to_usize(col, dcol);

            if new_col.is_none() || new_row.is_none() {
                continue;
            }

            let new_row = new_row.unwrap();
            let new_col = new_col.unwrap();
            if new_row == map.len() || new_col == map[0].len() {
                continue;
            }

            if map[new_row][new_col] == '#' {
                continue;
            }

            if visited[new_row][new_col] {
                continue;
            }

            state.push_back((new_row, new_col, distance + 1));
        }
    }
    0
}

fn add_i32_to_usize(u: usize, i: i32) -> Option<usize> {
    if i < 0 {
        u.checked_sub((-i) as usize)
    } else {
        u.checked_add(i as usize)
    }
}

fn part1(map: &Vec<Vec<char>>) -> usize {
    find_path(map)
}

fn part2(map: &Vec<Vec<char>>, coords: &Vec<(usize, usize)>) -> (usize, usize) {
    let mut start = 1024;
    let mut end = coords.len();
    while start != end {
        let mut map = map.clone();
        let mid = start + (end - start) / 2;
        for i in 1024..mid {
            let (col, row) = coords[i];
            map[row][col] = '#';
        }
        if find_path(&map) == 0 {
            end = mid;
        } else {
            start = mid + 1;
        }
    }
    coords[start - 1]
}

fn main() {
    let coords = parse_input();
    let map = make_map(&coords, 71, 1024);
    println!("part1: {}", part1(&map));
    println!("part2: {:?}", part2(&map, &coords));
}
