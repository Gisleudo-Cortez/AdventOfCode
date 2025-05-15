use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
enum Operand {
    Register(String),
    Value(isize),
}

#[derive(Debug, Clone)]
enum Instruction {
    Set(String, Operand),
    Sub(String, Operand),
    Mul(String, Operand),
    Jnz(Operand, Operand),
}

fn parse_operand(s: &str) -> Operand {
    s.parse::<isize>()
        .map(Operand::Value)
        .unwrap_or_else(|_| Operand::Register(s.to_string()))
}

fn parse_instructions(path: &str) -> Vec<Instruction> {
    let input = fs::read_to_string(path).expect("Error reading input file");
    input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split_whitespace().collect();
            match parts[0] {
                "set" => Instruction::Set(parts[1].into(), parse_operand(parts[2])),
                "sub" => Instruction::Sub(parts[1].into(), parse_operand(parts[2])),
                "mul" => Instruction::Mul(parts[1].into(), parse_operand(parts[2])),
                "jnz" => Instruction::Jnz(parse_operand(parts[1]), parse_operand(parts[2])),
                _ => panic!("Unknown instruction {}", parts[0]),
            }
        })
        .collect()
}

fn get_value(op: &Operand, registers: &HashMap<String, isize>) -> isize {
    match op {
        Operand::Value(v) => *v,
        Operand::Register(r) => *registers.get(r).unwrap_or(&0),
    }
}

fn simulate(instructions: &[Instruction]) -> usize {
    let mut registers: HashMap<String, isize> = HashMap::new();
    let mut pc: isize = 0;
    let mut mul_count = 0;

    while pc >= 0 && (pc as usize) < instructions.len() {
        match &instructions[pc as usize] {
            Instruction::Set(x, y) => {
                let val = get_value(y, &registers);
                registers.insert(x.clone(), val);
            }
            Instruction::Sub(x, y) => {
                let val = get_value(y, &registers);
                *registers.entry(x.clone()).or_insert(0) -= val;
            }
            Instruction::Mul(x, y) => {
                let val = get_value(y, &registers);
                *registers.entry(x.clone()).or_insert(0) *= val;
                mul_count += 1;
            }
            Instruction::Jnz(x, y) => {
                if get_value(x, &registers) != 0 {
                    pc += get_value(y, &registers);
                    continue;
                }
            }
        }
        pc += 1;
    }

    mul_count
}

fn is_not_prime(n: usize) -> bool {
    if n < 2 {
        return true;
    }
    for i in 2..=((n as f64).sqrt() as usize) {
        if n % i == 0 {
            return true;
        }
    }
    false
}

fn optimized_part2() -> usize {
    let (start, end, step) = (106700, 123700, 17);
    (start..=end)
        .step_by(step)
        .filter(|&n| is_not_prime(n))
        .count()
}

fn main() {
    let path = "../input.txt";
    let sep = "=".repeat(20);
    let instructions = parse_instructions(path);
    println!("{sep} Part 1 {sep}");
    let part1_mul_count = simulate(&instructions);
    println!("'mul' was invoked {} times", part1_mul_count);
    println!("{sep} Part 2 {sep}");
    let part2_result = optimized_part2();
    println!("Final value in register 'h' is {}", part2_result);
}
