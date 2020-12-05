mod method_two;

use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::iter::FromIterator;
use method_two::method_two;

fn read_file(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let f = File::open(filename)?;

    Ok(io::BufReader::new(f).lines())
}

fn process_line(line: &str) -> (usize, usize) {
    let (mut lower_r, mut upper_r) = (0, 128);
    for ch in line.chars().take_while(|&e| e == 'F' || e == 'B') {
        if ch == 'F' {
            upper_r -= (upper_r - lower_r) / 2;
        } else if ch == 'B' {
            lower_r += (upper_r - lower_r) / 2;
        }
    }

    let (mut lower_c, mut upper_c) = (0, 8);

    for ch in line.chars().skip_while(|&e| e == 'F' || e == 'B') {
        if ch == 'L' {
            upper_c -= (upper_c - lower_c) / 2;
        } else if ch == 'R' {
            lower_c += (upper_c - lower_c) / 2;
        }
    }

    (lower_r, lower_c)
}

fn method_one(lines: impl Iterator<Item=String>) {
    let mut max = 0;
    let mut set: HashSet<usize> = HashSet::from_iter(0..=938);

    for line in lines {
        let (row, col) = process_line(&line);
        let id = row * 8 + col;
        if id > max {
            max = id
        }
        set.remove(&id);
    }

    println!("Max: {}", max);

    let mut sorted = Vec::from_iter(set.iter());
    sorted.sort();
    println!("Missing: {:?}", sorted);
}

fn main() {
    if let Ok(lines) = read_file("input") {
        method_two(lines.map(|e| e.unwrap()));
    }
}
