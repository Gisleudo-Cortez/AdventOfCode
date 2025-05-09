use std::fs;

fn read_diagram(path: &str) -> Vec<Vec<char>> {
    fs::read_to_string(path)
        .expect("Failed to read input")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_start(diagram: &[Vec<char>]) -> (usize, usize) {
    let row = 0;
    let col = diagram[0]
        .iter()
        .position(|&c| c == '|')
        .expect("Start not found");
    (row, col)
}

fn traverse_diagram(diagram: &[Vec<char>]) -> (String, usize) {
    let (mut row, mut col) = find_start(diagram);
    let mut dir = (1i32, 0i32); // down
    let mut letters = String::new();
    let mut steps = 0;

    while let Some(&c) = diagram.get(row).and_then(|r| r.get(col)) {
        if c == ' ' {
            break;
        }

        if c.is_ascii_alphabetic() {
            letters.push(c);
        } else if c == '+' {
            dir = find_new_direction(diagram, row, col, dir);
        }

        row = (row as i32 + dir.0) as usize;
        col = (col as i32 + dir.1) as usize;
        steps += 1;
    }

    (letters, steps)
}

fn find_new_direction(
    diagram: &[Vec<char>],
    row: usize,
    col: usize,
    (drow, dcol): (i32, i32),
) -> (i32, i32) {
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    for &(dr, dc) in &directions {
        if dr == -drow && dc == -dcol {
            continue;
        }

        let new_row = row as i32 + dr;
        let new_col = col as i32 + dc;

        if new_row < 0
            || new_col < 0
            || diagram
                .get(new_row as usize)
                .and_then(|r| r.get(new_col as usize))
                .is_none()
        {
            continue;
        }

        let ch = diagram[new_row as usize][new_col as usize];
        if ch != ' ' {
            return (dr, dc);
        }
    }

    panic!("No valid direction found at junction");
}

fn main() {
    let path = "../input.txt";
    let diagram = read_diagram(path);
    let (letters_seen, steps_taken) = traverse_diagram(&diagram);

    let sep = "=".repeat(20);
    println!("{sep} Part 1 {sep}");
    println!("The letters seen are: {}", letters_seen);

    println!("{sep} Part 2 {sep}");
    println!("The total number of steps is: {}", steps_taken);
}
