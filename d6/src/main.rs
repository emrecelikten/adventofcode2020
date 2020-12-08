use std::collections::HashSet;

fn read_data(filename: &str) -> Vec<Vec<String>> {
    let buf = std::fs::read_to_string(filename).unwrap();
    let groups = buf.split("\n\n");
    groups.map(|group| group.split("\n")
        .filter(|e| !e.is_empty())
        .map(|e| e.to_owned())
        .collect())
        .collect()
}

fn main() {
    let data = read_data("input");
    let mut any_count = 0;
    let mut all_count = 0;
    for group in &data {
        let sets: Vec<HashSet<char>> = group.iter()
            .map(|person| person.chars().collect::<HashSet<char>>())
            .collect();

        let union = sets.iter()
            .fold(HashSet::new(),
                  |acc, e| acc.union(&e).cloned().collect());

        let intersection = sets
            .iter()
            .fold(sets[0].clone(),
                  |acc, e| acc.intersection(&e).cloned().collect());

        any_count += union.len();
        all_count += intersection.len();
    }

    println!("Group question count for any answered questions: {}", any_count);
    println!("Group question count for questions that were answered by all: {}", all_count);
}
