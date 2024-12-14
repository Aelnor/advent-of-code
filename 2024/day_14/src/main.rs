use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Robot {
    position: Point,
    x_speed: isize,
    y_speed: isize,
}

fn parse_input() -> Vec<Robot> {
    let reader = BufReader::new(File::open("data").unwrap());
    let mut result = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
        let caps = re.captures(&line).unwrap();
        result.push(Robot {
            position: Point {
                x: caps[1].parse::<usize>().unwrap(),
                y: caps[2].parse::<usize>().unwrap(),
            },
            x_speed: caps[3].parse::<isize>().unwrap(),
            y_speed: caps[4].parse::<isize>().unwrap(),
        });
    }

    result
}
const ARENA_WIDTH: usize = 101;
const ARENA_HEIGHT: usize = 103;

fn part1(robots: &Vec<Robot>) -> usize {
    let mut robots = robots.clone();

    for _ in 0..100 {
        for r in 0..robots.len() {
            move_robot(&mut robots[r]);
        }
    }

    calc_safety_factor(&robots)
}

fn part2(robots: &Vec<Robot>) {
    let mut robots = robots.clone();
    let mut step = 0;
    let mut seen = HashSet::new();

    loop {
        for r in 0..robots.len() {
            move_robot(&mut robots[r]);
        }
        // the way I did it:
        // 1. Push everything to a file
        // 2. Use text tools to search for irregularities. I thought the process was going to be
        //    complicated, but in reality the first search for six to seven ones in a row gave me
        //    the answer
        // 3. If I knew what I was looking at, I could do it programmatically (like looking for
        //    excessive about of robots in a row)
        step += 1;
        println!("step: {step}");
        print_robots(&robots);
        if !seen.insert(robots.clone()) {
            println!("total cycles: {step}");
            break;
        }
    }
}

fn move_robot(robot: &mut Robot) {
    let mut new_x = robot.position.x as isize + robot.x_speed;
    if new_x < 0 {
        new_x += ARENA_WIDTH as isize;
    }

    if new_x >= ARENA_WIDTH as isize {
        new_x -= ARENA_WIDTH as isize;
    }

    let mut new_y = robot.position.y as isize + robot.y_speed;
    if new_y < 0 {
        new_y += ARENA_HEIGHT as isize;
    }

    if new_y >= ARENA_HEIGHT as isize {
        new_y -= ARENA_HEIGHT as isize;
    }

    robot.position.x = new_x as usize;
    robot.position.y = new_y as usize;
}

fn calc_safety_factor(robots: &Vec<Robot>) -> usize {
    let mid_x = ARENA_WIDTH / 2;
    let mid_y = ARENA_HEIGHT / 2;
    let mut qs = vec![0; 4];
    for r in robots {
        let x = r.position.x;
        let y = r.position.y;
        if x == mid_x || y == mid_y {
            continue;
        }
        if x < mid_x && y < mid_y {
            qs[0] += 1;
        }
        if x > mid_x && y < mid_y {
            qs[1] += 1;
        }
        if x < mid_x && y > mid_y {
            qs[2] += 1;
        }
        if x > mid_x && y > mid_y {
            qs[3] += 1;
        }
    }
    qs.into_iter().product()
}

fn print_robots(robots: &Vec<Robot>) {
    for y in 0..ARENA_HEIGHT {
        for x in 0..ARENA_WIDTH {
            let mut n = 0;
            for r in robots {
                if r.position.x == x && r.position.y == y {
                    n += 1;
                }
            }
            if n >= 10 {
                unreachable!();
            }
            if n == 0 {
                print!(".");
            } else {
                print!("{n}");
            }
        }
        println!("");
    }
}

fn main() {
    let robots = parse_input();
    println!("part 1: {}", part1(&robots));
    part2(&robots);
}
