use regex::Regex;

fn part1() -> usize {
    let contents = std::fs::read_to_string("data").unwrap();
    calc(&contents)
}

fn part2() -> usize {
    let contents = std::fs::read_to_string("data").unwrap();
    let mut result = 0;

    let mut current: &str = &contents;

    while let Some(index) = current.find("don't()") {
        let (chunk, rest) = current.split_at(index);
        result += calc(chunk);

        let index = rest.find("do()");
        if index.is_none() {
            return result;
        }
        current = &rest[index.unwrap()..];
    }
    result += calc(&current);

    result
}

fn calc(s: &str) -> usize {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut result = 0;

    for cap in re.captures_iter(s) {
        let x = cap[1].parse::<usize>().unwrap();
        let y = cap[2].parse::<usize>().unwrap();
        result += x * y;
    }

    result
}

fn main() {
    println!("part1: {}", part1());
    println!("part2: {}", part2());
}
