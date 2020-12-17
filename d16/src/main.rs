use std::collections::HashMap;
use std::ops::RangeInclusive;

use regex::Regex;

fn load_data(filename: &str) -> String {
    std::fs::read_to_string(filename).unwrap()
}

type RuleMap = HashMap<String, (RangeInclusive<usize>, RangeInclusive<usize>)>;

fn parse_data(data: &str) -> (RuleMap, Vec<Vec<usize>>) {
    let mut rules: RuleMap = HashMap::new();
    let mut tickets: Vec<Vec<usize>> = Vec::new();
    let rule_regex = Regex::new(r"(.+?): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();

    let mut lines = data.lines().filter(|x| !x.is_empty());
    for line in (&mut lines).take_while(|x| rule_regex.is_match(x)) {
        let captures = rule_regex.captures_iter(line).nth(0).unwrap();
        rules.insert(captures[1].to_owned(),
                     (RangeInclusive::new(captures[2].parse().unwrap(),
                                          captures[3].parse().unwrap()),
                      RangeInclusive::new(captures[4].parse().unwrap(),
                                          captures[5].parse().unwrap())));
    }

    let my_ticket = lines.nth(0).unwrap().split(",").map(|x| x.parse().unwrap()).collect();
    tickets.push(my_ticket);

    for line in lines.skip(1) {
        let ticket = line.split(",").map(|x| x.parse().unwrap()).collect();
        tickets.push(ticket);
    }

    (rules, tickets)
}

fn part_one(rules: &RuleMap, tickets: &[Vec<usize>]) -> usize {
    let mut result = 0;
    for ticket in tickets {
        for num in ticket {
            let valid = is_valid_ticket(rules, num);
            if !valid {
                result += num;
            }
        }
    }

    result
}

fn is_valid_ticket(rules: &HashMap<String, (RangeInclusive<usize>, RangeInclusive<usize>)>, num: &usize) -> bool {
    let mut valid = false;
    for (_, (fr, sr)) in rules {
        if fr.contains(num) || sr.contains(num) {
            valid = true;
            break;
        }
    }
    valid
}

fn part_two(rules: &RuleMap, tickets: &[Vec<usize>]) -> HashMap<String, usize> {
    let filtered: Vec<&Vec<usize>> = tickets
        .iter()
        .filter(|ticket|
            ticket
                .iter()
                .all(|num| is_valid_ticket(rules, num)))
        .collect();


    let len = filtered[0].len();
    let mut possible_positions: Vec<Vec<&str>> = vec![Vec::new(); len];
    for (rule, (fr, sr)) in rules {
        for col in 0..len {
            if filtered.iter().all(|nums| fr.contains(&nums[col]) || sr.contains(&nums[col])) {
                possible_positions[col].push(&rule);
            }
        }
    }

    let mut positions: Vec<&str> = vec![""; len];
    loop {
        if let Some((min_pos, min_list)) = possible_positions
            .iter_mut()
            .enumerate()
            .filter(|(_, pos_list)| !pos_list.is_empty())
            .min_by_key(|(_, pos_list)| pos_list.len())
        {
            // Hopefully no dyn. prog.
            assert_eq!(min_list.len(), 1);

            let rule = min_list[0];
            for list in possible_positions.iter_mut() {
                if let Some(pos) = list.iter().position(|x| *x == rule) {
                    list.remove(pos);
                }
            }
            positions[min_pos] = rule;
        } else { break; }
    }

    let mut result = HashMap::new();
    for i in 0..len {
        result.insert(positions[i].to_owned(), tickets[0][i]);
    }

    result
}

fn main() {
    let (rules, tickets) = parse_data(&load_data("input"));
    let result_one = part_one(&rules, &tickets);
    println!("Result #1: {}", result_one);

    let result_two = part_two(&rules, &tickets);
    let mul = result_two.iter()
        .filter_map(|(k, v)| if k.contains("departure") { Some(v)} else {None})
        .fold(1, |acc, e| acc * e);

    println!("Result #2: {}", mul);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA1: &'static str = r"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    const TEST_DATA2: &'static str = r"class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

    #[test]
    fn test_parse_data() {
        let (rulemap, tickets) = parse_data(&TEST_DATA1);
        assert_eq!(rulemap.get("row").unwrap(), &(6..=11, 33..=44));
        assert_eq!(tickets[0], vec![7, 1, 14]);
        assert_eq!(tickets[tickets.len() - 1], vec![38, 6, 12]);
    }

    #[test]
    fn test_part_one() {
        let (rulemap, tickets) = parse_data(&TEST_DATA1);
        let result = part_one(&rulemap, &tickets);
        assert_eq!(result, 71);
    }

    #[test]
    fn test_part_two() {
        let (rulemap, tickets) = parse_data(&TEST_DATA2);
        let result = part_two(&rulemap, &tickets);
        assert_eq!(result.get("class").unwrap(), &12);
        assert_eq!(result.get("row").unwrap(), &11);
        assert_eq!(result.get("seat").unwrap(), &13);
    }
}
