use regex::Regex;
use std::fs;

fn calc_value(x: u64) -> u64 {
    (x * 252533) % 33554393
}

fn get_code(r: u64, c: u64) -> u64 {
    let mut current_value: u64 = 20151125;
    let target_index: u64 = (1..(r + c)).sum::<u64>() - (r - 1);

    for _ in 1..target_index {
        current_value = calc_value(current_value);
    }
    current_value
}

fn main() {
    let content = fs::read_to_string("../input.txt")
        .expect("Failed to read input file")
        .trim()
        .to_string();

    let re = Regex::new(r"\d+").unwrap();
    let numbers: Vec<u64> = re
        .find_iter(&content)
        .map(|m| m.as_str().parse::<u64>().unwrap())
        .collect();
    let code = get_code(numbers[0], numbers[1]);
    println!("The code to start the machine is: {code}");
}
