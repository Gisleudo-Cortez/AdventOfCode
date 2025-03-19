use std::{char, collections::HashMap, fs};

fn calculate_checksum(name: &str) -> String {
    let mut letter_counts: HashMap<char, i32> = HashMap::new();

    for ch in name.chars() {
        if ch.is_alphabetic() {
            *letter_counts.entry(ch).or_insert(0) += 1;
        }
    }
    let mut letter_vec: Vec<(char, i32)> = letter_counts.into_iter().collect();
    letter_vec.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    letter_vec.iter().map(|(ch, _)| *ch).take(5).collect()
}

fn check_valid_room(rooms: Vec<&str>) -> i32 {
    let mut valid_id_sum = 0;

    for line in rooms {
        let last_dash = line.rfind('-').unwrap();
        let name = &line[..last_dash];
        let id_checksum = &line[last_dash + 1..];

        if let Some((id_str, checksum)) = id_checksum.split_once('[') {
            let room_id = id_str.parse::<i32>().unwrap_or(0);
            let given_checksum = checksum.trim_end_matches(']');
            let calculated_checksum = calculate_checksum(name);

            if calculated_checksum == given_checksum {
                valid_id_sum += room_id;
            }
        }
    }
    valid_id_sum
}

fn decrypt_name(name: &str, sector_id: i32) -> String {
    name.chars()
        .map(|ch| {
            if ch == '-' {
                ' '
            } else if ch.is_alphabetic() {
                let shift = (ch as u8 - b'a' + (sector_id % 26) as u8) % 26;
                (b'a' + shift) as char
            } else {
                ch
            }
        })
        .collect()
}

fn find_north_pole_room(rooms: Vec<&str>) -> Option<i32> {
    for line in rooms {
        let last_dash = line.rfind('-').unwrap();
        let name = &line[..last_dash];
        let id_checksum = &line[last_dash + 1..];

        if let Some((id_str, _)) = id_checksum.split_once('[') {
            let room_id = id_str.parse::<i32>().unwrap_or(0);
            let decrypted_name = decrypt_name(name, room_id);

            if decrypted_name.contains("northpole") {
                println!(
                    "Found North Pole room: {} (ID: {})",
                    decrypted_name, room_id
                );
                return Some(room_id);
            }
        }
    }
    None
}

fn main() {
    let binding = fs::read_to_string("../input.txt").expect("Error reading input file");
    let data: Vec<&str> = binding.lines().collect();
    let sep = "=".repeat(20);
    println!("{sep} Part 1 {sep}");
    let part1 = check_valid_room(data.clone());
    println!("Sum of valid sector id: {part1}");

    println!("{sep} Part 2 {sep}");
    if let Some(north_id) = find_north_pole_room(data.clone()) {
        println!("North pole room id: {}", north_id);
    } else {
        println!("North Pole room not found");
    }
}
