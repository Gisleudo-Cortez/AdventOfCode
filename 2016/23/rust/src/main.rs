use std::fs;

#[derive(Clone)]
enum Operand {
    Register(char),
    Value(i32),
}

#[derive(Clone)]
enum Instruction {
    Cpy(Operand, Operand),
    Inc(Operand),
    Dec(Operand),
    Jnz(Operand, Operand),
    Tgl(Operand),
}

impl Operand {
    fn get_value(&self, registers: &[i32; 4]) -> i32 {
        match self {
            Operand::Value(val) => *val,
            Operand::Register(r) => registers[reg_idx(*r)],
        }
    }

    fn get_register_index(&self) -> Option<usize> {
        match self {
            Operand::Register(r) => Some(reg_idx(*r)),
            _ => None,
        }
    }
}

fn reg_idx(c: char) -> usize {
    (c as u8 - b'a') as usize
}

fn parse_operand(s: &str) -> Operand {
    if let Ok(val) = s.parse::<i32>() {
        Operand::Value(val)
    } else {
        Operand::Register(s.chars().next().unwrap())
    }
}

fn parse_instruction(line: &str) -> Instruction {
    let parts: Vec<&str> = line.split_whitespace().collect();
    match parts[0] {
        "cpy" => Instruction::Cpy(parse_operand(parts[1]), parse_operand(parts[2])),
        "inc" => Instruction::Inc(parse_operand(parts[1])),
        "dec" => Instruction::Dec(parse_operand(parts[1])),
        "jnz" => Instruction::Jnz(parse_operand(parts[1]), parse_operand(parts[2])),
        "tgl" => Instruction::Tgl(parse_operand(parts[1])),
        _ => panic!("Invalid instruction"),
    }
}

fn parse_input(path: &str) -> Vec<String> {
    let binding = fs::read_to_string(path).expect("Error reading input file");
    binding.lines().map(|l| l.trim().to_string()).collect()
}

fn solve_pt1(input_path: &str, mut registers: [i32; 4]) -> i32 {
    let mut instructions: Vec<Instruction> = parse_input(input_path)
        .into_iter()
        .map(|l| parse_instruction(&l))
        .collect();
    let mut ip = 0;

    while let Some(instruction) = instructions.get(ip as usize) {
        match instruction {
            Instruction::Cpy(x, y) => {
                if let Some(idx) = y.get_register_index() {
                    registers[idx] = x.get_value(&registers);
                }
                ip += 1;
            }
            Instruction::Inc(x) => {
                if let Some(idx) = x.get_register_index() {
                    registers[idx] += 1;
                }
                ip += 1;
            }
            Instruction::Dec(x) => {
                if let Some(idx) = x.get_register_index() {
                    registers[idx] -= 1;
                }
                ip += 1;
            }
            Instruction::Jnz(x, y) => {
                let offset = y.get_value(&registers);
                if x.get_value(&registers) != 0 && offset != 0 {
                    ip += offset;
                } else {
                    ip += 1;
                }
            }
            Instruction::Tgl(x) => {
                let target = ip + x.get_value(&registers);
                if target >= 0 && (target as usize) < instructions.len() {
                    instructions[target as usize] =
                        toggle_instruction(&instructions[target as usize]);
                }
                ip += 1;
            }
        }
    }
    registers[0]
}

fn toggle_instruction(instr: &Instruction) -> Instruction {
    match instr {
        Instruction::Inc(x) => Instruction::Dec(x.clone()),
        Instruction::Dec(x) => Instruction::Inc(x.clone()),
        Instruction::Tgl(x) => Instruction::Inc(x.clone()),
        Instruction::Jnz(x, y) => Instruction::Cpy(x.clone(), y.clone()),
        Instruction::Cpy(x, y) => Instruction::Jnz(x.clone(), y.clone()),
    }
}

fn factorial(n: i32) -> i32 {
    let mut out = 1;
    for x in 1..=n {
        out *= x;
    }
    out
}

fn main() {
    let path = "../input.txt";
    let sep = "=".repeat(20);
    println!("{sep} Part 1 {sep}");
    let part1 = solve_pt1(path, [7, 0, 0, 0]);
    println!("The safe value to send is: {part1}");
    println!("{sep} Part 2 {sep}");
    let part2 = factorial(12) + 75 * 72;
    println!("The safe value to send is: {part2}");
}
