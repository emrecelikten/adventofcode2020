use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

type AllergenMap<'a> = HashMap<&'a str, HashSet<&'a str>>;
type SolvedAllergenMap<'a> = HashMap<&'a str, &'a str>;
type CountMap<'a> = HashMap<&'a str, usize>;

fn read_data(filename: &str) -> String {
    std::fs::read_to_string(filename).unwrap()
}

fn count_ingredients<'a>(data: &str, allergen_map: &AllergenMap<'a>) -> CountMap<'a> {
    let mut result: CountMap = allergen_map.iter()
        .map(|(&ingredient, _)| {
            (ingredient, 0)
        })
        .collect();

    for line in data.lines() {
        let words: Vec<&str> = line.split(" (contains").next().unwrap().split(" ").collect();
        for (ingredient, count) in result.iter_mut() {
            if words.contains(ingredient) {
                *count += 1;
            }
        }
    }

    result
}

fn construct_potential_allergen_map(data: &str) -> AllergenMap {
    let mut inverse: AllergenMap = AllergenMap::new();
    let mut result = AllergenMap::new();

    for line in data.lines() {
        let mut splitted = line.split(" (contains ");
        let ingredients_str = splitted.next().unwrap().split(" ");
        let allergens_str = splitted.next().unwrap().strip_suffix(")").unwrap().split(", ");
        let ingredients = HashSet::from_iter(ingredients_str);
        let allergens = HashSet::from_iter(allergens_str);

        for ingredient in ingredients.iter() {
            let s = result
                .entry(*ingredient)
                .or_insert(allergens.clone());

            *s = s.union(&allergens).cloned().collect();
        }

        for allergen in allergens.iter() {
            let s = inverse
                .entry(allergen)
                .or_insert(ingredients.clone());

            let intersection: HashSet<&str> = s.intersection(&ingredients).cloned().collect();
            let diff = s.symmetric_difference(&ingredients);
            for ingredient in diff {
                result.get_mut(ingredient).unwrap().remove(allergen);
            }
            *s = intersection;
        }
    }

    result
}

fn eliminate_allergens<'a>(allergen_map: &mut AllergenMap<'a>, solved_map: &mut SolvedAllergenMap<'a>) {
    loop {
        let mut removed = false;
        let mut to_check = Vec::new();
        for (ingredient, _) in allergen_map.iter()
            .filter(|(_, v)| v.len() == 1) {
            to_check.push(*ingredient)
        }

        for ingredient in to_check {
            solved_map.insert(ingredient, allergen_map.get(ingredient).unwrap().iter().next().unwrap());
            allergen_map.remove(ingredient);
            break;
        }
        for allergen in solved_map.values() {
            for (_, allergens) in allergen_map.iter_mut() {
                if allergens.contains(allergen) {
                    allergens.remove(*allergen);
                    removed = true;
                }
            }
        }

        if !removed { break; }
    }
}


fn main() {
    let data = read_data("input");
    let mut allergen_map = construct_potential_allergen_map(&data);
    let mut map = SolvedAllergenMap::new();
    eliminate_allergens(&mut allergen_map, &mut map);

    let filtered: AllergenMap = allergen_map
        .clone()
        .into_iter()
        .filter(|(_, a)| a.len() == 0)
        .collect();
    let counts = count_ingredients(&data, &filtered);
    let sum: usize = counts.values().sum();
    println!("Result #1: {}", sum);

    let mut allergens: Vec<(&&str, &&str)> = map.iter().collect();
    allergens.sort_by_key(|(_, allergen)| *allergen);

    let gibberish: Vec<&str> = allergens.iter().map(|x| *x.0).collect();
    println!("Result #2: {}", gibberish.join(","));

}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &'static str = r"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn test_eliminate_allergen() {
        let mut allergens = construct_potential_allergen_map(&TEST_DATA);
        let mut solved = SolvedAllergenMap::new();
        // solved.insert("fvjkl", "soy");
        eliminate_allergens(&mut allergens, &mut solved);
        dbg!(&allergens);
        dbg!(&solved);
        assert_eq!(
            &HashSet::new(),
            allergens.get("trh").unwrap()
        );
    }

    #[test]
    fn test_count_ingredients() {
        let allergens = construct_potential_allergen_map(&TEST_DATA);
        let filtered: AllergenMap = allergens.into_iter()
            .filter(|(_, a)| a.is_empty())
            .collect();
        let counts = count_ingredients(&TEST_DATA, &filtered);

        assert_eq!(
            5_usize,
            counts.values().sum()
        );
    }
}