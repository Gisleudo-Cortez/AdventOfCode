use std::collections::HashMap;
use std::fs;

#[derive(Clone, Copy, PartialEq, Eq)]
enum NodeState {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_left(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn reverse(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

fn parse_input(path: &str) -> HashMap<(i32, i32), NodeState> {
    let input = fs::read_to_string(path).expect("Error reading input file");
    let mut grid = HashMap::new();
    let lines: Vec<_> = input.lines().collect();
    let offset = (lines.len() / 2) as i32;

    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                grid.insert((x as i32 - offset, y as i32 - offset), NodeState::Infected);
            }
        }
    }
    grid
}

fn simulate(mut grid: HashMap<(i32, i32), NodeState>, bursts: usize, part2: bool) -> usize {
    let mut pos = (0, 0);
    let mut dir = Direction::Up;
    let mut infections = 0;

    for _ in 0..bursts {
        let state = *grid.get(&pos).unwrap_or(&NodeState::Clean);

        match state {
            NodeState::Clean => {
                dir = dir.turn_left();
                grid.insert(
                    pos,
                    if part2 {
                        NodeState::Weakened
                    } else {
                        NodeState::Infected
                    },
                );
                if !part2 {
                    infections += 1;
                }
            }
            NodeState::Weakened => {
                grid.insert(pos, NodeState::Infected);
                infections += 1;
            }
            NodeState::Infected => {
                dir = dir.turn_right();
                grid.insert(
                    pos,
                    if part2 {
                        NodeState::Flagged
                    } else {
                        NodeState::Clean
                    },
                );
            }
            NodeState::Flagged => {
                dir = dir.reverse();
                grid.insert(pos, NodeState::Clean);
            }
        }

        pos = match dir {
            Direction::Up => (pos.0, pos.1 - 1),
            Direction::Down => (pos.0, pos.1 + 1),
            Direction::Left => (pos.0 - 1, pos.1),
            Direction::Right => (pos.0 + 1, pos.1),
        };
    }

    infections
}

fn main() {
    let path = "../input.txt";
    let sep = "=".repeat(20);
    let grid = parse_input(path);

    println!("{sep} Part 1 {sep}");
    let part1_result = simulate(grid.clone(), 10_000, false);
    println!("Infections after 10,000 bursts : {}", part1_result);
    println!("{sep} Part 2 {sep}");
    let part2_result = simulate(grid, 10_000_000, true);
    println!("Infections after 10,000,000 bursts : {}", part2_result);
}
