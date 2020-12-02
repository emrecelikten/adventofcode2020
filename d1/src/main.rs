use std::fs::File;
use std::collections::HashSet;
use std::str::FromStr;
use std::io::{self, BufRead};
use std::iter::FromIterator;

fn read_data(filename: &str) -> Vec<i32> {
    let mut data = Vec::new();

    let file = File::open(filename);
    let reader = file.map(|e| io::BufReader::new(e).lines());
    if let Ok(lines) = reader {
        for line in lines {
            if let Ok(e) = line {
                data.push(i32::from_str(&e).unwrap());
            }
        }
    }

    data
}

fn construct_set(data: &[i32]) -> HashSet<i32> {
    return HashSet::from_iter(data.into_iter().cloned())
}

fn find_double(set: &HashSet<i32>) {
    for i in set {
        if let Some(result) = set.get(&(&2020 - i)) {
            println!("{} * {} = {}", i, result, i * result);
            return;
        }
    }
}

fn find_triple_brute(set: &HashSet<i32>) {
    for i in set {
        for j in set {
            if i == j { continue; }
            if let Some(result) = set.get(&(&2020 - i - j)) {
                println!("{} * {} * {} = {}", i, j, result, i * j * result);
                return;
            }
        }
    }
}

fn main() {
    let data = read_data("./data.txt");
    let set = construct_set(&data);

    find_double(&set);

    // TODO: We can probably do better than this, which is O(n^2)
    find_triple_brute(&set);
}
