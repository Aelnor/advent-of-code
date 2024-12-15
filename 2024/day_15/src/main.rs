use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> (Vec<Vec<char>>, Vec<Vec<char>>, (usize, usize)) {
    let mut map = Vec::new();
    let mut commands = Vec::new();
    let mut row = 0;
    let mut col = 0;

    let reader = BufReader::new(File::open("data").unwrap());
    let mut map_parsed = false;
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.is_empty() {
            map_parsed = true;
            continue;
        }

        if !map_parsed {
            if let Some(index) = line.chars().position(|e| e == '@') {
                let mut chars: Vec<_> = line.chars().collect();
                chars[index] = '.';
                row = i;
                col = index;
                map.push(chars);
                continue;
            }
            map.push(line.chars().collect());
            continue;
        }

        commands.push(line.chars().collect());
    }
    (map, commands, (row, col))
}

fn part1(map: &Vec<Vec<char>>, commands: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let mut map = map.clone();

    let mut row = row;
    let mut col = col;

    for command in commands.iter().flatten() {
        let (drow, dcol) = get_direction(*command);
        (row, col) = one_dimentional_move(&mut map, row, col, drow, dcol);
    }
    calc_score(&map)
}

fn part2(map: &Vec<Vec<char>>, commands: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let mut map = map.clone();

    let mut row = row;
    let mut col = col;

    for command in commands.iter().flatten() {
        let mut can_move = true;

        let (drow, dcol) = get_direction(*command);

        if drow == 0 {
            (row, col) = one_dimentional_move(&mut map, row, col, drow, dcol);
            continue;
        }

        // that's a little bit dirty, but it is what it is
        // simple case 1: empty space in the way. Move the robot and forget
        let mut r = add_i32_to_usize(row, drow).unwrap();
        if map[r][col] == '.' {
            row = r;
            continue;
        }

        // simple(r) case 2: wain the way. Forget.
        if map[r][col] == '#' {
            continue;
        }

        // case 3: there's a box there, we have to do the job
        let mut state = Vec::new();
        match map[r][col] {
            '[' => state.push(HashSet::from([col])),
            ']' => state.push(HashSet::from([col - 1])),
            _ => unreachable!(),
        }

        loop {
            let prev_line = state.last().unwrap();
            let mut new_line = HashSet::new();
            r = add_i32_to_usize(r, drow).unwrap();

            for b in prev_line {
                match map[r][*b] {
                    '[' => {
                        new_line.insert(*b);
                    }
                    ']' => {
                        new_line.insert(*b - 1);
                    }
                    '#' => {
                        can_move = false;
                        break;
                    }
                    '.' => {}
                    _ => unreachable!(),
                };
                match map[r][*b + 1] {
                    '[' => {
                        new_line.insert(*b + 1);
                    }
                    ']' => {
                        // this is handled by the '[' in the previous match: two boxes are exactly on
                        // top of each other
                    }
                    '#' => {
                        can_move = false;
                        break;
                    }
                    '.' => {}
                    _ => unreachable!(),
                }
            }

            if !can_move || new_line.is_empty() {
                break;
            }

            state.push(new_line);
        }

        if !can_move {
            continue;
        }

        // Okay, we have all the boxes to move and their indexes. Let's go.
        while let Some(line) = state.pop() {
            let nextr = add_i32_to_usize(r, -drow).unwrap();
            for b in line {
                map[r][b] = map[nextr][b];
                map[nextr][b] = '.';
                map[r][b + 1] = map[nextr][b + 1];
                map[nextr][b + 1] = '.';
            }
            r = nextr;
        }
        row = r;
    }

    calc_score(&map)
}

fn one_dimentional_move(
    map: &mut Vec<Vec<char>>,
    row: usize,
    col: usize,
    drow: i32,
    dcol: i32,
) -> (usize, usize) {
    let mut can_move = true;
    let mut r = row;
    let mut c = col;
    loop {
        r = add_i32_to_usize(r, drow).unwrap();
        c = add_i32_to_usize(c, dcol).unwrap();

        if map[r][c] == '#' {
            can_move = false;
            break;
        }
        if map[r][c] == '.' {
            break;
        }
    }

    if !can_move {
        return (row, col);
    }

    loop {
        let nextr = add_i32_to_usize(r, -drow).unwrap();
        let nextc = add_i32_to_usize(c, -dcol).unwrap();
        map[r][c] = map[nextr][nextc];
        if nextr == row && nextc == col {
            break;
        }
        r = nextr;
        c = nextc;
    }
    (r, c)
}

fn calc_score(map: &Vec<Vec<char>>) -> usize {
    let mut result = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'O' || map[i][j] == '[' {
                result += 100 * i + j;
            }
        }
    }
    result
}

fn get_direction(c: char) -> (i32, i32) {
    match c {
        '>' => (0, 1),
        '<' => (0, -1),
        '^' => (-1, 0),
        'v' => (1, 0),
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

fn convert_map(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = Vec::new();
    for i in 0..map.len() {
        let mut line = Vec::new();
        for j in 0..map[i].len() {
            match map[i][j] {
                '#' => {
                    line.push('#');
                    line.push('#');
                }
                'O' => {
                    line.push('[');
                    line.push(']');
                }
                '.' => {
                    line.push('.');
                    line.push('.');
                }
                _ => unreachable!(),
            }
        }
        result.push(line);
    }
    result
}

fn main() {
    let (map, commands, (row, col)) = parse_input();
    println!("part1: {}", part1(&map, &commands, row, col));
    let new_map = convert_map(&map);
    let new_col = col * 2;
    println!("part2: {}", part2(&new_map, &commands, row, new_col));
}
