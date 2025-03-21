use std::{collections::HashMap, fs};

fn find_message(messages: Vec<&str>, max: bool) -> String {
    let num_cols = messages[0].len();
    let mut columns: Vec<HashMap<char, i32>> = vec![HashMap::new(); num_cols];

    for line in messages {
        if line.len() != num_cols {
            println!("Invalid line encountered exiting");
            return String::new();
        }
        for (i, ch) in line.chars().enumerate() {
            *columns[i].entry(ch).or_insert(0) += 1;
        }
    }
    if max {
        columns
            .into_iter()
            .filter_map(|col| {
                col.into_iter()
                    .max_by_key(|&(_ch, count)| count)
                    .map(|(ch, _)| ch)
            })
            .collect()
    } else {
        columns
            .into_iter()
            .filter_map(|col| {
                col.into_iter()
                    .min_by_key(|&(_ch, count)| count)
                    .map(|(ch, _)| ch)
            })
            .collect()
    }
}

fn main() {
    let binding = fs::read_to_string("../input.txt").expect("Error reading input file");
    let data: Vec<&str> = binding.lines().collect();
    let sep = "=".repeat(20);
    println!("{sep} Part 1 {sep}");
    let part1 = find_message(data.clone(), true);
    println!("The corrected most common message is: {part1}");
    let part2 = find_message(data, false);
    println!("{sep} Part 2 {sep}");
    println!("The correcter least common message is: {part2}");
}
