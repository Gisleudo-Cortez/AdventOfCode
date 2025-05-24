use std::collections::{HashMap, HashSet};
use std::fs;

fn parse_coordinates(path: &str) -> Vec<(i32, i32)> {
    fs::read_to_string(path)
        .expect("Error reading input file")
        .lines()
        .map(|line| {
            let parts: Vec<i32> = line.split(',').map(|s| s.trim().parse().unwrap()).collect();
            (parts[0], parts[1])
        })
        .collect()
}

fn manhattan(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn compute_areas(coords: &[(i32, i32)], distance_limit: i32) -> (usize, usize) {
    let min_x = coords.iter().map(|c| c.0).min().unwrap();
    let max_x = coords.iter().map(|c| c.0).max().unwrap();
    let min_y = coords.iter().map(|c| c.1).min().unwrap();
    let max_y = coords.iter().map(|c| c.1).max().unwrap();

    let mut area_map = HashMap::new();
    let mut infinite = HashSet::new();
    let mut safe_region = 0;

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let mut distances: Vec<_> = coords
                .iter()
                .enumerate()
                .map(|(i, &c)| (i, manhattan((x, y), c)))
                .collect();
            distances.sort_by_key(|&(_, d)| d);

            if distances[0].1 != distances[1].1 {
                area_map
                    .entry(distances[0].0)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);

                if x == min_x || x == max_x || y == min_y || y == max_y {
                    infinite.insert(distances[0].0);
                }
            }

            let total_distance: i32 = coords.iter().map(|&c| manhattan((x, y), c)).sum();
            if total_distance < distance_limit {
                safe_region += 1;
            }
        }
    }

    let largest_finite_area = area_map
        .into_iter()
        .filter(|(i, _)| !infinite.contains(i))
        .map(|(_, v)| v)
        .max()
        .unwrap_or(0);

    (largest_finite_area, safe_region)
}

fn main() {
    let path = "../input.txt";
    let sep = "=".repeat(20);
    let coords = parse_coordinates(path);

    let (part1_result, part2_result) = compute_areas(&coords, 10000);

    println!("{sep} Part 1 {sep}");
    println!("Largest finite area = {}", part1_result);
    println!("{sep} Part 2 {sep}");
    println!("Size of safe region = {}", part2_result);
}
