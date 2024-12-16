use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct State {
    row: usize,
    col: usize,
    score: usize,
    direction: char,
    path: Vec<(usize, usize)>,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Position {
    row: usize,
    col: usize,
    direction: char,
}

fn parse_input() -> (Vec<Vec<char>>, (usize, usize)) {
    let reader = BufReader::new(File::open("data").unwrap());
    let mut result = Vec::new();
    let mut row = 0;
    let mut col = 0;

    for (i, line) in reader.lines().enumerate() {
        let mut line: Vec<_> = line.unwrap().chars().collect();

        if let Some(pos) = line.iter().position(|e| *e == 'S') {
            line[pos] = '.';
        }
        if let Some(pos) = line.iter().position(|e| *e == 'E') {
            line[pos] = '.';
            col = pos;
            row = i;
        }
        result.push(line);
    }
    (result, (row, col))
}

fn solve(map: &Vec<Vec<char>>, target_row: usize, target_col: usize) -> (usize, usize) {
    let mut visited = HashMap::new();
    let row = map.len() - 2;
    let col = 1;

    let mut best_paths = HashSet::new();

    let mut state = VecDeque::new();
    state.push_back(State {
        row,
        col,
        direction: '>',
        score: 0,
        path: vec![(row, col)],
    });

    visited.insert(
        Position {
            row,
            col,
            direction: '>',
        },
        0,
    );
    let mut best_score = std::usize::MAX;

    while let Some(current_state) = state.pop_front() {
        let p = Position {
            row: current_state.row,
            col: current_state.col,
            direction: current_state.direction,
        };
        let score = *visited.get(&p).unwrap_or(&std::usize::MAX);

        if score < current_state.score {
            continue;
        }

        if current_state.row == target_row && current_state.col == target_col {
            if best_score < current_state.score {
                continue;
            }
            if best_score > current_state.score {
                best_paths.clear();
            }
            for e in &current_state.path {
                best_paths.insert(e.clone());
            }

            best_score = current_state.score;
            continue;
        }

        visited.insert(
            Position {
                row: current_state.row,
                col: current_state.col,
                direction: current_state.direction,
            },
            current_state.score,
        );

        let mut turns = vec![(current_state.direction, 1)];
        for t in get_turns(current_state.direction) {
            turns.push((t, 1001));
        }

        for (dir, score) in turns {
            let (drow, dcol) = direction(dir);
            let new_row = add_i32_to_usize(current_state.row, drow).unwrap();
            let new_col = add_i32_to_usize(current_state.col, dcol).unwrap();
            let mut new_path = current_state.path.clone();
            new_path.push((new_row, new_col));
            if map[new_row][new_col] != '#' {
                state.push_back(State {
                    row: new_row,
                    col: new_col,
                    direction: dir,
                    score: current_state.score + score,
                    path: new_path,
                })
            }
        }
    }

    (best_score, best_paths.len())
}

fn direction(c: char) -> (i32, i32) {
    match c {
        '>' => (0, 1),
        '<' => (0, -1),
        '^' => (-1, 0),
        'v' => (1, 0),
        _ => unreachable!(),
    }
}

fn get_turns(c: char) -> [char; 2] {
    match c {
        '>' | '<' => ['^', 'v'],
        '^' | 'v' => ['<', '>'],
        _ => unreachable!(),
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
    let (map, (row, col)) = parse_input();
    let (p1, p2) = solve(&map, row, col);
    println!("part1: {p1}");
    println!("part2: {p2}");
}
