use std::collections::HashMap;

// Second attempt using hash maps. VecDeque lookups are not fast enough.
const LABELING: [i32; 9] = [2, 1, 9, 7, 4, 8, 3, 6, 5];

type CupMap = HashMap<i32, i32>;

fn make_map(initial_numbers: &[i32], total_cups: usize) -> CupMap {
    let mut map = CupMap::new();

    for (idx, number) in initial_numbers.iter().enumerate() {
        map.insert(*number, initial_numbers[(idx + 1) % initial_numbers.len()]);
    }

    if total_cups > initial_numbers.len() {
        map.insert(initial_numbers[initial_numbers.len() - 1], initial_numbers.len() as i32 + 1);

        for i in initial_numbers.len() as i32 + 1..total_cups as i32 {
            map.insert(i, i + 1);
        }

        map.insert(total_cups as i32, initial_numbers[0]);
    }
    map
}

fn pick_cups(cups: &mut CupMap, current_cup: i32) -> [i32; 3] {
    let mut triple = [0; 3];
    let mut next = cups.remove(&current_cup).unwrap();
    for i in 0..3 {
        triple[i] = next;
        if !cups.contains_key(&next) { dbg!(&current_cup, &next); }
        next = cups.remove(&next).unwrap();
    }
    cups.insert(current_cup, next);
    triple
}

fn select_destination_cup(cups: &CupMap, picked_cups: &[i32; 3], current_cup: i32) -> i32 {
    // Hack for speedup
    let len = (cups.len() + picked_cups.len()) as i32;
    let mut max = len;
    let mut min = 1;

    for _ in 0..picked_cups.len() {
        if picked_cups.contains(&max) { max -= 1; };
        if picked_cups.contains(&min) { min += 1; };
    }

    let mut target_cup = current_cup - 1;
    if target_cup < min {
        target_cup = max;
    }

    while picked_cups.contains(&target_cup) {
        target_cup -= 1;
        if target_cup < min {
            target_cup = max;
        }
    }

    target_cup
}

fn place_cups(cups: &mut CupMap, dest_cup: i32, picked_cups: &[i32; 3]) {
    let next = cups.remove(&dest_cup).unwrap();
    cups.insert(dest_cup, picked_cups[0]);
    for i in 0..2 {
        cups.insert(picked_cups[i], picked_cups[i + 1]);
    }
    cups.insert(picked_cups[2], next);
}


fn select_new_cup(cups: &CupMap, current_cup: &mut i32) {
    *current_cup = *cups.get(current_cup).unwrap();
}

#[allow(unused_macros)]
macro_rules! bench {
    ($input:expr) => {{
        let time = std::time::Instant::now();
        let x = $input;
        let elapsed = time.elapsed();
        println!("{}: {:?}", stringify!($input), elapsed);
        x
    }}
}

fn iterate(cups: &mut CupMap, current_cup: &mut i32, times: usize) {
    for _i in 0..times {
        // dbg!(&current_cup, &cups);
        let picked = pick_cups(cups, *current_cup);
        let dest = select_destination_cup(cups, &picked, *current_cup);
        place_cups(cups, dest, &picked);
        select_new_cup(cups, current_cup);
    }
}

fn convert_to_vec(cups: &CupMap) -> Vec<i32> {
    let mut result = Vec::new();
    let mut elem = cups.iter().next().unwrap().0;
    for _ in 0..cups.len() {
        result.push(*elem);
        elem = cups.get(elem).unwrap();
    }
    let one_pos = result.iter().position(|&c| c == 1).unwrap();
    result.rotate_left(one_pos);
    result
}

fn main() {
    let mut cups = make_map(&LABELING, LABELING.len());
    let mut current_cup = LABELING[0];
    iterate(&mut cups, &mut current_cup, 100);

    let vec = convert_to_vec(&mut cups);
    let str: String = vec.iter()
        .skip(1)
        .take(cups.len() - 1)
        .map(|x| std::char::from_digit(*x as u32, 10).unwrap())
        .collect();
    println!("Result #1: {}", str);

    let mut cups_big = make_map(&LABELING, 1000000);
    let mut current_cup = LABELING[0];
    iterate(&mut cups_big, &mut current_cup, 10000000);
    let m1 = cups_big.get(&1).unwrap();
    let m2 = cups_big.get(m1).unwrap();

    println!("Result #2: {}", *m1 as i64 * *m2 as i64);
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_LABELING: [i32; 9] = [3, 8, 9, 1, 2, 5, 4, 6, 7];

    #[test]
    fn test_pick_cups() {
        let mut cups = make_map(&TEST_LABELING, TEST_LABELING.len());
        let mut current_cup = 3;
        let picked1 = pick_cups(&mut cups, current_cup);

        assert_eq!(
            vec![8, 9, 1],
            picked1
        );

        assert_eq!(
            2,
            *cups.get(&current_cup).unwrap()
        );

        assert_eq!(
            false,
            cups.contains_key(&9)
        );

        cups = make_map(&TEST_LABELING, TEST_LABELING.len());
        current_cup = 6;
        let picked2 = pick_cups(&mut cups, current_cup);

        assert_eq!(
            vec![7, 3, 8],
            picked2
        );

        assert_eq!(
            6,
            cups.len()
        );
    }

    #[test]
    fn test_select_destination_cup() {
        let mut cups = make_map(&TEST_LABELING, TEST_LABELING.len());
        let current_cup = 3;

        let picked = pick_cups(&mut cups, current_cup);
        let dest = select_destination_cup(&cups, &picked, current_cup);

        assert_eq!(
            2,
            dest
        );
    }

    #[test]
    fn test_place_cups() {
        let mut cups = make_map(&TEST_LABELING, TEST_LABELING.len());
        let current_cup = 9;
        let picked = pick_cups(&mut cups, current_cup);

        place_cups(&mut cups, 6, &picked);

        assert_eq!(
            1,
            *cups.get(&6).unwrap()
        );
        assert_eq!(
            2,
            *cups.get(&1).unwrap()
        );
        assert_eq!(
            5,
            *cups.get(&2).unwrap()
        );
        assert_eq!(
            7,
            *cups.get(&5).unwrap()
        );
    }

    #[test]
    fn test_select_new_cup_pos() {
        let cups = make_map(&TEST_LABELING, TEST_LABELING.len());
        let mut current_cup = 1;
        select_new_cup(&cups, &mut current_cup);

        assert_eq!(
            2,
            current_cup
        );
    }

    #[test]
    fn test_iterate() {
        let mut cups = make_map(&TEST_LABELING, TEST_LABELING.len());
        let mut current_cup = 3;

        iterate(&mut cups, &mut current_cup, 10);

        assert_eq!(
            vec![1, 9, 2, 6, 5, 8, 3, 7, 4],
            convert_to_vec(&cups)
        );
    }

    #[test]
    fn test_part_two() {
        let mut cups = make_map(&TEST_LABELING, 1000000);
        let mut current_cup = 3;

        assert_eq!(
            1000000,
            cups.len()
        );

        assert_eq!(
            10,
            *cups.get(&7).unwrap()
        );

        assert_eq!(
            3,
            *cups.get(&1000000).unwrap()
        );

        assert_eq!(
            true,
            cups.contains_key(&10)
        );

        bench!(iterate(&mut cups, &mut current_cup, 10000000));
        let m1 = cups.get(&1).unwrap();
        let m2 = cups.get(m1).unwrap();

        assert_eq!(
            934001,
            *m1
        );

        assert_eq!(
            159792,
            *m2
        );
    }
}