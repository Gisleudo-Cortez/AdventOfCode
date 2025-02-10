use std::{collections::HashMap, fs};
use itertools::Itertools;

fn parse(data: &[String]) -> HashMap<String, HashMap<String,i32>> {
    let mut out_hash: HashMap<String, HashMap<String,i32>> = HashMap::new();

    for line in data {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let person1 = parts[0].to_string();
        let person2 = parts.last().unwrap().trim_matches('.').to_string();

        let mut happiness = parts[3].parse::<i32>().unwrap();
        if parts[2] == "lose"{
            happiness = -happiness;
        }
        out_hash.entry(person1.clone())
        .or_insert_with(HashMap::new)
        .insert(person2, happiness);
    }
    out_hash
}

fn parse_2(data: &[String]) -> HashMap<String, HashMap<String,i32>> {
    let mut preferences = parse(data);
    let mut all_guests: Vec<String> = preferences.keys().cloned().collect();

    for prefs in preferences.values() {
        for guest in prefs.keys() {
            if !all_guests.contains(guest) {
                all_guests.push(guest.clone());
            }
        }
    }

    let me = "me".to_string();
    preferences.insert(me.clone(), HashMap::new());
    for guest in &all_guests {
        preferences.get_mut(&me).unwrap().insert(guest.clone(), 0);
        preferences.entry(guest.clone()).or_insert_with(HashMap::new).insert(me.clone(), 0);
    }
    preferences
}

fn calculate_happines(
    arrangement: &[&String], 
    preferences: &HashMap<String, HashMap<String,i32>>) 
    -> i32 {
        let mut total_happines = 0;
        let num_people = arrangement.len();

        for i in 0..num_people {
            let person = arrangement[i];
            let left = arrangement[(i + num_people - 1) % num_people];
            let rigth = arrangement[(i + 1) % num_people];

            total_happines += preferences[person][left];
            total_happines += preferences[person][rigth];
        }
        total_happines
}

fn find_best_seating(preferences: &HashMap<String, HashMap<String,i32>>) -> i32{
    let people: Vec<&String> = preferences.keys().collect();
    let mut max_happiness = i32::MIN;

    for arrangement in people.iter().permutations(people.len()) {
        let arrangement: Vec<&String> = arrangement.into_iter().cloned().collect();
        max_happiness = max_happiness.max(calculate_happines(&arrangement, preferences));
    }
    max_happiness
}

fn main() {
    let data: Vec<String> = fs::read_to_string("../input.txt")
    .expect("Failed to read file")
    .lines()
    .map(String::from)
    .collect();

    let sep = "=".repeat(20);
    let part1 = find_best_seating(&parse(&data));
    let part2 = find_best_seating(&parse_2(&data));
    println!("{} Part 1 {} \nMax happiness score: {}",sep, sep, part1);
    println!("{} Part 2 {} \nMax happiness score: {}",sep, sep, part2);
}
