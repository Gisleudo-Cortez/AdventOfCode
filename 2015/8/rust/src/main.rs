use std::{fs, i32};

fn calculate_parsed_length(raw: &str) -> i32 {
    let mut length: i32 = 0;
    let mut chars = raw.trim_matches('"').chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(&next) = chars.peek(){
                if next == '\\' || next == '"' {
                    length += 1;
                    chars.next();
                    continue;
                } else if next == 'x'{
                    if chars.next().is_some() && chars.next().is_some() {
                        length += 1;
                    }
                    continue;
                }
            }
        }
        length += 1;
    }
    length
}
// Why this does't work????
fn calculate_encoded_length(raw: &str) -> i32 {
    let mut length: i32= 2;
    for c in raw.chars(){
        match c {
            '"' | '\\' => length += 2,
            _ => length += 1,
        }
    }
    length
}

fn calculate_raw_and_parsed(data: Vec<String>) -> (i32,i32) {
    let mut total_raw: i32 = 0;
    let mut total_parsed: i32 = 0;

    for raw_string in data {
        let  raw_length = raw_string.len() as i32;
        let  parsed_length = calculate_parsed_length(&raw_string);
        total_parsed += parsed_length;
        total_raw += raw_length;
    }
    (total_raw, total_parsed)
}

fn calculate_raw_and_encoded(data: Vec<String>) -> (i32,i32) {
    let mut total_raw: i32 = 0;
    let mut total_encoded: i32 = 0;

    for raw_string in data {
        let raw_length = raw_string.len() as i32;
        let  encoded_length = calculate_encoded_length(&raw_string);
        total_raw += raw_length;
        total_encoded += encoded_length;
    }
    (total_raw, total_encoded)
}

fn main() {
    let data: Vec<String> = fs::read_to_string("../input.txt")
    .expect("Error reading file")
    .lines()
    .map(String::from)
    .collect();

    let (raw, parsed) = calculate_raw_and_parsed(data.clone());
    println!("Part 1");
    println!("Total Raw length: {}", raw);
    println!("Total Parsed length: {}", parsed);
    println!("Difference: {}\n", raw - parsed);

    let (raw_2, encoded) = calculate_raw_and_encoded(data);
    println!("Part 2");
    println!("Total Raw length: {}", raw_2);
    println!("Total Parsed length: {}", encoded);
    println!("Difference: {}", encoded - raw_2);
}

// use std::fs;

// fn main() {
//     let input = fs::read_to_string("input.txt").expect("Failed to read input file");
//     let part1 = input.lines().map(|line| line.len() - memory_length(line)).sum::<usize>();
//     let part2 = input.lines().map(|line| encoded_length(line) - line.len()).sum::<usize>();
    
//     println!("Part 1: {}", part1);
//     println!("Part 2: {}", part2);
// }

// fn memory_length(s: &str) -> usize {
//     let mut chars = s.chars();
//     let mut count = 0;
    
//     assert_eq!(chars.next(), Some('"'));
//     while let Some(c) = chars.next() {
//         match c {
//             '\\' => match chars.next() {
//                 Some('"') | Some('\\') => count += 1,
//                 Some('x') => {
//                     chars.next();
//                     chars.next();
//                     count += 1;
//                 }
//                 _ => unreachable!(),
//             },
//             '"' => break,
//             _ => count += 1,
//         }
//     }
//     count
// }

// fn encoded_length(s: &str) -> usize {
//     let mut count = 2; // Opening and closing quotes
//     for c in s.chars() {
//         match c {
//             '"' | '\\' => count += 2,
//             _ => count += 1,
//         }
//     }
//     count
// }
