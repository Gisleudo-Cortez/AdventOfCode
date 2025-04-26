use std::{collections::HashMap, fs};

// Function to interpret instructions and find register values
fn interpret(path: &str) -> (i32, i32) {
    // Store register values (name -> value)
    let mut registers: HashMap<&str, i32> = HashMap::new();
    // Track the highest value ever held by any register during execution
    let mut max_value_ever = i32::MIN; // Start with the smallest possible i32 value

    // Read the input file
    let data = fs::read_to_string(path).expect("Error reading input file.");

    // Process each line (instruction)
    for line in data.lines() {
        // Split the line into parts
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 7 {
            eprintln!("Skipping malformed line: {}", line); // Handle potential bad input lines
            continue;
        }

        // Extract instruction components
        let target_reg = parts[0]; // Register to modify
        let op = parts[1]; // Operation ("inc" or "dec")
        let op_val: i32 = parts[2].parse().expect("failed to parse operation value");
        // parts[3] is "if" - we skip it
        let cond_reg = parts[4]; // Register to check in condition
        let comparator = parts[5]; // Comparison operator (>, <, ==, etc.)
        let cond_val: i32 = parts[6].parse().expect("failed to parse condition value");

        // Ensure both registers exist in our map, defaulting to 0 if new
        let cond_reg_val = *registers.entry(cond_reg).or_insert(0);
        registers.entry(target_reg).or_insert(0); // Ensure target exists too

        // Check the condition
        let condition_met = match comparator {
            ">" => cond_reg_val > cond_val,
            "<" => cond_reg_val < cond_val,
            "!=" => cond_reg_val != cond_val,
            "==" => cond_reg_val == cond_val,
            ">=" => cond_reg_val >= cond_val,
            "<=" => cond_reg_val <= cond_val,
            _ => panic!("Invalid comparator: {comparator} on line: {line}"),
        };

        // If the condition is met, perform the operation
        if condition_met {
            // Get a mutable reference to the target register's value
            let target_val = registers.entry(target_reg).or_insert(0);

            // Perform "inc" or "dec"
            match op {
                "inc" => *target_val += op_val,
                "dec" => *target_val -= op_val,
                _ => panic!("Invalid operation: {op} on line: {line}"),
            }

            // Update the highest value ever seen (for Part 2)
            if *target_val > max_value_ever {
                max_value_ever = *target_val;
            }
        }
    }

    // Find the largest value in any register at the end (for Part 1)
    // Use or_insert(0) in case registers map is empty (though unlikely with AoC input)
    let final_max_value = *registers.values().max().unwrap_or(&0);

    // Return both the final max value and the highest value ever held
    (final_max_value, max_value_ever)
}

fn main() {
    let path = "../input.txt"; // Adjust path if needed
    let sep = "=".repeat(20);

    // Run the interpretation
    let (part1_result, part2_result) = interpret(path);

    // Print results
    println!("{sep} Part 1 {sep}");
    println!("Largest value in any register at the end: {}", part1_result);
    println!("{sep} Part 2 {sep}");
    println!(
        "Highest value held in any register during execution: {}",
        part2_result
    );
}
