use std::{collections::HashSet, fs};

fn parse_input(path: &str) -> Vec<Vec<String>> {
    fs::read_to_string(path)
        .expect("Error reading input file")
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn is_unique(phrase: Vec<String>) -> bool {
    let mut seen = HashSet::new();
    for word in phrase {
        if !seen.insert(word) {
            return false; // duplicate found
        }
    }
    true
}

/*
fn are_anagrams(a: &str, b: &str) -> bool {
    let mut ch1: Vec<char> = a.to_lowercase().chars().collect();
    let mut ch2: Vec<char> = b.to_lowercase().chars().collect();
    ch1.sort();
    ch2.sort();

    ch1 == ch2
}

fn is_anagram(phrase: Vec<String>) -> bool {
    for i in 0..phrase.len() {
        for j in (i + 1)..phrase.len() {
            if are_anagrams(&phrase[i], &phrase[j]) {
                return false; // invalidade passphrase, false to facilitate logic
            }
        }
    }
    true // there are no anagrams and the passphrase is valid
}
*/

fn is_anagram(phrase: Vec<String>) -> bool {
    let mut seen = HashSet::new();
    for word in phrase {
        let mut chars: Vec<char> = word.to_lowercase().chars().collect();
        chars.sort();
        let sorted = chars.into_iter().collect::<String>();
        if !seen.insert(sorted) {
            return false;
        }
    }
    true
}

fn solve_part1(path: &str) -> i32 {
    let mut count = 0;
    let list = parse_input(path);
    for phrase in list {
        if is_unique(phrase) {
            count += 1;
        }
    }
    count
}
/*
fn solve_part2(path: &str) -> i32 {
    let mut count = 0;
    let list = parse_input(path);
    for phrase in list {
        if is_unique(phrase.clone()) && is_anagram(phrase.clone()) {
            count += 1;
        }
    }
    count
}
*/

fn solve_part2(path: &str) -> usize {
    parse_input(path)
        .into_iter()
        .filter(|phrase| is_unique(phrase.clone()) && is_anagram(phrase.clone()))
        .count()
}

fn main() {
    let path = "../input.txt";
    let sep = "=".repeat(20);
    println!("{sep} Part 1 {sep}");
    let part1 = solve_part1(path);
    println!("The total of avaliable passphrases is: {part1}");
    println!("{sep} Part 2 {sep}");
    let part2 = solve_part2(path);
    println!("The total of avaliable passphrases is: {part2}");
}
