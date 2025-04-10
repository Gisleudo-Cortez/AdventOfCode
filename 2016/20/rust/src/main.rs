use std::{cmp::max, fs};

fn min_valid_ip(mut blocked_ranges: Vec<(usize, usize)>) -> usize {
    // Possible ips range from 0 to the max 32 bit integers
    blocked_ranges.sort();
    let mut merged: Vec<(usize, usize)> = Vec::new();
    for (start, end) in blocked_ranges {
        if merged.is_empty() || start > merged[merged.len() - 1].1 + 1 {
            merged.push((start, end));
        } else {
            let last_index = merged.len() - 1;
            merged[last_index].1 = max(merged[last_index].1, end);
        }
    }
    let mut current_ip: usize = 0;
    for (start, end) in merged {
        if current_ip < start {
            return current_ip;
        } else {
            current_ip = max(current_ip, end + 1);
        }
    }
    current_ip
}

fn all_valid_ips(mut blocked_ranges: Vec<(usize, usize)>) -> usize {
    // Possible ips range from 0 to the max 32 bit integers
    blocked_ranges.sort();
    let mut merged: Vec<(usize, usize)> = Vec::new();
    for (start, end) in blocked_ranges {
        if merged.is_empty() || start > merged[merged.len() - 1].1 + 1 {
            merged.push((start, end));
        } else {
            let last_index = merged.len() - 1;
            merged[last_index].1 = max(merged[last_index].1, end);
        }
    }
    let mut current_ip: usize = 0;
    let mut allowed_ips: Vec<usize> = vec![];
    for (start, end) in merged {
        if current_ip < start {
            allowed_ips.push(current_ip);
        } else {
            current_ip = max(current_ip, end + 1);
        }
    }
    allowed_ips.len()
}

fn main() {
    let binding = fs::read_to_string("../input.txt").expect("Failed to read input file");
    let excluded_ips: Vec<(usize, usize)> = binding
        .lines()
        .map(|l| l.split('-'))
        .map(|mut parts| {
            let start = parts
                .next()
                .expect("Missing separator or start val")
                .trim()
                .parse::<usize>()
                .expect("Unable to parse start into usize");
            let end = parts
                .next()
                .expect("Missing separator or end val")
                .trim()
                .parse::<usize>()
                .expect("Unable to parse end into usize");
            (start, end)
        })
        .collect();
    let sep = "=".repeat(20);
    println!("{sep} Part 1 {sep}");
    let part1 = min_valid_ip(excluded_ips.clone());
    println!("The minimum ip allowed in the network is: {part1}");
    println!("{sep} Part 2 {sep}");
    let part2 = all_valid_ips(excluded_ips);
    println!("The number of allowed ips is: {part2}");
}
