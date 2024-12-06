use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> Vec<Vec<char>> {
    let reader = BufReader::new(File::open("data").unwrap());

    let mut result = Vec::new();

    reader.lines().for_each(|line| {
        let line = line.unwrap();
        result.push(line.chars().collect());
    });

    result
}

fn find_guard(map: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '^' {
                return (i, j);
            }
        }
    }
    unreachable!();
}

fn part1(map: &Vec<Vec<char>>, guard: (usize, usize)) -> usize {
    get_visited_fields(map, guard).len()
}

fn get_visited_fields(map: &Vec<Vec<char>>, guard: (usize, usize)) -> HashSet<(usize, usize)> {
    let (mut y, mut x) = guard;
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut dir = 0;

    let mut visited = HashSet::new();
    loop {
        visited.insert((y, x));
        let (dy, dx) = directions[dir];

        let new_y = add_i32_to_usize(y, dy);
        let new_x = add_i32_to_usize(x, dx);

        if new_x.is_none() || new_y.is_none() {
            break;
        }

        let new_x = new_x.unwrap();
        let new_y = new_y.unwrap();
        if new_x >= map[0].len() || new_y >= map.len() {
            break;
        }

        if map[new_y][new_x] == '#' {
            dir = (dir + 1) % 4;
            continue;
        }
        y = new_y;
        x = new_x;
    }

    visited
}

fn part2(map: &Vec<Vec<char>>, guard: (usize, usize)) -> usize {
    let visited = get_visited_fields(map, guard);
    let mut map = map.clone();
    let mut result = 0;

    for (y, x) in visited {
        if y == guard.0 && x == guard.1 {
            continue;
        }
        map[y][x] = '#';
        if has_loop(&map, guard) {
            result += 1;
        }
        map[y][x] = '.';
    }

    result
}

fn has_loop(map: &Vec<Vec<char>>, guard: (usize, usize)) -> bool {
    let (mut y, mut x) = guard;
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut dir = 0;

    let mut positions = HashSet::new();
    loop {
        if positions.contains(&(y, x, dir)) {
            return true;
        }
        positions.insert((y, x, dir));
        let (dy, dx) = directions[dir];

        let new_y = add_i32_to_usize(y, dy);
        let new_x = add_i32_to_usize(x, dx);

        if new_x.is_none() || new_y.is_none() {
            return false;
        }

        let new_x = new_x.unwrap();
        let new_y = new_y.unwrap();
        if new_x >= map[0].len() || new_y >= map.len() {
            return false;
        }

        if map[new_y][new_x] == '#' {
            dir = (dir + 1) % 4;
            continue;
        }
        y = new_y;
        x = new_x;
    }
}

fn add_i32_to_usize(u: usize, i: i32) -> Option<usize> {
    if i > 0 {
        u.checked_add(i as usize)
    } else {
        u.checked_sub((-i) as usize)
    }
}

fn main() {
    let map = parse_input();
    let pos = find_guard(&map);
    println!("part 1: {}", part1(&map, pos));
    println!("part 2: {}", part2(&map, pos));
}
