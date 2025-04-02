use std::{collections::HashMap, fs};

use md5::{Digest, Md5};

fn get_hash(salt: &str, index: usize, stretch: bool, cache: &mut HashMap<usize, String>) -> String {
    if let Some(cached_hash) = cache.get(&index) {
        return cached_hash.clone();
    }

    let input = format!("{salt}{index}");
    let mut hasher = Md5::new();
    hasher.update(input.as_bytes());
    let mut hash_str = format!("{:x}", hasher.finalize());

    if stretch {
        for _ in 0..2016 {
            let mut stretch_hasher = Md5::new();
            stretch_hasher.update(hash_str.as_bytes());
            hash_str = format!("{:x}", stretch_hasher.finalize());
        }
    }
    cache.insert(index, hash_str.clone());
    hash_str
}

fn find_triplet(hash: &str) -> Option<char> {
    let chars: Vec<char> = hash.chars().collect();
    for window in chars.windows(3) {
        if window[0] == window[1] && window[1] == window[2] {
            return Some(window[0]);
        }
    }
    None
}

fn contains_quituple(hash: &str, target: char) -> bool {
    let chars: Vec<char> = hash.chars().collect();
    for window in chars.windows(5) {
        if window.iter().all(|&c| c == target) {
            return true;
        }
    }
    false
}

fn find_64_index(salt: &str, stretch: bool) -> usize {
    let mut keys_found = 0;
    let mut index = 0;
    let mut hash_cache: HashMap<usize, String> = HashMap::new();

    loop {
        let hash = get_hash(salt, index, stretch, &mut hash_cache);

        if let Some(triplet_char) = find_triplet(&hash) {
            for next_index in (index + 1)..=(index + 1000) {
                let next_hash = get_hash(salt, next_index, stretch, &mut hash_cache);

                if contains_quituple(&next_hash, triplet_char) {
                    keys_found += 1;
                    if keys_found == 64 {
                        println!(
                            "Found key #{}: index {}, char '{}', hash: {}, next_hash_idx: {}, next_hash: {}",
                            keys_found, index, triplet_char, hash, next_index, next_hash
                        );
                        return index;
                    }
                    break;
                }
            }
        }
        index += 1;
    }
}

fn main() {
    let salt = match fs::read_to_string("../input.txt") {
        Ok(content) => content.trim().to_string(),
        Err(e) => {
            eprintln!("Error reading input file 'input.txt': {}", e);
            // Use a default salt for testing if file reading fails
            eprintln!("Using default salt 'abc' for testing.");
            "abc".to_string()
        }
    };
    let sep = "=".repeat(20);
    println!("\n{sep} Part 1 {sep}");
    let part1_index = find_64_index(&salt, false);
    println!("\nIndex of 64th key: {}", part1_index);

    println!("\n{sep} Part 2 {sep}");
    let part2_index = find_64_index(&salt, true);
    println!("\nIndex of 64th key: {}", part2_index);
}
