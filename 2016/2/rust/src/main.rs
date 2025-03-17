use std::{collections::HashMap, fs, usize};

fn find_code(instructions: Vec<&str>, previous_position: (usize, usize)) -> String {
    let numpad = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let r = numpad.len();
    let c = numpad[0].len();
    let mut code = String::from("");
    let mut current_pos = previous_position;

    for instruction in instructions {
        for mv in instruction.chars() {
            match mv {
                'U' => {
                    if current_pos.0 == 0 {
                        //println!("Already at the top of numpad keeping position");
                        continue;
                    } else {
                        current_pos.0 -= 1;
                    }
                }
                'D' => {
                    if current_pos.0 == r - 1 {
                        //println!("Already at the bottom of numpad keeping position");
                        continue;
                    } else {
                        current_pos.0 += 1;
                    }
                }
                'L' => {
                    if current_pos.1 == 0 {
                        //println!("Already at the left most side of numpad keeping position");
                        continue;
                    } else {
                        current_pos.1 -= 1;
                    }
                }
                'R' => {
                    if current_pos.1 == c - 1 {
                        //println!("Already at the right most side of numpad keeping position");
                        continue;
                    } else {
                        current_pos.1 += 1;
                    }
                }
                _ => {
                    println!("Invalid movement: {mv}");
                    break;
                }
            }
        }
        code.push_str(&numpad[current_pos.0][current_pos.1].to_string());
    }
    code
}

fn find_code_2(instructions: Vec<&str>, previous_position: (usize, usize)) -> String {
    let numpad: HashMap<(usize, usize), String> = {
        let mut map = HashMap::new();
        map.insert((0, 2), "1".to_string());
        map.insert((1, 1), "2".to_string());
        map.insert((1, 2), "3".to_string());
        map.insert((1, 3), "4".to_string());
        map.insert((2, 0), "5".to_string());
        map.insert((2, 1), "6".to_string());
        map.insert((2, 2), "7".to_string());
        map.insert((2, 3), "8".to_string());
        map.insert((2, 4), "9".to_string());
        map.insert((3, 1), "A".to_string());
        map.insert((3, 2), "B".to_string());
        map.insert((3, 3), "C".to_string());
        map.insert((4, 2), "D".to_string());
        map
    };

    let mut code = String::from("");
    let mut current_pos = previous_position;

    for instruction in instructions {
        for mv in instruction.chars() {
            let next_pos = match mv {
                'U' => {
                    if current_pos.0 > 0 {
                        //println!("Already at the top of numpad keeping position");
                        (current_pos.0 - 1, current_pos.1)
                    } else {
                        current_pos
                    }
                }
                'D' => (current_pos.0 + 1, current_pos.1),
                'L' => {
                    if current_pos.1 > 0 {
                        (current_pos.0, current_pos.1 - 1)
                    } else {
                        current_pos
                    }
                }
                'R' => (current_pos.0, current_pos.1 + 1),
                _ => {
                    println!("Invalid movement: {mv}");
                    break;
                }
            };
            if numpad.contains_key(&next_pos) {
                current_pos = next_pos;
            }
        }
        code.push_str(&numpad[&current_pos]);
    }
    code
}

fn main() {
    let binding = fs::read_to_string("../input.txt").expect("Error reading input file");
    let data: Vec<&str> = binding.trim().lines().collect();
    let sep = "=".repeat(20);
    let code = find_code(data.clone(), (1, 1));
    println!("{sep} Part 1 {sep}");
    println!("The code to the bathroom is :{code}");
    let code_2 = find_code_2(data, (2, 0));
    println!("{sep} Part 2 {sep}");
    println!("The code to the bathroom is :{code_2}");
}
