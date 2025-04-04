use std::fs;

fn gen_string(base_data: String) -> String {
    let b: String = base_data.clone().chars().rev().collect();
    let mut replaced_b = String::with_capacity(base_data.len());
    for char in b.chars() {
        if char == '1' {
            replaced_b.push('0');
        } else if char == '0' {
            replaced_b.push('1');
        } else {
            panic!("Invalid character found");
        }
    }
    format!("{base_data}0{replaced_b}")
}

fn fill_disk(current_data: String, space_to_fill: usize) -> String {
    let mut new_data = gen_string(current_data.clone());
    while new_data.len() <= space_to_fill {
        new_data = gen_string(new_data);
    }
    new_data
}

fn gen_checksum(data: String) -> String {
    let chars: Vec<char> = data.chars().collect();
    let mut checksum = String::with_capacity(data.len() / 2);
    for part in chars.chunks(2) {
        if part[0] == part[1] {
            checksum.push('1');
        } else {
            checksum.push('0');
        }
    }
    checksum
}

fn solve(input: String, space_to_fill: usize) -> String {
    let mut data = fill_disk(input, space_to_fill);
    data.truncate(space_to_fill);
    let mut checksum = gen_checksum(data);
    while checksum.len() % 2 == 0 {
        checksum = gen_checksum(checksum)
    }
    checksum
}

fn main() {
    let binding = fs::read_to_string("../input.txt").expect("Error reading input file");
    let input = binding.trim().to_string();
    let sep = "=".repeat(20);
    println!("{sep} Part 1 {sep}");
    let part1 = solve(input.clone(), 272);
    println!("The checksum is: {part1}");
    println!("{sep} Part 2 {sep}");
    let part2 = solve(input, 35651584);
    println!("The checksum is: {part2}");
}
