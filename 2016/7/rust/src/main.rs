use std::{char, fs};

fn has_abba(ip: &str) -> bool {
    ip.as_bytes()
        .windows(4)
        .any(|c| c[0] != c[1] && c[0] == c[3] && c[1] == c[2])
}

fn supports_tls(ip: &str) -> bool {
    let mut inside_brackets = false;
    let mut has_abba_outside = false;
    let mut has_abba_inside = false;

    let mut segment = String::new();

    for c in ip.chars().chain(Some('[')) {
        match c {
            '[' | ']' => {
                if inside_brackets {
                    has_abba_inside |= has_abba(&segment);
                } else {
                    has_abba_outside |= has_abba(&segment);
                }
                segment.clear();
                inside_brackets = c == '[';
            }
            _ => segment.push(c),
        }
    }
    has_abba_outside && !has_abba_inside
}

fn find_aba(ip: &str) -> Vec<(char, char)> {
    ip.as_bytes()
        .windows(3)
        .filter_map(|c| {
            if c[0] == c[2] && c[0] != c[1] {
                Some((c[0] as char, c[1] as char))
            } else {
                None
            }
        })
        .collect()
}

fn support_ssl(ip: &str) -> bool {
    let mut inside = Vec::new();
    let mut outside = Vec::new();
    let mut segment = String::new();
    let mut inside_brackets = false;

    for c in ip.chars().chain(Some('[')) {
        match c {
            '[' | ']' => {
                if inside_brackets {
                    inside.push(segment.clone());
                } else {
                    outside.push(segment.clone());
                }
                segment.clear();
                inside_brackets = c == '[';
            }
            _ => segment.push(c),
        }
    }
    let aba_patterns: Vec<(char, char)> = outside.iter().flat_map(|s| find_aba(s)).collect();
    for (a, b) in aba_patterns {
        let bab = format!("{}{}{}", b, a, b);
        if inside.iter().any(|s| s.contains(&bab)) {
            return true;
        }
    }
    false
}

fn main() {
    let binding = fs::read_to_string("../input.txt").expect("Error reading input file");
    let data: Vec<&str> = binding.lines().collect();
    let sep = "=".repeat(20);
    println!("{sep} Part 1 {sep}");
    let part1 = data.iter().filter(|line| supports_tls(line)).count();
    println!("The total count of valid ips is: {}", part1);
    println!("{sep} Part 2 {sep}");
    let part2 = data.iter().filter(|line| support_ssl(line)).count();
    println!("The total count of SSL-supporting ips is: {}", part2);
}
