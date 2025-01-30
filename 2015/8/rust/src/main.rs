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
