use md5::{Digest, Md5};
use std::fs;
use std::io::Write;

fn calculate_md5(input: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

fn generate_password_part1(door_id: &str) -> String {
    (0..)
        .filter_map(|index| {
            let hash = calculate_md5(&format!("{}{}", door_id, index));
            if hash.starts_with("00000") {
                hash.chars().nth(5)
            } else {
                None
            }
        })
        .take(8)
        .collect()
}

fn generate_password_part2(door_id: &str) -> String {
    let mut password = ['_'; 8];
    let mut found_positions = 0;
    let mut index = 0;

    print!("\nDecrypting: [{}]\r", password.iter().collect::<String>());

    while found_positions < 8 {
        let hash = calculate_md5(&format!("{}{}", door_id, index));

        if hash.starts_with("00000") {
            if let (Some(pos_char), Some(value_char)) = (hash.chars().nth(5), hash.chars().nth(6)) {
                if let Some(pos) = pos_char.to_digit(10) {
                    if pos < 8 && password[pos as usize] == '_' {
                        password[pos as usize] = value_char;
                        found_positions += 1;

                        print!("\rDecrypting: [{}]", password.iter().collect::<String>());
                        std::io::stdout().flush().unwrap();
                    }
                }
            }
        }
        index += 1;
    }

    println!();
    password.iter().collect()
}

fn main() {
    let door_id = fs::read_to_string("../input.txt")
        .expect("Error reading input file")
        .trim()
        .to_string();

    let sep = "=".repeat(20);
    println!("{sep} Part 1 {sep}");
    let part1 = generate_password_part1(&door_id);
    println!("The password is: {part1}");

    println!("{sep} Part 2 {sep}");
    let part2 = generate_password_part2(&door_id);
    println!("The password is: {part2}");
}
