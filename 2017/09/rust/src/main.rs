use std::fs;

fn solve(path: &str) -> (usize, usize) {
    let input = fs::read_to_string(path)
        .expect("Error reading input file")
        .to_string();
    let mut chars = input.chars();

    let mut inside_garbage = false;
    let mut depth = 0;
    let mut score = 0;
    let mut garbage_count = 0;

    while let Some(c) = chars.next() {
        if inside_garbage {
            match c {
                '!' => {
                    chars.next();
                }
                '>' => {
                    inside_garbage = false;
                }
                _ => {
                    garbage_count += 1;
                }
            }
        } else {
            match c {
                '{' => {
                    depth += 1;
                }
                '}' => {
                    score += depth;
                    depth -= 1;
                }
                '<' => {
                    inside_garbage = true;
                }
                ',' => {}
                _ => {}
            }
        }
    }
    (score, garbage_count)
}

fn main() {
    let path = "../input.txt";
    let sep = "=".repeat(20);
    let (p1, p2) = solve(path);
    println!("{sep} Part 1 {sep}");
    println!("The total score for all groups is: {p1}");
    println!("{sep} Part 2 {sep}");
    println!("The total non canceled characters within the garbage is: {p2}");
}
