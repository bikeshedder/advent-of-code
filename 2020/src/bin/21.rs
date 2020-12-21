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
    let foods: Vec<Food> = INPUT.lines().map(|line| Food::parse(line)).collect();
    let mut allergen_ingredients_map: HashMap<&str, HashSet<&str>> =
        foods.iter().fold(HashMap::new(), |mut map, food| {
            for allergen in food.allergens.iter() {
                map.entry(allergen)
                    .and_modify(|ingredients| ingredients.retain(|i| food.ingredients.contains(i)))
                    .or_insert_with(|| food.ingredients.clone());
            }
            map
        });
    let mut identified_allergens: Vec<(&str, &str)> = Vec::new();
    while !allergen_ingredients_map.is_empty() {
        let to_remove = allergen_ingredients_map
            .iter()
            .filter_map(|(allergen, ingredients)| {
                if ingredients.len() == 1 {
                    Some((*allergen, *ingredients.iter().next().unwrap()))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        for (allergen, ingredient) in to_remove {
            identified_allergens.push((allergen, ingredient));
            allergen_ingredients_map.remove(allergen);
            for ingredients in allergen_ingredients_map.values_mut() {
                ingredients.remove(ingredient);
            }
        }
    }

    let allergen_ingredients: HashSet<&str> = identified_allergens.iter().map(|(_, i)| *i).collect();
    let solution1 = foods
        .iter()
        .map(|food| food.ingredients.difference(&allergen_ingredients).count())
        .sum::<usize>();
    println!("{}", solution1);

    identified_allergens.sort();
    let solution2 = identified_allergens
        .iter()
        .map(|(_, ingredient)| *ingredient)
        .collect::<Vec<_>>()
        .join(",");
    println!("{}", solution2);
}
