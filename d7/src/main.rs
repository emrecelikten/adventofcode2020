/// I am not really proud of this one.

extern crate regex;

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::str::FromStr;

use regex::Regex;

fn read_data(filename: &str) -> Vec<String> {
    let data = std::fs::read_to_string(filename).unwrap();
    data.split("\n").map(|e| e.to_owned()).filter(|e| !e.is_empty()).collect()
}

#[derive(PartialEq, Debug)]
struct BagCountEntry {
    bag_type: String,
    count: usize,
}

fn parse_data_one(data: &Vec<String>) -> HashMap<String, Vec<BagCountEntry>> {
    let line_regex: Regex = Regex::new(r"(\d+)\s(.+?) bags?(?:,\s|\.)").unwrap();

    let mut m: HashMap<String, Vec<BagCountEntry>> = HashMap::new();
    for line in data {
        let mut splitted = line.split("contain").map(|e| e.trim());
        let key = splitted.next().and_then(|e| e.strip_suffix(" bags")).unwrap();
        let captures_iter = line_regex.captures_iter(splitted.next().unwrap());

        for capture in captures_iter {
            let count = usize::from_str(&capture[1]).unwrap();
            let child_key = &capture[2];
            let new_entry = BagCountEntry { bag_type: key.to_owned(), count };
            match m.get_mut(child_key) {
                Some(v) => { v.push(new_entry); }
                None => { m.insert(child_key.to_owned(), vec!(new_entry)); }
            }
        }
    }
    m
}

fn count_one(bag_type: &str, bag_map: &HashMap<String, Vec<BagCountEntry>>) -> usize {
    if let Some(parents) = bag_map.get(bag_type) {
        let mut to_visit: HashSet<&String> = HashSet::from_iter(parents.iter().map(|e| &e.bag_type));
        let mut visited: HashSet<&String> = HashSet::new();
        while !to_visit.is_empty() {
            let cur = to_visit.iter().next().unwrap().clone();
            to_visit.remove(cur);
            if let Some(parents) = bag_map.get(cur) {
                to_visit.extend(parents.iter().map(|e| &e.bag_type));
            }
            visited.insert(cur);
        }
        visited.len()
    } else { 0 }
}

fn parse_data_two(data: &Vec<String>) -> HashMap<String, Vec<BagCountEntry>> {
    let line_regex: Regex = Regex::new(r"(\d+)\s(.+?) bags?(?:,\s|\.)").unwrap();

    let mut m: HashMap<String, Vec<BagCountEntry>> = HashMap::new();
    for line in data {
        let mut splitted = line.split("contain").map(|e| e.trim());
        let key = splitted.next().and_then(|e| e.strip_suffix(" bags")).unwrap();
        let captures_iter = line_regex.captures_iter(splitted.next().unwrap());

        let mut children_vec: Vec<BagCountEntry> = Vec::new();
        for capture in captures_iter {
            let count = usize::from_str(&capture[1]).unwrap();
            let child_key = &capture[2];
            let new_entry = BagCountEntry { bag_type: child_key.to_owned(), count };
            children_vec.push(new_entry);
        }
        m.insert(key.to_owned(), children_vec);
    }
    m
}

fn count_two(bag_type: &str, bag_map: &HashMap<String, Vec<BagCountEntry>>) -> usize {
    if let Some(entries) = bag_map.get(bag_type) {
        let mut count = 0;
        for entry in entries {
            count += entry.count;
            count += entry.count * count_two(&entry.bag_type, bag_map);
        }
        count
    } else { 0 }
}

fn main() {
    let data = read_data("input");
    let m = parse_data_one(&data);
    let c = count_one("shiny gold", &m);
    println!("{} many bags can contain a shiny gold bag.", c);

    let m2 = parse_data_two(&data);
    let c2 = count_two("shiny gold", &m2);
    println!("A shiny gold bag contains {} many bags.", c2);

}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_count_one() {
        let mut m: HashMap<String, Vec<BagCountEntry>> = HashMap::new();
        let vec1 = vec![BagCountEntry { bag_type: "b".to_string(), count: 1 },
                        BagCountEntry { bag_type: "c".to_owned(), count: 2 }];
        let vec2 = vec![BagCountEntry { bag_type: "c".to_owned(), count: 1 }];
        m.insert("a".to_owned(), vec1);
        m.insert("b".to_owned(), vec2);

        assert_eq!(count_one("a", &m), 2);
        assert_eq!(count_one("b", &m), 1);
        assert_eq!(count_one("c", &m), 0);
    }

    #[test]
    fn test_parse_data_one() {
        let line = "light gold bags contain 2 light lime bags, 1 faded green bag, 3 clear olive bags, 2 dim bronze bags.";
        let result = parse_data_one(&vec![line.to_owned()]);
        let mut expected: HashMap<String, Vec<BagCountEntry>> = HashMap::new();
        expected.insert("light lime".to_owned(),
                        vec![BagCountEntry { bag_type: "light gold".to_owned(), count: 2 }]);
        expected.insert("faded green".to_owned(),
                        vec![BagCountEntry { bag_type: "light gold".to_owned(), count: 1 }]);
        expected.insert("clear olive".to_owned(),
                        vec![BagCountEntry { bag_type: "light gold".to_owned(), count: 3 }]);
        expected.insert("dim bronze".to_owned(),
                        vec![BagCountEntry { bag_type: "light gold".to_owned(), count: 2 }]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_data_two() {
        let line = "light gold bags contain 2 light lime bags, 1 faded green bag, 3 clear olive bags, 2 dim bronze bags.";
        let result = parse_data_two(&vec![line.to_owned()]);
        let mut expected: HashMap<String, Vec<BagCountEntry>> = HashMap::new();
        expected.insert("light gold".to_owned(),
                        vec![BagCountEntry { bag_type: "light lime".to_owned(), count: 2 },
                             BagCountEntry { bag_type: "faded green".to_owned(), count: 1 },
                             BagCountEntry { bag_type: "clear olive".to_owned(), count: 3 },
                             BagCountEntry { bag_type: "dim bronze".to_owned(), count: 2 }]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_one() {
        let data = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        let data_vec = data.split("\n").map(|e| e.to_owned()).collect();
        let m = parse_data_one(&data_vec);
        assert_eq!(count_one("shiny gold", &m), 4);
    }

    #[test]
    fn test_part_two() {
        let data = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        let data_vec = data.split("\n").map(|e| e.to_owned()).collect();
        let m = parse_data_two(&data_vec);
        assert_eq!(count_two("shiny gold", &m), 32);
    }

    #[test]
    fn test_part_two_2() {
        let data = r"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

        let data_vec = data.split("\n").map(|e| e.to_owned()).collect();
        let m = parse_data_two(&data_vec);
        assert_eq!(count_two("shiny gold", &m), 126);
    }
}