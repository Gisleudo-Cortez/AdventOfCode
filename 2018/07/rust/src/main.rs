use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

fn parse_dependencies(path: &str) -> HashMap<char, HashSet<char>> {
    let mut deps: HashMap<char, HashSet<char>> = HashMap::new();
    let input = fs::read_to_string(path).expect("Error reading input file");
    for line in input.lines() {
        let from = line.chars().nth(5).unwrap();
        let to = line.chars().nth(36).unwrap();
        deps.entry(to).or_default().insert(from);
        deps.entry(from).or_default();
    }
    deps
}

fn determine_order(mut deps: HashMap<char, HashSet<char>>) -> String {
    let mut ready: BinaryHeap<_> = deps
        .iter()
        .filter(|(_, v)| v.is_empty())
        .map(|(&k, _)| std::cmp::Reverse(k))
        .collect();
    let mut result = String::new();

    while let Some(std::cmp::Reverse(ch)) = ready.pop() {
        result.push(ch);
        for (_step, pre) in deps.iter_mut() {
            pre.remove(&ch);
        }
        for (&step, pre) in &deps {
            if pre.is_empty()
                && !result.contains(step)
                && !ready.iter().any(|&std::cmp::Reverse(x)| x == step)
            {
                ready.push(std::cmp::Reverse(step));
            }
        }
    }

    result
}

fn calculate_time(
    mut deps: HashMap<char, HashSet<char>>,
    workers: usize,
    base_duration: usize,
) -> usize {
    let mut in_progress: Vec<(usize, char)> = vec![];
    let mut available = BinaryHeap::new();
    let mut time = 0;
    let mut completed = HashSet::new();

    for (&step, prereqs) in &deps {
        if prereqs.is_empty() {
            available.push(std::cmp::Reverse(step));
        }
    }

    loop {
        while in_progress.len() < workers {
            if let Some(std::cmp::Reverse(step)) = available.pop() {
                let duration = base_duration + (step as u8 - b'A' + 1) as usize;
                in_progress.push((time + duration, step));
            } else {
                break;
            }
        }

        if in_progress.is_empty() {
            break;
        }

        in_progress.sort();
        let (next_time, finished_step) = in_progress.remove(0);
        time = next_time;
        completed.insert(finished_step);

        for (step, prereqs) in deps.iter_mut() {
            prereqs.remove(&finished_step);
            if prereqs.is_empty()
                && !completed.contains(step)
                && !in_progress.iter().any(|&(_, s)| s == *step)
                && !available.iter().any(|&std::cmp::Reverse(s)| s == *step)
            {
                available.push(std::cmp::Reverse(*step));
            }
        }
    }

    time
}

fn main() {
    let path = "../input.txt";
    let sep = "=".repeat(20);
    let deps = parse_dependencies(path);

    let part1_result = determine_order(deps.clone());
    let part2_result = calculate_time(deps, 5, 60);

    println!("{sep} Part 1 {sep}");
    println!("{}", part1_result);
    println!("{sep} Part 2 {sep}");
    println!("{}", part2_result);
}
