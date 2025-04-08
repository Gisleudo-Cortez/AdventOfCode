use std::{char, fs};

fn gen_row(prev_row: &[char]) -> Vec<char> {
    let width = prev_row.len();
    let mut next_row: Vec<char> = Vec::with_capacity(width);

    for i in 0..width {
        let left = prev_row.get(i.wrapping_sub(1)).unwrap_or(&'.');
        let center = prev_row[i];
        let right = prev_row.get(i + 1).unwrap_or(&'.');

        if (*left == '^' && center == '^' && *right == '.')
            || (*left == '.' && center == '^' && *right == '^')
            || (*left == '^' && center == '.' && *right == '.')
            || (*left == '.' && center == '.' && *right == '^')
        {
            next_row.push('^');
        } else {
            next_row.push('.');
        }
    }
    next_row
}

fn count_safe_tiles(row: &[char]) -> i32 {
    row.iter().filter(|&c| *c == '.').count() as i32
}

fn gen_and_count_safe(input: &str, rows: usize) -> i32 {
    let mut current_row: Vec<char> = input.chars().collect();
    let mut total_safe_tiles = 0;
    for _ in 0..rows {
        total_safe_tiles += count_safe_tiles(&current_row);
        current_row = gen_row(&current_row);
    }
    total_safe_tiles
}

fn main() {
    let binding = fs::read_to_string("../input.txt").expect("Error reading input file");
    let input = binding.trim();
    let sep = "=".repeat(20);
    println!("{sep} Part 1 {sep}");
    let part1 = gen_and_count_safe(input, 40);
    println!("The total number of safe tiles for 40 rows is: {part1}");
    let part2 = gen_and_count_safe(input, 400000);
    println!("{sep} Part 2 {sep}");
    println!("The total number of safe tiles for 400000 rows is: {part2}");
}
