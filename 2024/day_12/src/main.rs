use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Eq, PartialEq, Debug, Hash, Copy, Clone)]
enum Position {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Eq, PartialEq, Debug, Hash, Clone, Copy)]
struct Fence {
    row: usize,
    col: usize,
    position: Position,
}

fn parse_input() -> Vec<Vec<char>> {
    let reader = BufReader::new(File::open("data").unwrap());
    let mut result = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();

        result.push(line.chars().collect());
    }
    result
}

fn solve(map: Vec<Vec<char>>) -> (usize, usize) {
    let mut part1_result = 0;
    let mut part2_result = 0;

    let mut processed = HashSet::new();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let mut perimeter = 0;
            let mut area = 0;
            let mut fences = HashSet::new();
            find_region(
                &map,
                i,
                j,
                &mut processed,
                &mut perimeter,
                &mut area,
                &mut fences,
            );
            part1_result += area * perimeter;
            part2_result += area * count_fences(fences)
        }
    }
    (part1_result, part2_result)
}

fn find_region(
    map: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    processed: &mut HashSet<(usize, usize)>,
    perimeter: &mut usize,
    area: &mut usize,
    fences: &mut HashSet<Fence>,
) {
    if processed.contains(&(i, j)) {
        return;
    }

    processed.insert((i, j));
    *area += 1;
    let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    for d in 0..directions.len() {
        let (di, dj) = directions[d];
        let position = match d {
            0 => Position::Bottom,
            1 => Position::Top,
            2 => Position::Right,
            3 => Position::Left,
            _ => unreachable!(),
        };
        let new_i = add_i32_to_usize(i, di);
        let new_j = add_i32_to_usize(j, dj);

        if new_i.is_none() || new_j.is_none() {
            fences.insert(Fence {
                row: i,
                col: j,
                position,
            });
            *perimeter += 1;
            continue;
        }

        let new_i = new_i.unwrap();
        let new_j = new_j.unwrap();
        if new_i == map.len() || new_j == map[0].len() {
            fences.insert(Fence {
                row: i,
                col: j,
                position,
            });
            *perimeter += 1;
            continue;
        }

        if map[i][j] != map[new_i][new_j] {
            fences.insert(Fence {
                row: i,
                col: j,
                position,
            });
            *perimeter += 1;
            continue;
        }

        find_region(map, new_i, new_j, processed, perimeter, area, fences);
    }
}

fn add_i32_to_usize(u: usize, i: i32) -> Option<usize> {
    if i < 0 {
        u.checked_sub((-i) as usize)
    } else {
        u.checked_add(i as usize)
    }
}

fn count_fences(mut fences: HashSet<Fence>) -> usize {
    let mut result = 0;
    while !fences.is_empty() {
        let first_fence = fences.iter().next().unwrap().clone();
        let mut row = first_fence.row;
        let mut col = first_fence.col;
        let position = first_fence.position;
        loop {
            match position {
                Position::Top | Position::Bottom => col += 1,
                Position::Left | Position::Right => row += 1,
            }
            if !fences.remove(&Fence { row, col, position }) {
                break;
            }
        }
        row = first_fence.row;
        col = first_fence.col;
        loop {
            match position {
                Position::Top | Position::Bottom => {
                    if col == 0 {
                        break;
                    }
                    col -= 1
                }
                Position::Left | Position::Right => {
                    if row == 0 {
                        break;
                    }
                    row -= 1
                }
            }
            if !fences.remove(&Fence { row, col, position }) {
                break;
            }
        }

        fences.remove(&first_fence);
        result += 1;
    }

    result
}

fn main() {
    let map = parse_input();
    let (part1_result, part2_result) = solve(map);
    println!("part1: {}", part1_result);
    println!("part2: {}", part2_result);
}
