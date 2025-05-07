use std::fs;

fn spin_lock(steps: usize, cycles: usize, after_val: usize) -> usize {
    let mut buffer = vec![0];
    let mut curr_idx = 0;

    for val in 1..=cycles {
        curr_idx = (curr_idx + steps) % buffer.len() + 1;
        buffer.insert(curr_idx, val);
    }

    let pos = buffer.iter().position(|&x| x == after_val).unwrap();
    buffer[(pos + 1) % buffer.len()]
}

fn spin_lock_2(steps: usize, cycles: usize) -> usize {
    let mut after_0 = 0;
    let mut curr_idx = 0;

    for val in 1..=cycles {
        curr_idx = (curr_idx + steps) % val + 1;
        if curr_idx == 1 {
            after_0 = val;
        }
    }

    after_0
}

fn main() {
    let input = fs::read_to_string("../input.txt")
        .expect("Error reading input file")
        .trim()
        .parse::<usize>()
        .unwrap();

    let sep = "=".repeat(20);
    println!("{sep} Part 1 {sep}");
    let pt1 = spin_lock(input, 2017, 2017);
    println!("The value after 2017 is: {pt1}");

    println!("{sep} Part 2 {sep}");
    let pt2 = spin_lock_2(input, 50_000_000);
    println!("The value after 0 is: {pt2}");
}
