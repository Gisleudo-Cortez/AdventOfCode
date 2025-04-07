use md5::{Digest, Md5};
use std::{collections::VecDeque, fs};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct State {
    x: usize,
    y: usize,
    path: String,
}

fn is_open(ch: char) -> bool {
    matches!(ch, 'b' | 'c' | 'd' | 'e' | 'f')
}

fn get_open_doors(passcode: &str, path: &str) -> [bool; 4] {
    let mut hasher = Md5::new();
    hasher.update(passcode.as_bytes());
    hasher.update(path.as_bytes());
    let result = format!("{:x}", hasher.finalize());
    let chars: Vec<char> = result.chars().collect();

    [
        is_open(chars[0]), // Up
        is_open(chars[1]), // Down
        is_open(chars[2]), // Left
        is_open(chars[3]), // Right
    ]
}

fn shortest_path(passcode: &str) -> Option<String> {
    let mut queue = VecDeque::new();
    queue.push_back(State {
        x: 0,
        y: 0,
        path: String::new(),
    });

    while let Some(state) = queue.pop_front() {
        // Check if reached the destination (bottom-right corner)
        if state.x == 3 && state.y == 3 {
            return Some(state.path);
        }

        let doors = get_open_doors(passcode, &state.path);

        // Try to move in all possible directions
        // Up
        if state.y > 0 && doors[0] {
            let mut new_path = state.path.clone();
            new_path.push('U');
            queue.push_back(State {
                x: state.x,
                y: state.y - 1,
                path: new_path,
            });
        }

        // Down
        if state.y < 3 && doors[1] {
            let mut new_path = state.path.clone();
            new_path.push('D');
            queue.push_back(State {
                x: state.x,
                y: state.y + 1,
                path: new_path,
            });
        }

        // Left
        if state.x > 0 && doors[2] {
            let mut new_path = state.path.clone();
            new_path.push('L');
            queue.push_back(State {
                x: state.x - 1,
                y: state.y,
                path: new_path,
            });
        }

        // Right
        if state.x < 3 && doors[3] {
            let mut new_path = state.path.clone();
            new_path.push('R');
            queue.push_back(State {
                x: state.x + 1,
                y: state.y,
                path: new_path,
            });
        }
    }

    None
}

fn longest_path(passcode: &str) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back(State {
        x: 0,
        y: 0,
        path: String::new(),
    });
    let mut longest = 0;

    while let Some(state) = queue.pop_front() {
        // Check if reached the destination (bottom-right corner)
        if state.x == 3 && state.y == 3 {
            longest = longest.max(state.path.len());
            continue;
        }

        let doors = get_open_doors(passcode, &state.path);

        // Try to move in all possible directions
        // Up
        if state.y > 0 && doors[0] {
            let mut new_path = state.path.clone();
            new_path.push('U');
            queue.push_back(State {
                x: state.x,
                y: state.y - 1,
                path: new_path,
            });
        }

        // Down
        if state.y < 3 && doors[1] {
            let mut new_path = state.path.clone();
            new_path.push('D');
            queue.push_back(State {
                x: state.x,
                y: state.y + 1,
                path: new_path,
            });
        }

        // Left
        if state.x > 0 && doors[2] {
            let mut new_path = state.path.clone();
            new_path.push('L');
            queue.push_back(State {
                x: state.x - 1,
                y: state.y,
                path: new_path,
            });
        }

        // Right
        if state.x < 3 && doors[3] {
            let mut new_path = state.path.clone();
            new_path.push('R');
            queue.push_back(State {
                x: state.x + 1,
                y: state.y,
                path: new_path,
            });
        }
    }

    longest
}

fn main() {
    let input = fs::read_to_string("../input.txt").expect("Error reading input file");
    let sep = "=".repeat(20);
    println!("{sep} Part 1 {sep}");
    let part1 = shortest_path(input.trim()).unwrap();
    println!("The shortest path to the vault is: {part1}");
    println!("{sep} Part 2 {sep}");
    let part2 = longest_path(input.trim());
    println!("The longest length to the vault is: {part2} steps");
}
