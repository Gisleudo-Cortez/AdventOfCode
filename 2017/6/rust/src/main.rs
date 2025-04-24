use std::{collections::HashMap, fs};

fn parse_input(path: &str) -> Vec<usize> {
    fs::read_to_string(path)
        .expect("Failed to read input file")
        .split_whitespace()
        .map(|n| n.parse::<usize>().expect("Invalid number"))
        .collect()
}

fn redistribute(banks: &mut [usize]) {
    if banks.is_empty() {
        return;
    }

    let (mut idx, &blocks) = banks
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.cmp(b.1).then_with(|| b.0.cmp(&a.0)))
        .unwrap();

    banks[idx] = 0;
    let len = banks.len();

    for _ in 0..blocks {
        idx = (idx + 1) % len;
        banks[idx] += 1;
    }
}

/// Returns (cycles_until_repeat, size_of_loop).
fn solve(path: &str) -> (usize, usize) {
    let mut banks = parse_input(path);
    let mut step = 0;
    let mut seen: HashMap<Vec<usize>, usize> = HashMap::new();

    while let std::collections::hash_map::Entry::Vacant(entry) = seen.entry(banks.clone()) {
        entry.insert(step);
        redistribute(&mut banks);
        step += 1;
    }

    let first_seen_at = seen[&banks];
    (step, step - first_seen_at)
}

fn main() {
    let (part1, part2) = solve("../input.txt");
    println!("The number of cycles until repeat is: {}", part1);
    println!("The number of cycles is: {}", part2);
}
