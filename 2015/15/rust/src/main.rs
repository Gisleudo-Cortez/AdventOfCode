use std::{collections::HashMap, fs};

struct Ingredients {
    name:String,
    capacity:i32,
    durability:i32,
    flavor:i32,
    texture:i32,
    calories:i32,
}

fn parse(data: &str) -> Ingredients {
    let binding = data.replace(",","");
    let data: Vec<&str> = binding.split_whitespace().collect();
    Ingredients {
        name: data[0].trim_end_matches(':').to_string(),
        capacity: data[2].parse::<i32>().expect("Error parsing capacity"),
        durability: data[4].parse::<i32>().expect("Error parsing durability"),
        flavor: data[6].parse::<i32>().expect("Error parsing flavor"),
        texture: data[8].parse::<i32>().expect("Error parsing texture"),
        calories: data[10].parse::<i32>().expect("Error parsing calories"),
    }
}

fn calculate_score(amounts: &[i32], ingredients: &[Ingredients], target_calories: Option<i32>) -> i32 {
    let num_ingredients = amounts.len();
    let mut capacity = 0;
    let mut durability = 0;
    let mut flavor = 0;
    let mut texture = 0;
    let mut calories = 0;

    for i in 0..num_ingredients {
        capacity += amounts[i] * ingredients[i].capacity;
        durability += amounts[i] * ingredients[i].durability;
        flavor += amounts[i] * ingredients[i].flavor;
        texture += amounts[i] * ingredients[i].texture;
        calories += amounts[i] * ingredients[i].calories;
    }

    if let Some(target) = target_calories {
        if calories != target {
            return 0;
        }
    }
    let properties = [capacity, durability, flavor, texture];
    properties.iter().map(|&p| p.max(0)).product()
}

fn find_best_combination(
    index: usize,
    remaining_teaspoons: i32,
    current_amounts: &mut Vec<i32>,
    ingredients: &[Ingredients],
    target_calories: Option<i32>,
    max_score: &mut i32,
    best_amounts: &mut HashMap<String, i32>,
) {
    if index == ingredients.len() -1 {
        current_amounts[index] = remaining_teaspoons;
        let score = calculate_score(current_amounts, ingredients, target_calories);

        if score > *max_score {
            *max_score = score;
            *best_amounts = ingredients
            .iter()
            .enumerate()
            .map(|(i, ing)| (ing.name.clone(), current_amounts[i]))
            .collect();
        }
        return;
    }
    for teaspoons in 0..=remaining_teaspoons {
        current_amounts[index] = teaspoons;
        find_best_combination(
            index + 1, 
            remaining_teaspoons - teaspoons, current_amounts, 
            ingredients, 
            target_calories, 
            max_score, 
            best_amounts
        );
    }
}

fn calculate_max_score(
    ingredients: &[Ingredients],
    n_spoons: i32,
    target_calories: Option<i32>,
) -> (i32, HashMap<String, i32>) {
    
    let mut max_score = 0;
    let mut best_amounts = HashMap::new();
    let mut current_amounts = vec![0; ingredients.len()];

    find_best_combination(0,
         n_spoons,
         &mut current_amounts, 
         ingredients, 
         target_calories, 
         &mut max_score,
         &mut best_amounts
        );
        (max_score, best_amounts)
}

fn main() {
    let data: Vec<Ingredients> = fs::read_to_string("../input.txt")
    .expect("Error reading input file")
    .trim()
    .lines()
    .map(|l| parse(l))
    .collect();

    let sep = "=".repeat(20);
    let (max_score_1, best_amounts_1) = calculate_max_score(&data, 100, None);
    println!("{} Part 1 {}\nmax_score: {}\nbest_amounts: {:?}", sep,sep, max_score_1, best_amounts_1);

    let (max_score_2, best_amounts_2) = calculate_max_score(&data, 100, Some(500));
    println!("{} Part 2 {}\nmax_score: {}\nbest_amounts: {:?}", sep,sep, max_score_2, best_amounts_2);
}
