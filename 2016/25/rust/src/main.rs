use std::fs;

#[derive(Clone, Debug)]
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
    Out(Operand),
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
        "out" => Instruction::Out(parse_operand(parts[1])),
        _ => panic!("Invalid instruction"),
    }
}

fn parse_input(path: &str) -> Vec<String> {
    let binding = fs::read_to_string(path).expect("Error reading input file");
    binding.lines().map(|l| l.trim().to_string()).collect()
}

fn simulate(instructions: &[Instruction], mut registers: [i32; 4]) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
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
            Instruction::Out(x) => {
                output.push(x.get_value(&registers));
                if output.len() >= 20 {
                    break;
                }
                ip += 1;
            }
        }
    }
    output
}

fn is_alternating_pattern(output: &[i32]) -> bool {
    for (i, _) in output.iter().enumerate() {
        if output[i] != (i as i32 % 2) {
            return false;
        }
    }
    true
}

fn find_min_a(instructions: &[Instruction]) -> i32 {
    for a in 0.. {
        let mut registers = [0; 4];
        registers[0] = a;

        let output = simulate(instructions, registers);
        if is_alternating_pattern(&output) {
            return a;
        }
    }
    unreachable!()
}

fn main() {
    let path = "../input.txt";
    let sep = "=".repeat(20);
    println!("{sep} Part 1 {sep}");
    let instructions: Vec<Instruction> = parse_input(path)
        .into_iter()
        .map(|l| parse_instruction(&l))
        .collect();
    let part1 = find_min_a(&instructions);
    println!("Smallest 'a' producing alternating output: {part1}");
}
