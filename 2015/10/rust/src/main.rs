use std::fs;


fn solve(data: &str, n_iter: usize) -> usize {
    let mut text = data.to_string();

    for _ in 0..n_iter {
        let mut out_str = String::new();
        let mut chars = text.chars().peekable();
        let mut count = 1;

        let mut prev = chars.next().unwrap();

        while let Some(&next) = chars.peek() {
            if next == prev {
                count += 1;
            } else {
                out_str.push_str(&count.to_string());
                out_str.push(prev);
                count = 1;
            }
            prev = chars.next().unwrap();
        }
        out_str.push_str(&count.to_string());
        out_str.push(prev);
        text = out_str;
    }
    text.len()
}

fn main() {
    let data = fs::read_to_string("../input.txt").expect("Failed to read").trim().to_string();
    let sep = "=".repeat(20);
    println!("{} Part 1 {}", sep, sep);
    let p1 = solve(&data, 40);
    println!("{}",p1);
    println!("{} Part 2 {}", sep, sep);
    let p2 = solve(&data, 50);
    println!("{}",p2);
}
