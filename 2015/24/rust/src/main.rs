use itertools::Itertools;
use std::fs;

fn calc_qe(group: &Vec<u64>) -> u64 {
    group.iter().product()
}

fn find_valid_groups(numbers: &Vec<u64>, target: u64, max_size: usize) -> Vec<Vec<u64>> {
    let mut valid_groups = Vec::new();

    for size in 1..=max_size {
        for group in numbers.iter().combinations(size) {
            let group: Vec<u64> = group.into_iter().copied().collect();
            if group.iter().sum::<u64>() == target {
                valid_groups.push(group);
            }
        }
        if !valid_groups.is_empty() {
            break;
        }
    }
    valid_groups
}

fn can_split_remaining(numbers: &Vec<u64>, target: u64, groups_left: usize) -> bool {
    if groups_left == 0 {
        return numbers.is_empty();
    }
    if numbers.is_empty() || numbers.iter().sum::<u64>() != target * groups_left as u64 {
        return false;
    }
    if groups_left == 1 {
        return numbers.iter().sum::<u64>() == target;
    }

    for size in 1..=numbers.len() {
        for group in numbers.iter().combinations(size) {
            let group_sum: u64 = group.iter().map(|&&x| x).sum();
            if group_sum == target {
                let remaining: Vec<u64> = numbers
                    .iter()
                    .cloned()
                    .filter(|x| !group.contains(&x))
                    .collect();
                if can_split_remaining(&remaining, target, groups_left - 1) {
                    return true;
                }
            }
        }
    }
    false
}

fn match_groups(presents: Vec<u64>, compartments: usize) -> Option<(u64, Vec<u64>)> {
    let total_weight: u64 = presents.iter().sum();
    if total_weight % compartments as u64 != 0 {
        return None;
    }
    let target_w = total_weight / compartments as u64;
    let max_group_size = presents.len() / compartments;

    let mut first_groups = find_valid_groups(&presents, target_w, max_group_size);
    first_groups.sort_by_key(|g| calc_qe(g));

    for g1 in &first_groups {
        let remaining: Vec<u64> = presents
            .iter()
            .cloned()
            .filter(|x| !g1.contains(x))
            .collect();
        if can_split_remaining(&remaining, target_w, compartments - 1) {
            return Some((calc_qe(g1), g1.clone()));
        }
    }
    None
}

fn main() {
    let data: Vec<u64> = fs::read_to_string("../input.txt")
        .expect("Failed to read file")
        .lines()
        .filter_map(|x| x.parse().ok())
        .collect();

    println!("==================== Part 1 ====================");
    if let Some((qe, group)) = match_groups(data.clone(), 3) {
        println!("QE: {}, First Group: {:?}", qe, group);
    }

    println!("==================== Part 2 ====================");
    if let Some((qe, group)) = match_groups(data, 4) {
        println!("QE: {}, First Group: {:?}", qe, group);
    }
}
