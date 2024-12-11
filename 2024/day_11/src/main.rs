use std::collections::HashMap;

fn parse_input() -> Vec<usize> {
    std::fs::read_to_string("data")
        .unwrap()
        .split(' ')
        .map(|e| e.trim().parse::<usize>().unwrap())
        .collect()
}

fn part1(stones: &Vec<usize>) -> usize {
    solve(stones, 25)
}

fn part2(stones: &Vec<usize>) -> usize {
    solve(stones, 75)
}

fn solve(stones: &Vec<usize>, blinks: usize) -> usize {
    let mut state = HashMap::new();
    for stone in stones {
        *state.entry(*stone).or_insert(0) += 1;
    }

    for _ in 0..blinks {
        let mut new_state = HashMap::new();
        for (number, count) in state {
            if number == 0 {
                *new_state.entry(1 as usize).or_insert(0) += count;
                continue;
            }
            let number_str = number.to_string();
            if number_str.len() % 2 == 0 {
                let (first_half, second_half) = number_str.split_at(number_str.len() / 2);
                *new_state
                    .entry(first_half.parse::<usize>().unwrap())
                    .or_insert(0) += count;
                *new_state
                    .entry(second_half.parse::<usize>().unwrap())
                    .or_insert(0) += count;
                continue;
            }

            *new_state.entry(number * 2024).or_insert(0) += count;
        }
        state = new_state;
    }
    state.values().sum()
}

fn main() {
    let stones = parse_input();
    println!("part 1: {}", part1(&stones));
    println!("part 2: {}", part2(&stones));
}
