use std::cmp::{max, min};
use std::fs;

#[derive(Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

fn parse_line(line: &str) -> Point {
    let nums: Vec<i64> = line
        .split(|c| c == '<' || c == '>' || c == ',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    Point {
        x: nums[0],
        y: nums[1],
        vx: nums[2],
        vy: nums[3],
    }
}

fn advance(points: &mut [Point]) {
    for p in points.iter_mut() {
        p.x += p.vx;
        p.y += p.vy;
    }
}

fn revert(points: &mut [Point]) {
    for p in points.iter_mut() {
        p.x -= p.vx;
        p.y -= p.vy;
    }
}

fn area(points: &[Point]) -> i64 {
    let (min_x, max_x, min_y, max_y) = bounds(points);
    (max_x - min_x) * (max_y - min_y)
}

// Determines the bounding box of the points
fn bounds(points: &[Point]) -> (i64, i64, i64, i64) {
    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;
    for p in points {
        min_x = min(min_x, p.x);
        max_x = max(max_x, p.x);
        min_y = min(min_y, p.y);
        max_y = max(max_y, p.y);
    }
    (min_x, max_x, min_y, max_y)
}

fn render(points: &[Point]) -> String {
    let (min_x, max_x, min_y, max_y) = bounds(points);
    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;
    let mut grid = vec![vec![' '; width]; height];
    for p in points {
        let x = (p.x - min_x) as usize;
        let y = (p.y - min_y) as usize;
        grid[y][x] = '#';
    }
    grid.iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    let path = "../input.txt";
    let sep = "=".repeat(20);
    let input = fs::read_to_string(path).expect("Error reading input file");
    let mut points: Vec<Point> = input.lines().map(parse_line).collect();

    let mut time = 0;
    let mut prev_area = area(&points);
    loop {
        advance(&mut points);
        time += 1;
        let curr_area = area(&points);
        if curr_area > prev_area {
            revert(&mut points);
            time -= 1;
            break;
        }
        prev_area = curr_area;
    }

    println!("{sep} Part 1 {sep}");
    println!("Message formed by the points:\n{}", render(&points));
    println!("{sep} Part 2 {sep}");
    println!("Time taken for alignment: {} seconds", time);
}
