use std::fs;

fn parse_polymer(path: &str) -> String {
    fs::read_to_string(path)
        .expect("Error reading input file")
        .trim()
        .to_string()
}

fn react_polymer(polymer: &str) -> String {
    let mut stack: Vec<char> = Vec::new();
    for ch in polymer.chars() {
        if let Some(&last) = stack.last() {
            if last != ch && last.eq_ignore_ascii_case(&ch) {
                stack.pop();
                continue;
            }
        }
        stack.push(ch);
    }
    stack.iter().collect()
}

fn shortest_reacted_polymer(polymer: &str) -> usize {
    (b'a'..=b'z')
        .map(|unit| {
            let filtered: String = polymer
                .chars()
                .filter(|&ch| ch.to_ascii_lowercase() as u8 != unit)
                .collect();
            react_polymer(&filtered).len()
        })
        .min()
        .unwrap()
}

fn main() {
    let path = "../input.txt";
    let sep = "=".repeat(20);
    let polymer = parse_polymer(path);

    let part1_result = react_polymer(&polymer).len();
    let part2_result = shortest_reacted_polymer(&polymer);

    println!("{sep} Part 1 {sep}");
    println!("Remaining polymer length is {}", part1_result);
    println!("{sep} Part 2 {sep}");
    println!("Shortest possible polymer length is {}", part2_result);
}
