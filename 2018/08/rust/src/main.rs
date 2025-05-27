use std::fs;

fn parse_input(path: &str) -> Vec<usize> {
    fs::read_to_string(path)
        .expect("Error reading input file")
        .trim()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn parse_node(data: &mut impl Iterator<Item = usize>) -> (usize, usize) {
    let child_count = data.next().unwrap();
    let metadata_count = data.next().unwrap();

    let mut metadata_sum = 0;
    let mut child_values = Vec::new();

    for _ in 0..child_count {
        let (sum, value) = parse_node(data);
        metadata_sum += sum;
        child_values.push(value);
    }

    let mut node_value = 0;
    for _ in 0..metadata_count {
        let entry = data.next().unwrap();
        metadata_sum += entry;
        if child_count == 0 {
            node_value += entry;
        } else if entry > 0 && entry <= child_values.len() {
            node_value += child_values[entry - 1];
        }
    }

    (metadata_sum, node_value)
}

fn main() {
    let path = "../input.txt";
    let sep = "=".repeat(20);
    let data = parse_input(path);
    let (part1_result, part2_result) = parse_node(&mut data.into_iter());

    println!("{sep} Part 1 {sep}");
    println!("{}", part1_result);
    println!("{sep} Part 2 {sep}");
    println!("{}", part2_result);
}
