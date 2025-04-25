use std::fs;

/*
fn solve_pt1(captha: String) -> i32 {
    let captha = captha.trim();
    let chars: Vec<char> = captha.chars().collect();
    let len = chars.len();
    let mut sum = 0;
    for i in 0..len {
        let next = (i + 1) % len;
        if chars[i] == chars[next] {
            if let Some(digit) = chars[i].to_digit(10) {
                sum += digit as i32;
            }
        }
    }
    sum
}

fn solve_pt2(captha: String) -> i32 {
    let captha = captha.trim();
    let chars: Vec<char> = captha.chars().collect();
    let len = chars.len();
    let mut sum = 0;
    for i in 0..len {
        let next = (i + len / 2) % len;
        if chars[i] == chars[next] {
            if let Some(digit) = chars[i].to_digit(10) {
                sum += digit as i32;
            }
        }
    }
    sum
}
*/
// refactor

fn solve(captha: &str, offset: usize) -> i32 {
    let chars: Vec<char> = captha.trim().chars().collect();
    let len = chars.len();

    chars.iter().enumerate().fold(0, |sum, (i, &ch)| {
        let next = chars[(i + offset) % len];
        if ch == next {
            sum + ch.to_digit(10).unwrap_or(0) as i32
        } else {
            sum
        }
    })
}

fn main() {
    let input = fs::read_to_string("../input.txt").expect("Error reading input file");
    let sep = "=".repeat(20);
    println!("{sep} Part 1 {sep}");
    let part1 = solve(&input, 1);
    println!("The solution for the captha is: {part1}");
    println!("{sep} Part 2 {sep}");
    let part2 = solve(&input, input.trim().len() / 2);
    println!("The solution for the captha is: {part2}");
}
