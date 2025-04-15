use std::{
    collections::{HashMap, VecDeque},
    fs,
};

use itertools::Itertools;

struct Point {
    letter: u8,
    x: usize,
    y: usize,
}

fn parse_input(path: &str) -> Vec<Vec<char>> {
    fs::read_to_string(path)
        .expect("Error parsing input file")
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}

fn find_points(grid: &[Vec<char>]) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    let not_points = ['.', '#'];
    for (ix, x) in grid.iter().enumerate() {
        for (iy, &y) in x.iter().enumerate() {
            if !not_points.contains(&y) {
                points.push(Point {
                    letter: (y as u8) - b'0',
                    x: ix,
                    y: iy,
                });
            }
        }
    }
    points
}

fn bfs(start: (usize, usize), grid: &[Vec<char>]) -> HashMap<(usize, usize), usize> {
    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back((start, 0));
    visited.insert(start, 0);

    while let Some(((x, y), dist)) = queue.pop_front() {
        for (nx, ny) in neighbors(x, y, grid.len(), grid[0].len()) {
            if grid[nx][ny] != '#' && !visited.contains_key(&(nx, ny)) {
                visited.insert((nx, ny), dist + 1);
                queue.push_back(((nx, ny), dist + 1));
            }
        }
    }
    visited
}

fn neighbors(x: usize, y: usize, rows: usize, cols: usize) -> Vec<(usize, usize)> {
    let mut n = Vec::new();
    if x > 0 {
        n.push((x - 1, y));
    }
    if y > 0 {
        n.push((x, y - 1));
    }
    if x + 1 < rows {
        n.push((x + 1, y));
    }
    if y + 1 < cols {
        n.push((x, y + 1));
    }
    n
}

fn solve(path: &str, return_to_start: bool) -> usize {
    let grid = parse_input(path);
    let points = find_points(&grid);
    let n = points.len();
    let mut dist = vec![vec![0; n]; n];
    for (i, from) in points.iter().enumerate() {
        let dists = bfs((from.x, from.y), &grid);
        for (j, to) in points.iter().enumerate() {
            if i != j {
                dist[i][j] = *dists.get(&(to.x, to.y)).unwrap();
            }
        }
    }
    let start_idx = points.iter().position(|p| p.letter == 0).unwrap();
    let other_indices: Vec<usize> = (0..n).filter(|&i| i != start_idx).collect();

    let mut min_total_dist = usize::MAX;

    for perm in other_indices.iter().permutations(other_indices.len()) {
        let mut total = 0;
        let mut prev = start_idx;

        for &next in perm {
            total += dist[prev][next];
            prev = next;
        }
        if return_to_start {
            total += dist[prev][start_idx];
        }
        min_total_dist = min_total_dist.min(total);
    }
    min_total_dist
}

fn main() {
    let path = "../input.txt";
    let sep = "=".repeat(20);
    println!("{sep} Part 1 {sep}");
    let part1 = solve(path, false);
    println!("Minimum distance is: {part1}");
    println!("{sep} Part 1 {sep}");
    let part2 = solve(path, true);
    println!("The minimun distance while returning to start is: {part2}");
}
