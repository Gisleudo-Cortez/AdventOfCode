use std::collections::HashMap;
use std::fs;

fn parse_ids(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect("Error reading input file")
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn calculate_checksum(ids: &[String]) -> usize {
    let mut twos = 0;
    let mut threes = 0;

    for id in ids {
        let mut counts = HashMap::new();
        for ch in id.chars() {
            *counts.entry(ch).or_insert(0) += 1;
        }
        if counts.values().any(|&c| c == 2) {
            twos += 1;
        }
        if counts.values().any(|&c| c == 3) {
            threes += 1;
        }
    }

    twos * threes
}

fn find_prototype_boxes(ids: &[String]) -> String {
    for (i, a) in ids.iter().enumerate() {
        for b in ids.iter().skip(i + 1) {
            let mut diff_count = 0;
            let mut common = String::new();
            for (ac, bc) in a.chars().zip(b.chars()) {
                if ac == bc {
                    common.push(ac);
                } else {
                    diff_count += 1;
                }
                if diff_count > 1 {
                    break;
                }
            }
            if diff_count == 1 {
                return common;
            }
        }
    }
    String::new()
}

fn main() {
    let path = "../input.txt";
    let sep = "=".repeat(20);
    let ids = parse_ids(path);

    let part1_result = calculate_checksum(&ids);
    let part2_result = find_prototype_boxes(&ids);

    println!("{sep} Part 1 {sep}");
    println!("Checksum is {}", part1_result);
    println!("{sep} Part 2 {sep}");
    println!("Common letters are {}", part2_result);
}
