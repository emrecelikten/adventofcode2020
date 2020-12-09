use std::str::FromStr;

fn load_data(filename: &str) -> Vec<String> {
    std::fs::read_to_string(filename)
        .unwrap()
        .split("\n")
        .filter_map(|e| if e.is_empty() { None } else { Some(e.to_owned()) })
        .collect()
}

fn parse_data<T: AsRef<str>>(data: &[T]) -> Vec<i64> {
    data.iter().map(|e| i64::from_str(e.as_ref()).unwrap()).collect()
}

fn find_invalid(data: &[i64], window_size: usize) -> Vec<i64> {
    let mut window: Vec<i64> = data[0..window_size].to_vec();

    let mut invalid = Vec::new();
    for (pos, &e) in data.iter().skip(window_size).enumerate() {
        let mut valid = false;
        for i in 0..window_size - 1 {
            for j in i + 1..window_size {
                if window[i] != window[j] && window[i] + window[j] == e {
                    valid = true;
                }
            }
        }
        window[pos % window_size] = e;
        if !valid { invalid.push(e); }
    }
    invalid
}

fn find_weakness(data: &[i64], invalid_num: i64) -> i64 {
    let filtered_nums: Vec<i64> = data.into_iter().cloned().filter(|&e| e < invalid_num).collect();

    for i in 2..25 {
        let windows = filtered_nums.windows(i);
        for window in windows {
            let sum: i64 = window.into_iter().sum();
            if sum == invalid_num {
                return window.iter().min().unwrap() + window.iter().max().unwrap()
            }
        }
    }
    panic!("Number not found!");
}

fn main() {
    let data = load_data("input");
    let numbers = parse_data(&data);
    let invalid_nums = find_invalid(&numbers, 25);
    println!("The invalid numbers: {:?}", invalid_nums);

    let weakness = find_weakness(&numbers, invalid_nums[0]);
    println!("Weakness: {}", weakness);
}


#[cfg(test)]
mod tests {
    use super::*;

    static DATA: &'static str = r"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn test_find_invalid() {
        let data: Vec<&str> = DATA.split("\n").collect();
        let vec = parse_data(&data);
        let invalid = find_invalid(&vec, 5);
        assert_eq!(invalid, vec![127]);
    }

    #[test]
    fn test_find_weakness() {
        let data: Vec<&str> = DATA.split("\n").collect();
        let vec = parse_data(&data);
        let invalid = 127;
        let result = find_weakness(&vec, invalid);
        assert_eq!(result, 62);
    }
}