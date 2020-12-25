use std::collections::VecDeque;
use std::iter::FromIterator;

const LABELING: [i32; 9] = [2, 1, 9, 7, 4, 8, 3, 6, 5];

fn make_deque(numbers: &[i32]) -> VecDeque<i32> {
    VecDeque::from_iter(numbers.iter().cloned())
}

fn make_deque_part_two(numbers: &[i32]) -> VecDeque<i32> {
    let cups = 1000000;
    let mut result = VecDeque::with_capacity(cups);
    result.extend(numbers);
    for i in numbers.len() + 1..=cups {
        result.push_back(i as i32);
    }
    result
}

fn pick_cups(cups: &mut VecDeque<i32>, current_cup_pos: &mut usize) -> Vec<i32> {
    let cups_to_remove: Vec<usize> = (1..=3_usize)
        .map(|idx| (*current_cup_pos + idx) % cups.len())
        .collect();

    let mut removed = Vec::new();
    let mut removed_idx = Vec::new();
    for cup_pos in cups_to_remove {
        let before_count = removed_idx.iter()
            .filter(|&&idx| idx < cup_pos)
            .count();
        let cup = cups.remove(cup_pos - before_count).unwrap();
        removed.push(cup);
        removed_idx.push(cup_pos);
        if cup_pos < *current_cup_pos {
            *current_cup_pos -= 1;
        }
    }

    cups.make_contiguous();
    removed
}

fn select_destination_cup(cups: &VecDeque<i32>, picked_cups: &Vec<i32>, current_cup_pos: &usize) -> usize {
    let cur_label = cups[*current_cup_pos];
    // Hack for speedup
    let len = (cups.len() + picked_cups.len()) as i32;
    let mut max = len;
    let mut min = 1;

    for i in 0..picked_cups.len() {
        if picked_cups.contains(&max) { max -= 1; };
        if picked_cups.contains(&min) { min += 1; };
    }

    let mut target_label = cur_label - 1;
    if target_label < min {
        target_label = max;
    }

    while picked_cups.contains(&target_label) {
        target_label -= 1;
        if target_label < min {
            target_label = max;
        }
    };

    // dbg!(&cups, &picked_cups, &cups[*current_cup_pos], target_label);
    // Use rposition since many of the numbers we are looking for is near the end of the list.
    cups.as_slices().0.iter().rposition(|&x| x == target_label).unwrap()
}

fn place_cups(cups: &mut VecDeque<i32>, current_cup_pos: &mut usize, dest_cup_pos: usize, picked_cups: &Vec<i32>) {
    for (idx, cup) in picked_cups.iter().enumerate() {
        cups.insert(dest_cup_pos + 1 + idx, *cup);
    }
    if dest_cup_pos < *current_cup_pos {
        *current_cup_pos += picked_cups.len();
    }
}

fn select_new_cup_pos(cups: &VecDeque<i32>, current_cup_pos: &mut usize) {
    *current_cup_pos = (*current_cup_pos + 1) % cups.len()
}

fn iterate(cups: &mut VecDeque<i32>, current_cup_pos: &mut usize, times: usize) {
    for _i in 0..times {
        if _i % 5000 == 0 {
            dbg!(&_i);
        }
        // let time = std::time::Instant::now();
        let picked = pick_cups(cups, current_cup_pos);
        // let mut elapsed = time.elapsed();
        // println!("Picked: {:?}", elapsed);

        // let time = std::time::Instant::now();
        let dest = select_destination_cup(cups, &picked, current_cup_pos);
        // let mut elapsed = time.elapsed();
        // println!("Select dest: {:?}", elapsed);

        // let time = std::time::Instant::now();
        place_cups(cups, current_cup_pos, dest, &picked);
        // let mut elapsed = time.elapsed();
        // println!("Place: {:?}", elapsed);

        // let time = std::time::Instant::now();
        select_new_cup_pos(cups, current_cup_pos);
        // let mut elapsed = time.elapsed();
        // println!("Select new: {:?}", elapsed);
    }
}

fn rotate_to_one(cups: &mut VecDeque<i32>) {
    let one_pos = cups.iter().position(|&x| x == 1).unwrap();
    cups.rotate_left(one_pos);
}

fn main() {
    let mut cups = make_deque(&LABELING);
    let mut current_cup_pos = 0;
    iterate(&mut cups, &mut current_cup_pos, 100);

    rotate_to_one(&mut cups);
    let str: String = cups.iter()
        .skip(1)
        .take(cups.len() - 1)
        .map(|x| std::char::from_digit(*x as u32, 10).unwrap())
        .collect();

    println!("Result #1: {}", str);
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_LABELING: [i32; 9] = [3, 8, 9, 1, 2, 5, 4, 6, 7];

    #[test]
    fn test_pick_cups() {
        let mut cups = make_deque(&TEST_LABELING);
        let mut current_cup_pos = 0;
        let picked1 = pick_cups(&mut cups, &mut current_cup_pos);

        assert_eq!(
            vec![8, 9, 1],
            picked1
        );

        assert_eq!(
            current_cup_pos,
            0
        );

        cups = make_deque(&TEST_LABELING);
        current_cup_pos = 7;
        let picked2 = pick_cups(&mut cups, &mut current_cup_pos);

        assert_eq!(
            vec![7, 3, 8],
            picked2
        );

        assert_eq!(
            current_cup_pos,
            5
        );

        assert_eq!(
            VecDeque::from(vec![9, 1, 2, 5, 4, 6]),
            cups
        );
    }

    #[test]
    fn test_select_destination_cup() {
        let mut cups = make_deque(&TEST_LABELING);
        let mut current_cup_pos = 0;
        let picked = pick_cups(&mut cups, &mut current_cup_pos);

        let dest_pos = select_destination_cup(&cups, &picked, &current_cup_pos);

        assert_eq!(
            2,
            cups[dest_pos]
        );
    }

    #[test]
    fn test_place_cups() {
        let mut cups = make_deque(&TEST_LABELING);
        let mut current_cup_pos = 4;
        let picked = pick_cups(&mut cups, &mut current_cup_pos);

        let dest_pos = 0;

        place_cups(&mut cups, &mut current_cup_pos, dest_pos, &picked);

        assert_eq!(
            VecDeque::from(vec![3, 5, 4, 6, 8, 9, 1, 2, 7]),
            cups
        );
    }

    #[test]
    fn test_select_new_cup_pos() {
        let cups = make_deque(&TEST_LABELING);
        let mut current_cup_pos = 8;
        select_new_cup_pos(&cups, &mut current_cup_pos);

        assert_eq!(
            0,
            current_cup_pos
        );
    }

    #[test]
    fn test_iterate() {
        let mut cups = make_deque(&TEST_LABELING);
        let mut current_cup_pos = 0;

        iterate(&mut cups, &mut current_cup_pos, 10);

        assert_eq!(
            VecDeque::from(vec![5, 8, 3, 7, 4, 1, 9, 2, 6]),
            cups
        );

        assert_eq!(
            1,
            current_cup_pos
        );
    }

    #[test]
    fn test_part_two() {
        let mut cups = make_deque_part_two(&TEST_LABELING);
        assert_eq!(
            3,
            cups[0]
        );

        assert_eq!(
            10,
            cups[9],
        );

        assert_eq!(
            1000000,
            cups.len()
        );

        iterate(&mut cups, &mut 0, 10000000);
        rotate_to_one(&mut cups);

        assert_eq!(
            934001,
            cups[1]
        );

        assert_eq!(
            159792,
            cups[2]
        );
    }
}