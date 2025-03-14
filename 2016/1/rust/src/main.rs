use std::{collections::HashMap, fs};

fn calculate_distance_from_origin(instructions: &Vec<&str>, pt_2: bool) -> i32 {
    //0 = north, 1 = east, 2 = south, 3 = west
    let mut current_pos = (0, 0);
    let total_directions = 4;
    let mut curr_direction = 0;
    let mut points_visited: HashMap<(i32, i32), i32> = HashMap::new();
    points_visited.insert(current_pos, 1);

    for step in instructions {
        let (turn, dist_str) = step.split_at(1);
        let distance: i32 = dist_str.parse().unwrap_or(0);

        match turn {
            "R" => curr_direction = (curr_direction + 1) % total_directions,
            "L" => curr_direction = (curr_direction + 3) % total_directions, // equivalent to -1 modulo 4
            _ => {
                println!("INVALID DIRECTION: {}", turn);
                continue;
            }
        }

        if pt_2 {
            for _ in 0..distance {
                match curr_direction {
                    0 => current_pos.1 += 1,
                    1 => current_pos.0 += 1,
                    2 => current_pos.1 -= 1,
                    3 => current_pos.0 -= 1,
                    _ => {}
                }
                if points_visited.contains_key(&current_pos) {
                    return current_pos.0.abs() + current_pos.1.abs();
                }
                points_visited.insert(current_pos, 1);
            }
        } else {
            match curr_direction {
                0 => current_pos.1 += distance,
                1 => current_pos.0 += distance,
                2 => current_pos.1 -= distance,
                3 => current_pos.0 -= distance,
                _ => {}
            }
        }
    }
    // Return Manhattan distance from origin
    current_pos.0.abs() + current_pos.1.abs()
}

fn main() {
    let binding = fs::read_to_string("../input.txt")
        .expect("Error reading input file")
        .replace(" ", "");
    let sep = "=".repeat(20);
    let instructions: Vec<&str> = binding.trim().split(",").collect();

    println!("{} Part 1 {}", sep, sep);
    let distance = calculate_distance_from_origin(&instructions, false);
    println!("The total distance from the origin is {}", distance);

    println!("{} Part 2 {}", sep, sep);
    let distance_2 = calculate_distance_from_origin(&instructions, true);
    println!(
        "The total distance to the first point visited twice is: {}",
        distance_2
    );
}
