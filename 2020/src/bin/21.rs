use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input/21.txt");

struct Food<'a> {
    ingredients: HashSet<&'a str>,
    allergens: HashSet<&'a str>,
}

impl<'a> Food<'a> {
    fn parse(s: &'a str) -> Self {
        let mut line_it = s.split(" (contains ");
        let ingredients = line_it.next().unwrap().split(" ").collect::<HashSet<_>>();
        let allergens = line_it.next().unwrap();
        let allergens = allergens[..allergens.len() - 1]
            .split(", ")
            .collect::<HashSet<_>>();
        Self {
            ingredients,
            allergens,
        }
    }
}

fn main() {
    let mut foods: Vec<Food> = INPUT.lines().map(|line| Food::parse(line)).collect();
    let mut allergen_foods_map: HashMap<&str, Vec<usize>> = foods
        .iter()
        .map(|food| &food.allergens)
        .enumerate()
        .fold(HashMap::new(), |mut map, (food_idx, allergens)| {
            for allergen in allergens {
                map.entry(allergen).or_default().push(food_idx);
            }
            map
        });
    let mut allergen_ingredient_map: Vec<(&str, &str)> = Vec::new();
    while !allergen_foods_map.is_empty() {
        let mut to_remove: Vec<(&str, &str)> = Vec::new();
        for (allergen, food_indices) in allergen_foods_map.iter() {
            let mut candidates = foods[food_indices[0]].ingredients.clone();
            for idx in &food_indices[1..] {
                let ingredients = &foods[*idx].ingredients;
                candidates.retain(|i| ingredients.contains(i));
            }
            if candidates.len() == 1 {
                to_remove.push((allergen, candidates.iter().next().unwrap()));
            }
        }
        for (allergen, ingredient) in to_remove {
            allergen_ingredient_map.push((allergen, ingredient));
            allergen_foods_map.remove(allergen);
            for food in foods.iter_mut() {
                food.ingredients.remove(ingredient);
            }
        }
    }

    let solution1 = foods
        .iter()
        .map(|food| food.ingredients.len())
        .sum::<usize>();
    println!("{}", solution1);

    allergen_ingredient_map.sort();
    let solution2 = allergen_ingredient_map
        .iter()
        .map(|(_, ingredient)| ingredient.to_string())
        .collect::<Vec<_>>()
        .join(",");
    println!("{}", solution2);
}
