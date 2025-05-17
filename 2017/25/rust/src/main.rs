use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Clone, Copy)]
enum MoveDir {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Action {
    write: u8,
    move_dir: MoveDir,
    next_state: String,
}

#[derive(Debug, Clone)]
struct State {
    actions: Vec<Action>,
}

fn parse_blueprint(path: &str) -> (String, usize, HashMap<String, State>) {
    let input = fs::read_to_string(path).expect("Error reading input file");
    let lines: Vec<_> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();

    let start_state = lines[0]
        .split_whitespace()
        .nth(3)
        .unwrap()
        .trim_end_matches('.')
        .to_string();
    let steps = lines[1]
        .split_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .next()
        .unwrap();

    let mut states = HashMap::new();
    let mut i = 2;
    while i < lines.len() {
        let state_name = lines[i]
            .split_whitespace()
            .nth(2)
            .unwrap()
            .trim_end_matches(':')
            .to_string();

        let mut actions = Vec::new();
        for j in 0..2 {
            let write = lines[i + 2 + j * 4]
                .trim()
                .split_whitespace()
                .last()
                .unwrap()
                .trim_end_matches('.')
                .parse::<u8>()
                .unwrap();
            let move_dir = match lines[i + 3 + j * 4]
                .trim()
                .split_whitespace()
                .last()
                .unwrap()
                .trim_end_matches('.')
            {
                "left" => MoveDir::Left,
                "right" => MoveDir::Right,
                _ => panic!("Invalid move direction"),
            };
            let next_state = lines[i + 4 + j * 4]
                .trim()
                .split_whitespace()
                .last()
                .unwrap()
                .trim_end_matches('.')
                .to_string();
            actions.push(Action {
                write,
                move_dir,
                next_state,
            });
        }

        states.insert(state_name.clone(), State { actions });
        i += 9;
    }

    (start_state, steps, states)
}

fn simulate_turing_machine(
    start_state: String,
    steps: usize,
    states: &HashMap<String, State>,
) -> usize {
    let mut tape = HashSet::new();
    let mut cursor: isize = 0;
    let mut current_state = start_state;

    for step in 0..steps {
        let current_val = tape.contains(&cursor) as usize;
        let state = states
            .get(&current_state)
            .unwrap_or_else(|| panic!("Missing state '{}' at step {}", current_state, step));
        let action = &state.actions[current_val];

        if action.write == 1 {
            tape.insert(cursor);
        } else {
            tape.remove(&cursor);
        }

        match action.move_dir {
            MoveDir::Left => cursor -= 1,
            MoveDir::Right => cursor += 1,
        }

        current_state = action.next_state.clone();
    }

    tape.len()
}

fn main() {
    let path = "../input.txt";
    let (start_state, steps, states) = parse_blueprint(path);
    let checksum = simulate_turing_machine(start_state, steps, &states);
    println!("Diagnostic checksum after {} steps is: {}", steps, checksum);
}
