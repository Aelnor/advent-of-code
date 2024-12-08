use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> Vec<Vec<char>> {
    let mut result = Vec::new();

    let reader = BufReader::new(File::open("data").unwrap());
    for line in reader.lines() {
        let line = line.unwrap();

        result.push(line.chars().collect());
    }
    result
}

fn part1(map: &Vec<Vec<char>>) -> usize {
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes = HashSet::new();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] != '.' {
                if let Some(list) = antennas.get(&map[i][j]) {
                    for antenna in list {
                        if let Some((new_i, new_j)) = get_antinode(map, (i, j), *antenna) {
                            antinodes.insert((new_i, new_j));
                        }
                        if let Some((new_i, new_j)) = get_antinode(map, *antenna, (i, j)) {
                            antinodes.insert((new_i, new_j));
                        }
                    }
                }
                antennas.entry(map[i][j]).or_insert(Vec::new()).push((i, j));
            }
        }
    }

    antinodes.len()
}

fn part2(map: &Vec<Vec<char>>) -> usize {
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antinodes = HashSet::new();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] != '.' {
                if let Some(list) = antennas.get(&map[i][j]) {
                    antinodes.insert((i, j));
                    for antenna in list {
                        antinodes.insert(*antenna);
                        let mut left = *antenna;
                        let mut right = (i, j);
                        while let Some((new_i, new_j)) = get_antinode(map, left, right) {
                            antinodes.insert((new_i, new_j));
                            right = left;
                            left = (new_i, new_j);
                        }

                        left = (i, j);
                        right = *antenna;

                        while let Some((new_i, new_j)) = get_antinode(map, left, right) {
                            antinodes.insert((new_i, new_j));
                            right = left;
                            left = (new_i, new_j);
                        }
                    }
                }
                antennas.entry(map[i][j]).or_insert(Vec::new()).push((i, j));
            }
        }
    }

    antinodes.len()
}

fn add_i32_to_usize(u: usize, i: i32) -> Option<usize> {
    if i >= 0 {
        u.checked_add(i as usize)
    } else {
        u.checked_sub((-i) as usize)
    }
}

fn get_antinode(
    map: &Vec<Vec<char>>,
    left: (usize, usize),
    right: (usize, usize),
) -> Option<(usize, usize)> {
    let di = left.0 as i32 - right.0 as i32;
    let dj = left.1 as i32 - right.1 as i32;

    let new_i = add_i32_to_usize(left.0, di);
    let new_j = add_i32_to_usize(left.1, dj);

    if new_i.is_none() || new_j.is_none() {
        return None;
    }

    let new_i = new_i.unwrap();
    let new_j = new_j.unwrap();

    if new_i >= map.len() || new_j >= map[0].len() {
        return None;
    }
    Some((new_i, new_j))
}

fn main() {
    let map = parse_input();
    println!("part 1: {}", part1(&map));
    println!("part 2: {}", part2(&map));
}
