use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Instruction{
    Direct(String),
    Not(String),
    And(String, String),
    Or(String, String),
    LShift(String, u16),
    RShift(String, u16),
}

struct Circuit {
    wire_instructions: HashMap<String, Instruction>,
    wire_cache: HashMap<String, u16>,
}

impl Circuit {
    fn new() -> Self {
        Self {
            wire_instructions: HashMap::new(),
            wire_cache: HashMap::new(),
        }
    }

    fn add_instruction(&mut self, instruction :&str) {
        let parts: Vec<&str> = instruction.split(" -> ").collect();
        if parts.len() != 2 {
            panic!("Invalid instruction format: {}", instruction);
        }
        let target_wire = parts[1].trim().to_string();
        let source = parts[0].trim();

        let parsed_instruction = match source.split_whitespace().collect::<Vec<&str>>().as_slice() {
            [val] => Instruction::Direct(val.to_string()),

            ["NOT", x] => Instruction::Not(x.to_string()),

            [x, "AND", y] => Instruction::And(x.to_string(), y.to_string()),

            [x, "OR", y] => Instruction::Or(x.to_string(), y.to_string()),

            [x, "LSHIFT", y] => Instruction::LShift(x.to_string(), y.parse().unwrap()),

            [x, "RSHIFT", y] => Instruction::RShift(x.to_string(), y.parse().unwrap()),

            _ => panic!("Invalid instruction format: {}", instruction),
        };
        self.wire_instructions.insert(target_wire, parsed_instruction);
    }

    fn override_wire(&mut self, wire: &str, value: u16) {
        self.wire_instructions.insert(wire.to_string(), Instruction::Direct(value.to_string()));
    }

    // fn reset_cache(&mut self){
    //     self.wire_cache.clear();
    // }

    fn get_wire_value(&mut self, wire: &str) -> u16 {
        if let Some(&cached_value) = self.wire_cache.get(wire) {
            return cached_value;
        }

        if let Ok(value) = u16::from_str(wire) {
            return value;
        }

        let instruction = self.wire_instructions.get(wire).expect("Wire not found").clone();
        let value = match instruction {
            Instruction::Direct(x) => self.get_wire_value(&x),

            Instruction::Not(x) => !self.get_wire_value(&x) & 0xFFFF,

            Instruction::And(x, y) => self.get_wire_value(&x) & self.get_wire_value(&y),

            Instruction::Or(x, y) => self.get_wire_value(&x) | self.get_wire_value(&y),

            Instruction::LShift(x, n) => (self.get_wire_value(&x) << n) & 0xFFFF,

            Instruction::RShift(x,n ) => self.get_wire_value(&x) >> n,
        };
        self.wire_cache.insert(wire.to_string(), value);
        value
    }
}

fn solve_part1(instructions: &[String]) -> u16 {
    let mut circuit = Circuit::new();
    for instruction in instructions{
        circuit.add_instruction(instruction);
    }
    circuit.get_wire_value("a")
}

fn solve_part2(instructions: &[String], part1_result: u16) -> u16 {
    let mut circuit = Circuit::new();
    for instruction in instructions{
        circuit.add_instruction(instruction);
    }
    circuit.override_wire("b", part1_result);
    circuit.get_wire_value("a")
}

fn main() {
    let data = fs::read_to_string("../input.txt").expect("Failed to read file");
    let instructions: Vec<String> = data.lines().map(|line| line.to_string()).collect();

    let part1_result = solve_part1(&instructions);
    println!("Part 1 - wire 'a' values: {}", part1_result);

    let part2_result = solve_part2(&instructions, part1_result);
    println!("Part 2 - wire 'a' values: {}", part2_result);
}