use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::str::FromStr;
use util::read_input;

#[derive(Debug)]
struct Food {
    ingredients: Box<[String]>,
    allergens: Box<[String]>,
}

impl FromStr for Food {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" (contains ");
        let mut ingredients = Vec::new();
        for ingredient in split.next().unwrap().split(' ') {
            ingredients.push(ingredient.to_string());
        }
        let mut allergens = Vec::new();
        for allergen in split.next().unwrap().strip_suffix(')').unwrap().split(", ") {
            allergens.push(allergen.to_string());
        }

        Ok(Self {
            ingredients: ingredients.into_boxed_slice(),
            allergens: allergens.into_boxed_slice(),
        })
    }
}

fn allergen_free(input: &[Food]) -> usize {
    let mut allergen_to_ingredient = HashMap::new();
    let mut all_ingredients = HashSet::new();
    for food in input {
        let ingredient_set = HashSet::from_iter(food.ingredients.iter());
        for allergen in food.allergens.iter() {
            let mut old_ingredient_set = allergen_to_ingredient
                .entry(allergen)
                .or_insert(ingredient_set.clone());
            *old_ingredient_set = old_ingredient_set
                .intersection(&ingredient_set)
                .cloned()
                .collect();
        }
        all_ingredients = all_ingredients.union(&ingredient_set).cloned().collect();
    }

    let mut non_allergenic_ingredients = all_ingredients;

    for ingredient_set in allergen_to_ingredient.values() {
        non_allergenic_ingredients = non_allergenic_ingredients
            .difference(ingredient_set)
            .cloned()
            .collect();
    }

    let mut result = 0;
    for food in input {
        for ingredient in food.ingredients.iter() {
            if non_allergenic_ingredients.contains(ingredient) {
                result += 1;
            }
        }
    }

    result
}

fn find_ingredients(input: &[Food]) -> String {
    let mut allergen_to_ingredient = HashMap::new();
    for food in input {
        let ingredient_set: HashSet<_> = HashSet::from_iter(food.ingredients.iter());
        for allergen in food.allergens.iter() {
            let mut old_ingredient_set = allergen_to_ingredient
                .entry(allergen)
                .or_insert(ingredient_set.clone());
            *old_ingredient_set = old_ingredient_set
                .intersection(&ingredient_set)
                .cloned()
                .collect();
        }
    }

    let mut identified = HashSet::new();
    loop {
        let mut found = false;

        let kv = allergen_to_ingredient.clone();
        for (key, value) in kv {
            if value.len() == 1 && !identified.contains(value.iter().next().unwrap()) {
                found = true;
                identified.insert(value.iter().next().unwrap().clone());
                for (key2, value2) in allergen_to_ingredient.iter_mut() {
                    if key != *key2 {
                        value2.remove(value.iter().next().unwrap());
                        // value2 = value2.difference(value).cloned().collect();
                    }
                }
            }
        }

        if !found {
            break;
        }
    }

    let mut allergens_found = allergen_to_ingredient.keys().collect::<Vec<_>>();
    allergens_found.sort();
    let mut result = Vec::new();
    for allergen in allergens_found {
        result.push(
            allergen_to_ingredient[allergen]
                .iter()
                .next()
                .unwrap()
                .clone()
                .clone(),
        );
    }

    // let mut result = identified.iter().cloned().cloned().collect::<Vec<_>>();
    // result.sort();
    result.join(",")
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<Food>(&args[1]).collect::<Vec<Food>>();

    println!("{}", allergen_free(&input));
    println!("{}", find_ingredients(&input));
}
