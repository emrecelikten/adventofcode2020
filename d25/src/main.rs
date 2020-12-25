fn read_data(filename: &str) -> String {
    std::fs::read_to_string(filename).unwrap()
}

fn transform(subject_number: usize, loop_size: usize) -> usize {
    let mut result = 1;
    for _ in 0..loop_size {
        result *= subject_number;
        result %= 20201227;
    }
    result
}

fn find_loop_size(subject_number: usize, target_key: usize) -> usize {
    let mut result = 1;
    let mut loop_size = 0;
    loop {
        loop_size += 1;
        result *= subject_number;
        result %= 20201227;
        if result == target_key { break; }
    }
    loop_size
}


fn main() {
    let card = 2959251;
    let door = 4542595;

    let card_loop = find_loop_size(7, card);
    let encryption = transform(door, card_loop);

    println!("Result #1: {}", encryption);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_loop_size() {
        let subject_number = 7;
        let pub1 = 5764801;
        assert_eq!(
            8,
            find_loop_size(subject_number, pub1)
        );

        let pub2 = 17807724;
        assert_eq!(
            11,
            find_loop_size(subject_number, pub2)
        );

        assert_eq!(
            14897079,
            transform(pub2, 8)
        );

        assert_eq!(
            transform(pub2, 8),
            transform(pub1, 11)
        );
    }

}
