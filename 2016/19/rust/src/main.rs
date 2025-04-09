use std::{collections::VecDeque, fs};

fn simulate(n: i32) -> i32 {
    let mut queue = VecDeque::from_iter(1..=n);
    while queue.len() > 1 {
        if let Some(current) = queue.pop_front() {
            let _ = queue.pop_front();
            queue.push_back(current);
        }
    }
    queue[0]
}

fn largest_power_of_3(n: i32) -> i32 {
    let mut power: i32 = 1;
    while let Some(next_power) = power.checked_mul(3) {
        if next_power <= n {
            power = next_power;
        } else {
            break;
        }
    }
    power
}

fn simulate_pt2(n: i32) -> i32 {
    let l = largest_power_of_3(n);
    if n == l {
        n
    } else if n <= 2 * l {
        return n - l;
    } else {
        return 2 * n - 3 * l;
    }
}

fn main() {
    let binding = fs::read_to_string("../input.txt").expect("Error reading input file");
    let input = binding.trim().parse::<i32>().unwrap();
    let sep = "=".repeat(20);
    println!("{sep} Part 1 {sep}");
    let part1 = simulate(input);
    println!("The elf with all the remaining presents is elf: {part1}");
    println!("{sep} Part 2 {sep}");
    let part2 = simulate_pt2(input);
    println!("The elf with all the remaining presents is elf: {part2}");
}
