use std::collections::{HashMap, HashSet};
use std::fs;

fn parse_input(path: &str) -> (HashSet<i64>, HashMap<String, char>) {
    let input = fs::read_to_string(path).expect("Error reading input file");
    let mut lines = input.lines();
    let initial_state_line = lines.next().unwrap();
    let initial_state_str = initial_state_line.trim_start_matches("initial state: ");
    let mut state = HashSet::new();
    for (i, c) in initial_state_str.chars().enumerate() {
        if c == '#' {
            state.insert(i as i64);
        }
    }
    lines.next(); // skip empty line
    let mut rules = HashMap::new();
    for line in lines {
        let parts: Vec<&str> = line.split(" => ").collect();
        if parts.len() == 2 {
            rules.insert(parts[0].to_string(), parts[1].chars().next().unwrap());
        }
    }
    (state, rules)
}

fn next_generation(state: &HashSet<i64>, rules: &HashMap<String, char>) -> HashSet<i64> {
    let mut new_state = HashSet::new();
    let min = *state.iter().min().unwrap();
    let max = *state.iter().max().unwrap();
    for i in (min - 2)..=(max + 2) {
        let pattern: String = ((i - 2)..=(i + 2))
            .map(|j| if state.contains(&j) { '#' } else { '.' })
            .collect();
        if let Some(&'#') = rules.get(&pattern) {
            new_state.insert(i);
        }
    }
    new_state
}

fn sum_of_pots(state: &HashSet<i64>) -> i64 {
    state.iter().sum()
}

fn part1(state: &HashSet<i64>, rules: &HashMap<String, char>) -> i64 {
    let mut current_state = state.clone();
    for _ in 0..20 {
        current_state = next_generation(&current_state, rules);
    }
    sum_of_pots(&current_state)
}

fn part2(state: &HashSet<i64>, rules: &HashMap<String, char>) -> i64 {
    let mut current_state = state.clone();
    let mut prev_sum = sum_of_pots(&current_state);
    let mut prev_diff = 0;
    let mut same_diff_count = 0;
    let target = 50_000_000_000i64;
    for generation in 1.. {
        current_state = next_generation(&current_state, rules);
        let current_sum = sum_of_pots(&current_state);
        let diff = current_sum - prev_sum;
        if diff == prev_diff {
            same_diff_count += 1;
            if same_diff_count >= 100 {
                return current_sum + (target - generation) * diff;
            }
        } else {
            same_diff_count = 0;
            prev_diff = diff;
        }
        prev_sum = current_sum;
    }
    0
}

fn main() {
    let path = "../input.txt";
    let sep = "=".repeat(20);
    let (state, rules) = parse_input(path);

    println!("{sep} Part 1 {sep}");
    let result1 = part1(&state, &rules);
    println!("Sum of pot numbers after 20 generations: {result1}");

    println!("{sep} Part 2 {sep}");
    let result2 = part2(&state, &rules);
    println!("Sum of pot numbers after 50 billion generations: {result2}");
}
