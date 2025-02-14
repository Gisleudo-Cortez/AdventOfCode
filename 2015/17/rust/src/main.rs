use std::{collections::HashMap, fs, num::ParseIntError};


fn backtrack(
    index: usize, 
    remaining: i32, 
    combo_length:i32,
    comb_by_length: &mut HashMap<i32, i32>,
    containers: &[i32]){
        if remaining == 0 {
            *comb_by_length.entry(combo_length).or_insert(0) += 1;
            return;
    }
        if remaining < 0 || index >= containers.len() {
            return ;
        }

        backtrack(
            index + 1, 
            remaining - containers[index], combo_length + 1, 
            comb_by_length, 
            containers);

        backtrack(
            index + 1, 
            remaining, 
            combo_length, 
            comb_by_length, 
            containers);
}

fn find_combinations(containers: &mut Vec<i32>, target: i32) -> (i32, i32) {
    containers.sort_by(|a,b| b.cmp(a));
    let mut combinations_by_length: HashMap<i32, i32> = HashMap::new();
    
    backtrack(0, target, 0, &mut combinations_by_length, containers);

    if combinations_by_length.is_empty() {
        return (0,0);
    }

    let total_combinations: i32 = combinations_by_length.values().sum();
    let min_length: &i32 = combinations_by_length.keys().min().unwrap();
    let min_length_combinations = combinations_by_length[min_length];
    
    (total_combinations, min_length_combinations)
}

fn parse_input(path: &str) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(path)
    .expect("Error reading input file")
    .trim()
    .lines()
    .map(|n| n.parse::<i32>())
    .collect::<Result<Vec<_>, ParseIntError>>()?;
    Ok(data)
}

fn main() {
    let sep = "=".repeat(20);
    let target_volume = 150;
    
    match parse_input("../input.txt") {
        Ok(mut data) => {
            if data.is_empty() {
                println!("No valid data in file");
                return;
            }
            let (total_combinations, min_combinations) = find_combinations(&mut data, target_volume);
            println!("{} Part 1 {}\nTotal combinations: {}", sep, sep, total_combinations);
            println!("{} Part 2 {}\nMinimum containers combinations: {}", sep, sep, min_combinations);
        }
        Err(e) => println!("Error parsing input file with error: {}", e),
    }
}
