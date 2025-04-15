use std::collections::HashSet;
use std::{char, fs};

const FORBIDEN: &[char] = &['i','o','l'];

fn has_increasing_straight(password: &str) -> bool {
    let chars: Vec<char> = password.chars().collect();
    for i in 0..(chars.len() - 2) {
        if chars[i] as u8 + 1 == chars[i + 1] as u8 && chars[i] as u8 + 2 == chars[i + 2] as u8 {
            return true;
        }
    }
    false
}

fn has_two_pairs(password: &str) -> bool {
    let mut pairs = HashSet::new();
    let chars: Vec<char> = password.chars().collect();
    let mut i = 0;

    while i < chars.len() - 1 {
        if chars[i] == chars[i + 1] {
        pairs.insert(chars[i]);
        i += 2;
        } else {
            i += 1;
        }
        if pairs.len() >= 2{
            return true;
        }
    }
    false
}

fn increment_password(password: &str) -> String {
    let mut chars: Vec<char> = password.chars().collect();
    let mut i = chars.len() - 1;
    
    while i as i32 >= 0 {
        let mut next_char = ((chars[i] as u8 - b'a' + 1) % 26 + b'a') as char;

        if FORBIDEN.contains(&next_char) {
            next_char = ((chars[i] as u8 - b'a' + 1) % 26 + b'a') as char;
        }
        chars[i] = next_char;

        if chars[i] != 'a' {
            break;
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }
    chars.iter().collect()
}

fn generate_password(mut password: String) -> String {
    password = increment_password(&password);

    loop {
        if FORBIDEN.iter().any(|&c| password.contains(c)) {
            let idx = password.chars().position(|c| FORBIDEN.contains(&c)).unwrap();
            let mut chars: Vec<char> = password.chars().collect();
            chars[idx] = ((chars[idx] as u8 - b'a' + 1) % 26 + b'a') as char;

            for j in (idx + 1)..chars.len() {
                chars[j] = 'a';
            }
            password = chars.iter().collect();
            continue;
        }
        if has_increasing_straight(&password) && has_two_pairs(&password) {
            return password;
        }
        password = increment_password(&password);
    }
}

fn main() {
    let data = fs::read_to_string("../input.txt").expect("Error reading file").trim().to_string();
    let sep = "=".repeat(20);
    println!("{} Part 1 {}", sep, sep);
    let password = generate_password(data.clone());
    println!("Original: {}, updated: {}", data, password);
    println!("{} Part 2 {}", sep, sep);
    let new_password = generate_password("hepxxzaa".to_string());
    println!("Original: {}, updated: {}", password.clone(), new_password);
}
