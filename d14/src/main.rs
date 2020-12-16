#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, PartialEq)]
enum Command {
    Mask { data: String },
    Mem { addr: usize, num: usize },
}

fn read_data(filename: &str) -> String {
    std::fs::read_to_string(filename).unwrap()
}

fn parse_data(data: &str) -> Vec<Command> {
    lazy_static! {
        static ref MEMREGEX: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
        static ref MASKREGEX: Regex = Regex::new(r"mask = (.+)").unwrap();
    }

    data.lines().map(|line| {
        if line.starts_with("mask") {
            let cap = MASKREGEX.captures(line).unwrap();
            Command::Mask { data: cap[1].to_owned() }
        } else if line.starts_with("mem") {
            let cap = MEMREGEX.captures(line).unwrap();
            Command::Mem { addr: cap[1].parse().unwrap(), num: cap[2].parse().unwrap() }
        } else {
            panic!("Unknown line!");
        }
    }).collect()
}

fn mask_data(num: usize, mask: &str) -> usize {
    let mut result = num;
    for (i, ch) in mask.chars().enumerate() {
        if ch == '1' {
            result |= 1 << (35 - i)
        } else if ch == '0' {
            result &= !(1 << (35 - i))
        }
    }

    result
}

fn mask_address(address: usize, mask: &str) -> Vec<usize> {
    let mut result: Vec<usize> = vec![address];

    for (i, ch) in mask.chars().enumerate() {
        if ch == 'X' {
            let mut temp = Vec::new();
            for e in result {
                temp.push(e | 1 << (35 - i));
                temp.push(e & !(1 << (35 - i)));
            }
            result = temp;
        }
        if ch == '1' {
            for e in &mut result {
                *e |= 1 << (35 - i);
            }
        }
    }
    result
}


fn eval_commands_one(data: &[Command]) -> usize {
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut cur_mask = "";
    for command in data {
        match command {
            Command::Mask { data: mask } => cur_mask = mask,
            Command::Mem { addr, num } => { map.insert(*addr, mask_data(*num, cur_mask)); }
        }
    }

    map.values().sum()
}

fn eval_commands_two(data: &[Command]) -> usize {
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut cur_mask: &str = "";
    for command in data {
        match command {
            Command::Mask { data: mask } => cur_mask = mask,
            Command::Mem { addr, num } => {
                for masked_addr in mask_address(*addr, cur_mask) {
                    map.insert(masked_addr, *num);
                }
            }
        }
    }

    map.values().sum()
}

fn main() {
    let data = parse_data(&read_data("input"));
    let result_one = eval_commands_one(&data);
    println!("Result #1: {}", result_one);
    let result_two = eval_commands_two(&data);
    // let elapsed = now.elapsed();
    println!("Result #2: {}", result_two);
    // dbg!(elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &'static str = r"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    #[test]
    fn test_parse_data() {
        let result = parse_data(&TEST_DATA);

        assert_eq!(result[0], Command::Mask { data: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string() });
        assert_eq!(result[2], Command::Mem { addr: 7, num: 101 });
    }

    #[test]
    fn test_mask_data() {
        let m = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
        let result1 = mask_data(11, m);
        let result2 = mask_data(101, m);
        let result3 = mask_data(0, m);
        assert_eq!(result1, 73);
        assert_eq!(result2, 101);
        assert_eq!(result3, 64);
    }

    #[test]
    fn test_mask_address() {
        let m = "000000000000000000000000000000X1001X";
        let addr = 42;
        let result = mask_address(addr, m);
        assert_eq!(result.len(), 4);
        dbg!(&result);
        assert_eq!(result.contains(&26), true);
        assert_eq!(result.contains(&27), true);
        assert_eq!(result.contains(&58), true);
        assert_eq!(result.contains(&59), true);
    }
}