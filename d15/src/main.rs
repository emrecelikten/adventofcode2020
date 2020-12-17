use std::collections::HashMap;
use std::collections::hash_map::Entry::{Vacant, Occupied};
use std::iter::FromIterator;

fn speak_numbers(initial_numbers: &[usize], last_index: usize) -> usize {
    let mut num_history: HashMap<usize, usize> = HashMap::new();
    for (idx, num) in initial_numbers.iter().enumerate() {
        num_history.insert(*num, idx + 1);
    }

    let mut last_num = 0;
    let mut idx = initial_numbers.len() + 1;
    loop {
        idx += 1;
        if idx == last_index + 1 { break; }
        let val = match num_history.entry(last_num) {
            Vacant(entry) => {
                entry.insert(idx - 1);
                0
            },
            Occupied(entry) => {
                let v = entry.into_mut();
                let c = *v;
                *v = idx - 1;
                c
            },
        };
        if val != 0 { last_num = idx - 1 - val; }
        else { last_num = val; }
    }
    last_num
}

fn main() {
    let data = vec![0,1,5,10,3,12,19];
    let result_one = speak_numbers(&data, 2020);
    println!("Result #1: {}", result_one);
    let now = std::time::Instant::now();
    let result_two = speak_numbers(&data, 30000000);
    println!("Result #2: {}", result_two);
    println!("Part 2 completed in {:?}.", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test0 = vec![0, 3, 6];
        assert_eq!(speak_numbers(&test0, 2020), 436);

        let test1 = vec![1,3,2];
        assert_eq!(speak_numbers(&test1, 2020), 1);

        let test2 = vec![2,1,3];
        assert_eq!(speak_numbers(&test2, 2020), 10);

        let test3 = vec![1,2,3];
        assert_eq!(speak_numbers(&test3, 2020), 27);

        let test4 = vec![3,1,2];
        assert_eq!(speak_numbers(&test4, 2020), 1836);
    }
}