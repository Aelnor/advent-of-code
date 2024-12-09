fn parse_input() -> Vec<Option<usize>> {
    let mut result = Vec::new();

    let s = std::fs::read_to_string("data").unwrap();

    result.reserve(s.len() * 10);

    let mut is_block = true;
    let mut file_id = 0;
    for c in s.chars() {
        let n = c.to_digit(10);
        if n.is_none() {
            break;
        }
        let n = n.unwrap();
        if is_block {
            for _ in 0..n {
                result.push(Some(file_id));
            }
            file_id += 1;
        } else {
            for _ in 0..n {
                result.push(None);
            }
        }
        is_block = !is_block;
    }

    result
}

fn part1(data: &mut Vec<Option<usize>>) -> usize {
    compact(data);
    calc_checksum(data)
}

fn part2(data: &mut Vec<Option<usize>>) -> usize {
    compact_no_fragmentation(data);
    calc_checksum(data)
}

fn compact(data: &mut Vec<Option<usize>>) {
    let mut left = 0;
    let mut right = data.len() - 1;

    while left < right {
        while data[right].is_none() && left < right {
            right -= 1;
        }
        while data[left].is_some() && left < right {
            left += 1;
        }
        if right <= left {
            break;
        }
        data[left] = data[right];
        data[right] = None;

        /*
        for i in 0..data.len() {
            if let Some(id) = data[i] {
                print!("{}", id);
            } else {
                print!(".")
            }
        }
        println!("");
        */
    }
}

fn compact_no_fragmentation(data: &mut Vec<Option<usize>>) {
    let mut last_start = data.len() - 1;
    loop {
        let mut right = last_start;
        while right != 0 && data[right].is_none() {
            right -= 1;
        }
        if right == 0 {
            break;
        }
        let file_id = data[right].unwrap();
        let mut chunk_len = 0;
        while right != 0 && data[right] == Some(file_id) {
            right -= 1;
            chunk_len += 1;
        }
        if right == 0 {
            break;
        }

        last_start = right;
        right += 1;
        let mut left = 0;

        loop {
            let space = find_empty_chunk(data, left);
            if space.is_none() {
                break;
            }
            let (empty_start, empty_chunk_len) = space.unwrap();

            left = empty_start + empty_chunk_len;

            if empty_chunk_len < chunk_len {
                continue;
            }

            if empty_start > right {
                break;
            }

            move_chunk(data, right, empty_start);
            break;
        }
    }
}

fn find_empty_chunk(data: &Vec<Option<usize>>, start: usize) -> Option<(usize, usize)> {
    let mut left = start;
    while left < data.len() && data[left].is_some() {
        left += 1;
    }
    if left == data.len() - 1 {
        return None;
    }
    let mut chunk_len = 0;
    let empty_start = left;
    while left < data.len() && data[left].is_none() {
        chunk_len += 1;
        left += 1;
    }
    if chunk_len == 0 {
        return None;
    }
    Some((empty_start, chunk_len))
}

fn move_chunk(data: &mut Vec<Option<usize>>, source: usize, destination: usize) {
    assert!(data[source].is_some());

    let id = data[source];

    let mut s = source;
    let mut d = destination;

    while s < data.len() && data[s] == id {
        data[d] = data[s];
        data[s] = None;
        s += 1;
        d += 1;
    }
}

fn calc_checksum(data: &Vec<Option<usize>>) -> usize {
    let mut result = 0;
    for i in 0..data.len() {
        if let Some(id) = data[i] {
            result += i * id;
        }
    }
    result
}

fn main() {
    let mut data = parse_input();
    println!("part 1: {}", part1(&mut data));
    data = parse_input();
    println!("part 2: {}", part2(&mut data));
}
