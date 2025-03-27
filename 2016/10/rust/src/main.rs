use std::{cell::RefCell, collections::HashMap, fs};

use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Destination {
    Bot(i32),
    Output(i32),
}

fn parse_destination(type_str: &str, id_str: &str) -> Destination {
    let id = id_str.parse::<i32>().expect("Invalid destination ID");
    match type_str {
        "bot" => Destination::Bot(id),
        "output" => Destination::Output(id),
        _ => panic!("Invalid destination type"),
    }
}

fn solve(input: &str, target_low: i32, target_high: i32) -> (i32, i32) {
    let mut bot_instructions: HashMap<i32, (Destination, Destination)> = HashMap::new();
    let mut initial_values: Vec<(i32, i32)> = Vec::new();

    let value_re = Regex::new(r"value (\d+) goes to bot (\d+)").unwrap();
    let gives_re =
        Regex::new(r"bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)")
            .unwrap();

    for line in input.lines() {
        if let Some(caps) = value_re.captures(line) {
            let value = caps[1].parse::<i32>().unwrap();
            let bot_id = caps[2].parse::<i32>().unwrap();
            initial_values.push((value, bot_id));
        } else if let Some(caps) = gives_re.captures(line) {
            let bot_id = caps[1].parse::<i32>().unwrap();
            let low_dest = parse_destination(&caps[2], &caps[3]);
            let high_dest = parse_destination(&caps[4], &caps[5]);
            bot_instructions.insert(bot_id, (low_dest, high_dest));
        } else if !line.trim().is_empty() {
            eprintln!("Warning unparsed line: {}", line);
        }
    }

    let bots: RefCell<HashMap<i32, Vec<i32>>> = RefCell::new(HashMap::new());
    let outputs: RefCell<HashMap<i32, Vec<i32>>> = RefCell::new(HashMap::new());

    for &bot_id in bot_instructions.keys() {
        bots.borrow_mut().entry(bot_id).or_default();
    }

    for (value, bot_id) in initial_values {
        bots.borrow_mut().entry(bot_id).or_default().push(value);
    }

    let mut part1_answer_bot_id: Option<i32> = None;

    loop {
        let mut bots_mut = bots.borrow_mut();

        let ready_bot_id = bots_mut
            .iter()
            .find(|(_, chips)| chips.len() == 2)
            .map(|(&id, _)| id);

        if let Some(bot_id) = ready_bot_id {
            let mut chips_vec = bots_mut
                .remove(&bot_id)
                .expect("Bot ID disappeared unexpectedly");

            chips_vec.sort();
            let low_val = chips_vec[0];
            let high_val = chips_vec[1];

            if low_val == target_low && high_val == target_high {
                part1_answer_bot_id = Some(bot_id); // Store the bot ID
            }

            let (low_dest, high_dest) = *bot_instructions
                .get(&bot_id)
                .expect("Bot has chips but no instructions?");

            drop(bots_mut);

            // Distribute low chip
            match low_dest {
                Destination::Bot(dest_id) => {
                    bots.borrow_mut().entry(dest_id).or_default().push(low_val);
                }
                Destination::Output(dest_id) => {
                    outputs
                        .borrow_mut()
                        .entry(dest_id)
                        .or_default()
                        .push(low_val);
                }
            }

            // Distribute high chip
            match high_dest {
                Destination::Bot(dest_id) => {
                    bots.borrow_mut().entry(dest_id).or_default().push(high_val);
                }
                Destination::Output(dest_id) => {
                    outputs
                        .borrow_mut()
                        .entry(dest_id)
                        .or_default()
                        .push(high_val);
                }
            }
        } else {
            break; // No more bots ready, simulation is complete
        }
    }

    let final_outputs = outputs.borrow();

    // Helper to safely get the single value from an output bin
    let get_output_val = |output_id: i32| -> i32 {
        match final_outputs.get(&output_id) {
            Some(chips) if chips.len() == 1 => chips[0],
            Some(chips) => panic!("Output {} has {} chips, expected 1", output_id, chips.len()),
            None => panic!("Output {} is missing or empty", output_id),
        }
    };

    let val0 = get_output_val(0);
    let val1 = get_output_val(1);
    let val2 = get_output_val(2);

    let part2_answer_product = val0 * val1 * val2;

    // Ensure Part 1 answer was found before returning
    let final_part1_bot_id = part1_answer_bot_id.expect(&format!(
        "Target comparison ({},{}) never happened!",
        target_low, target_high
    ));

    (final_part1_bot_id, part2_answer_product)
}

fn main() {
    let binding = fs::read_to_string("../input.txt").expect("Error reading input file");
    let input = binding.trim(); // Trim trailing whitespace

    let sep = "=".repeat(20);

    // Call solve once to get both parts
    let (part1_bot, part2_product) = solve(input, 17, 61);

    println!("{sep} Part 1 {sep}");
    println!("The responsible bot is: {}", part1_bot);
    println!("{sep} Part 2 {sep}");
    println!("Product of outputs: {}", part2_product);
}
