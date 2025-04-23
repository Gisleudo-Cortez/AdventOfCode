use std::fs;

fn parse_input(path: &str) -> Vec<i32> {
    fs::read_to_string(path)
        .expect("error reading input file")
        .lines()
        .map(|l| l.trim().parse::<i32>().unwrap())
        .collect()
}

fn solve_part1(path: &str) -> usize {
    let mut count: usize = 1;
    let mut instructions = parse_input(path);
    let len = instructions.len();
    let mut i: i32 = 0;
    loop {
        let offset = instructions[i as usize];
        if (i + offset) >= len as i32 {
            return count;
        } else {
            count += 1;
            instructions[i as usize] += 1;
            i += offset;
        }
    }
}

fn solve_part2(path: &str) -> usize {
    let mut count: usize = 1;
    let mut instructions = parse_input(path);
    let len = instructions.len();
    let mut i: i32 = 0;
    loop {
        let offset = instructions[i as usize];
        if (i + offset) >= len as i32 {
            return count;
        } else {
            count += 1;
            if offset < 3 {
                instructions[i as usize] += 1;
            } else {
                instructions[i as usize] -= 1;
            }
            i += offset;
        }
    }
}

fn main() {
    let sep = "=".repeat(20);
    let path = "../input.txt";
    println!("{sep} Part 1 {sep}");
    let part1 = solve_part1(path);
    println!("The number of steps needed to escape is: {part1}");
    println!("{sep} Part 2 {sep}");
    let part2 = solve_part2(path);
    println!("The number of steps needed to escape is: {part2}");
}
