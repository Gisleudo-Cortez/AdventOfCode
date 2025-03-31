use std::{collections::HashMap, fs};

fn get_value(operand: &str, registers: &HashMap<char, i32>) -> i32 {
    match operand.parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            let reg_char = operand.chars().next().expect("Invalid register operand");
            *registers.get(&reg_char).unwrap_or(&0)
        }
    }
}

fn solve(input: &str, initial_reg: HashMap<char, i32>) -> i32 {
    let instructions: Vec<Vec<&str>> = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.split_whitespace().collect())
        .collect();

    let mut registers = initial_reg;
    let mut pc: isize = 0;

    while pc >= 0 && (pc as usize) < instructions.len() {
        let current_inst_index = pc as usize;
        let parts = &instructions[current_inst_index];
        let mut next_pc = pc + 1;

        match parts[0] {
            "cpy" => {
                if parts.len() != 3 {
                    panic!(
                        "Invalid cpy instruction length at line {}: {:?}",
                        current_inst_index + 1,
                        parts
                    );
                }
                let val_to_cp = get_value(parts[1], &registers);
                let dest_reg = parts[2]
                    .chars()
                    .next()
                    .expect("Destination must be a register");
                registers.insert(dest_reg, val_to_cp);
            }
            "inc" => {
                if parts.len() != 2 {
                    panic!(
                        "Invalid cpy instruction length at line {}: {:?}",
                        current_inst_index + 1,
                        parts
                    );
                }
                let reg_char = parts[1]
                    .chars()
                    .next()
                    .expect("inc operand mut be register");
                *registers.entry(reg_char).or_insert(0) += 1;
            }
            "dec" => {
                if parts.len() != 2 {
                    panic!(
                        "Invalid cpy instruction length at line {}: {:?}",
                        current_inst_index + 1,
                        parts
                    );
                }
                let reg_char = parts[1]
                    .chars()
                    .next()
                    .expect("dec operand mut be register");
                *registers.entry(reg_char).or_insert(0) -= 1;
            }
            "jnz" => {
                if parts.len() != 3 {
                    panic!(
                        "Invalid cpy instruction length at line {}: {:?}",
                        current_inst_index + 1,
                        parts
                    );
                }
                let check_val = get_value(parts[1], &registers);
                if check_val != 0 {
                    let jmp_offset = parts[2].parse::<isize>().expect("Invalid jnz offset");
                    next_pc = pc + jmp_offset;
                }
            }
            _ => {
                panic!(
                    "Invalid instruction '{}' at line {}",
                    parts[0],
                    current_inst_index + 1
                );
            }
        }
        pc = next_pc;
    }
    *registers.get(&'a').unwrap_or(&0)
}

fn main() {
    let input = fs::read_to_string("../input.txt").expect("Error reading input file");
    let sep = "=".repeat(20);

    let mut initial_registers_p1 = HashMap::new();
    initial_registers_p1.insert('a', 0);
    initial_registers_p1.insert('b', 0);
    initial_registers_p1.insert('c', 0);
    initial_registers_p1.insert('d', 0);

    println!("{sep} Part 1 {sep}");
    let result_p1 = solve(&input, initial_registers_p1);
    println!("Final value of register 'a' (Part 1): {}", result_p1);

    let mut initial_registers_p2 = HashMap::new();
    initial_registers_p2.insert('a', 0);
    initial_registers_p2.insert('b', 0);
    initial_registers_p2.insert('c', 1);
    initial_registers_p2.insert('d', 0);

    println!("{sep} Part 2 {sep}");
    let result_p2 = solve(&input, initial_registers_p2);
    println!("Final value of register 'a' (Part 2): {}", result_p2);
}
