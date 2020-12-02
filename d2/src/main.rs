use std::fs::File;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

fn read_file(file_name: &str) -> Vec<String> {
    let mut data = Vec::new();
    let file = File::open(file_name);
    if let Ok(f) = file {
        let reader = io::BufReader::new(f);
        for line_result in reader.lines() {
            if let Ok(line) = line_result {
                data.push(line);
            }
        }
    }

    data
}

#[derive(Debug)]
struct Entry<'a> {
    i1: usize,
    i2: usize,
    ch: &'a str,
    password: &'a str,
}

fn split_line(line: &str) -> Entry {
    let splitted: Vec<&str> = line.split([' ', '-', ':'].as_ref())
        .filter(|e| !e.is_empty())
        .collect();

    Entry {
        i1: usize::from_str(splitted[0]).unwrap(),
        i2: usize::from_str(splitted[1]).unwrap(),
        ch: splitted[2],
        password: splitted[3]
    }
}

fn parse_line_one(line: &str) -> bool {
    let entry = split_line(line);
    let count = entry.password.split("").filter(|e| e == &entry.ch).count();

    count <= entry.i2 && count >= entry.i1
}

fn parse_line_two(line: &str) -> bool {
    let entry = split_line(line);

    let password_chars: Vec<&str> = entry.password.split("").filter(|e| !e.is_empty()).collect();
    (password_chars[entry.i1 - 1] == entry.ch) ^ (password_chars[entry.i2 - 1] == entry.ch)
}

fn main() {
    let data = read_file("input");

    let count1 = data.iter().map(|e| parse_line_one(&e)).filter(|&e| e).count();
    println!("Count for part 1: {}", count1);

    let count2 = data.iter().map(|e| parse_line_two(&e)).filter(|&e| e).count();
    println!("Count for part 2: {}", count2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_line() {
        let splitted = split_line("4-8 g: ggtxgtgbg");
        assert_eq!(splitted.i1, 4);
        assert_eq!(splitted.i2, 8);
        assert_eq!(splitted.ch, "g");
        assert_eq!(splitted.password, "ggtxgtgbg");
    }

    #[test]
    fn test_parse_line_one() {
        assert_eq!(parse_line_one("4-8 g: ggtxgtgbg"), true);
        assert_eq!(parse_line_one("4-8 g: abcde"), false);
    }

    #[test]
    fn test_parse_line_two() {
        assert_eq!(parse_line_two("4-8 g: ggtxgtgbg"), false);
        assert_eq!(parse_line_two("1-3 a: abcde"), true);
        assert_eq!(parse_line_two("1-3 b: cdefg"), false);
        assert_eq!(parse_line_two("2-9 c: ccccccccc"), false);

    }

}