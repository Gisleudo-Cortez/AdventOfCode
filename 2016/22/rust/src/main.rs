use std::collections::{HashSet, VecDeque};
use std::fs;

#[derive(Debug)]
struct Node {
    x: usize,
    y: usize,
    size: u32,
    used: u32,
    avail: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Goal,
    Data,
}

fn parse_input(input_path: &str) -> Vec<Node> {
    let input = fs::read_to_string(input_path).expect("Error parsing input file");
    let mut nodes: Vec<Node> = Vec::with_capacity(input.len() - 2);
    for line in input.lines().skip(2) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let coordinates = parts[0];
        let coord_parts: Vec<&str> = coordinates
            .split(['-', 'x', 'y'])
            .filter(|s| !s.is_empty())
            .collect();
        let x = coord_parts[1].parse::<usize>().unwrap();
        let y = coord_parts[2].parse::<usize>().unwrap();
        let size = parts[1].trim_end_matches('T').parse::<u32>().unwrap();
        let used = parts[2].trim_end_matches('T').parse::<u32>().unwrap();
        let avail = parts[3].trim_end_matches('T').parse::<u32>().unwrap();

        nodes.push(Node {
            x,
            y,
            size,
            used,
            avail,
        });
    }
    nodes
}

fn count_viable_pairs(nodes: Vec<Node>) -> u32 {
    let mut count: u32 = 0;
    let mut avails: Vec<&Node> = nodes.iter().collect();
    avails.sort_unstable_by_key(|n| n.avail);
    for a in nodes.iter().filter(|n| n.used > 0) {
        let idx = avails.partition_point(|b| b.avail < a.used);
        for b in &avails[idx..] {
            if a.x == b.x && a.y == b.y {
                continue;
            }
            count += 1;
        }
    }
    count
}

fn classify_node(n: &Node, goal_pos: (usize, usize)) -> Tile {
    if n.used == 0 {
        Tile::Empty
    } else if (n.x, n.y) == goal_pos {
        Tile::Goal
    } else if n.used > 100 {
        Tile::Wall
    } else {
        Tile::Data
    }
}

fn build_grid(nodes: &[Node]) -> (Vec<Vec<Tile>>, (usize, usize), (usize, usize)) {
    let max_x = nodes.iter().map(|n| n.x).max().unwrap();
    let max_y = nodes.iter().map(|n| n.y).max().unwrap();
    let goal_pos = (max_x, 0);
    let mut empty_pos = (0, 0);
    let mut grid = vec![vec![Tile::Data; max_x + 1]; max_y + 1];

    for node in nodes {
        let tile = classify_node(node, goal_pos);
        if tile == Tile::Empty {
            empty_pos = (node.x, node.y);
        }
        grid[node.y][node.x] = tile;
    }
    (grid, empty_pos, goal_pos)
}

fn neighbors((x, y): (usize, usize), width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut result = Vec::with_capacity(4);
    if x > 0 {
        result.push((x - 1, y));
    }
    if x + 1 < width {
        result.push((x + 1, y));
    }
    if y > 0 {
        result.push((x, y - 1));
    }
    if y + 1 < height {
        result.push((x, y + 1));
    }
    result
}

fn bfs(grid: &[Vec<Tile>], start: (usize, usize), goal: (usize, usize)) -> Option<u32> {
    let height = grid.len();
    let width = grid[0].len();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((start, 0)); // (position, steps)
    visited.insert(start);

    while let Some(((x, y), steps)) = queue.pop_front() {
        if (x, y) == goal {
            return Some(steps);
        }

        for (nx, ny) in neighbors((x, y), width, height) {
            if visited.contains(&(nx, ny)) {
                continue;
            }
            if grid[ny][nx] == Tile::Wall {
                continue;
            }
            queue.push_back(((nx, ny), steps + 1));
            visited.insert((nx, ny));
        }
    }

    None
}

fn solve_pt1(path: &str) -> u32 {
    let data = parse_input(path);
    count_viable_pairs(data)
}

fn solve_pt2(path: &str) -> i32 {
    let nodes = parse_input(path);
    let (grid, empty, goal) = build_grid(&nodes);

    let target = (goal.0 - 1, goal.1); // move empty left of goal
    let mut total_steps = bfs(&grid, empty, target).expect("No path to goal-adjacent") as i32;
    total_steps += 5 * (goal.0 as i32 - 1) + 1;
    total_steps
}

fn main() {
    let path = "../input.txt";
    let sep = "=".repeat(20);
    println!("{sep} Part 1 {sep}");
    let part1 = solve_pt1(path);
    println!("The total of viable pairs is: {part1}");
    println!("{sep} Part 2 {sep}");
    let part2 = solve_pt2(path);
    println!("Steps to move goal data to (0, 0): {part2}");
}
