use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> Vec<Vec<usize>> {
    let file = File::open("data").unwrap();
    let reader = BufReader::new(file);

    let mut records = Vec::new();

    for line in reader.lines() {
        let record = line
            .unwrap()
            .split(' ')
            .map(|e| e.parse::<usize>().unwrap())
            .collect();
        records.push(record);
    }

    records
}

fn solve(records: &Vec<Vec<usize>>, dampener: bool) -> usize {
    let mut result = 0;
    for r in records {
        let (safe, error) = is_safe(r);
        if safe {
            result += 1;
            continue;
        }
        if !dampener {
            continue;
        }

        let start = if error < 2 { 0 } else { error - 2 };
        for i in start..start + 3 {
            let mut new_record = r.clone();
            new_record.remove(i);
            let (safe, _) = is_safe(&new_record);
            if safe {
                result += 1;
                break;
            }
        }
    }

    result
}

fn is_safe(record: &Vec<usize>) -> (bool, usize) {
    let mut prev = 0;

    for i in 0..record.len() - 1 {
        let (o, d) = delta(record[i], record[i + 1]);
        if d > 3 || d < 1 {
            return (false, i + 1);
        }
        if prev == 0 {
            prev = o;
            continue;
        }
        if prev != o {
            return (false, i + 1);
        }
    }
    (true, 0)
}

fn delta(l: usize, r: usize) -> (i8, usize) {
    if l > r {
        (1, l - r)
    } else {
        (-1, r - l)
    }
}

fn main() {
    let records = parse_input();
    println!("p1: {}", solve(&records, false));
    println!("p2: {}", solve(&records, true));
}
