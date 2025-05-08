use std::{
    collections::{HashMap, VecDeque},
    fs,
};

#[derive(Debug, Clone)]
enum Operand {
    Register(char),
    Value(i64),
}

#[derive(Debug, Clone)]
enum Instruction {
    Snd(Operand),
    Set(char, Operand),
    Add(char, Operand),
    Mul(char, Operand),
    Mod(char, Operand),
    Rcv(char),
    Jgz(Operand, Operand),
}

fn parse_operand(s: &str) -> Operand {
    if let Ok(val) = s.parse::<i64>() {
        Operand::Value(val)
    } else {
        Operand::Register(s.chars().next().unwrap())
    }
}

fn parse_instruction(instruction: &str) -> Result<Instruction, String> {
    let parts: Vec<&str> = instruction.split_whitespace().collect();
    if parts.len() < 2 {
        return Err(format!("Too few parts in instruction: {}", instruction));
    }

    let reg = parts[1].chars().next().ok_or("Missing register")?;
    let op1 = parse_operand(parts[1]);
    let op2 = if parts.len() > 2 {
        parse_operand(parts[2])
    } else {
        Operand::Value(0)
    };

    match parts[0] {
        "snd" => Ok(Instruction::Snd(op1)),
        "set" => Ok(Instruction::Set(reg, op2)),
        "add" => Ok(Instruction::Add(reg, op2)),
        "mul" => Ok(Instruction::Mul(reg, op2)),
        "mod" => Ok(Instruction::Mod(reg, op2)),
        "rcv" => Ok(Instruction::Rcv(reg)),
        "jgz" => Ok(Instruction::Jgz(op1, op2)),
        op => Err(format!("Unknown instruction: {}", op)),
    }
}

fn get_value(op: &Operand, registers: &HashMap<char, i64>) -> i64 {
    match op {
        Operand::Value(v) => *v,
        Operand::Register(r) => *registers.get(r).unwrap_or(&0),
    }
}

fn parse_input(path: &str) -> Vec<Instruction> {
    fs::read_to_string(path)
        .expect("Error reading input file")
        .lines()
        .map(|line| parse_instruction(line).unwrap())
        .collect()
}

fn solve(instructions: Vec<Instruction>) -> i64 {
    let mut registers: HashMap<char, i64> = HashMap::new();
    let mut pc: isize = 0;
    let mut last_sound: i64 = 0;

    while (pc as usize) < instructions.len() {
        match &instructions[pc as usize] {
            Instruction::Snd(x) => {
                last_sound = get_value(x, &registers);
            }
            Instruction::Set(x, y) => {
                let val = get_value(y, &registers);
                registers.insert(*x, val);
            }
            Instruction::Add(x, y) => {
                *registers.entry(*x).or_insert(0) += get_value(y, &registers);
            }
            Instruction::Mul(x, y) => {
                *registers.entry(*x).or_insert(0) *= get_value(y, &registers);
            }
            Instruction::Mod(x, y) => {
                *registers.entry(*x).or_insert(0) %= get_value(y, &registers);
            }
            Instruction::Rcv(x) => {
                if *registers.get(x).unwrap_or(&0) != 0 {
                    return last_sound;
                }
            }
            Instruction::Jgz(x, y) => {
                if get_value(x, &registers) > 0 {
                    pc += get_value(y, &registers) as isize;
                    continue;
                }
            }
        }
        pc += 1;
    }
    0
}

struct Program {
    pc: isize,
    registers: HashMap<char, i64>,
    instructions: Vec<Instruction>,
    send_count: usize,
    inbox: VecDeque<i64>,
    outbox: VecDeque<i64>,
    waiting: bool,
    terminated: bool,
}

impl Program {
    fn new(id: usize, instructions: Vec<Instruction>) -> Self {
        let mut registers = HashMap::new();
        registers.insert('p', id as i64);
        Program {
            pc: 0,
            registers,
            instructions,
            send_count: 0,
            inbox: VecDeque::new(),
            outbox: VecDeque::new(),
            waiting: false,
            terminated: false,
        }
    }

    fn get_value(&self, operand: &Operand) -> i64 {
        match operand {
            Operand::Value(v) => *v,
            Operand::Register(r) => *self.registers.get(r).unwrap_or(&0),
        }
    }

    fn step(&mut self) {
        if self.pc < 0 || self.pc as usize >= self.instructions.len() {
            self.terminated = true;
            return;
        }

        match &self.instructions[self.pc as usize] {
            Instruction::Snd(x) => {
                let val = self.get_value(x);
                self.outbox.push_back(val);
                self.send_count += 1;
                self.pc += 1;
            }
            Instruction::Set(x, y) => {
                let val = self.get_value(y);
                self.registers.insert(*x, val);
                self.pc += 1;
            }
            Instruction::Add(x, y) => {
                *self.registers.entry(*x).or_insert(0) += self.get_value(y);
                self.pc += 1;
            }
            Instruction::Mul(x, y) => {
                *self.registers.entry(*x).or_insert(0) *= self.get_value(y);
                self.pc += 1;
            }
            Instruction::Mod(x, y) => {
                *self.registers.entry(*x).or_insert(0) %= self.get_value(y);
                self.pc += 1;
            }
            Instruction::Rcv(x) => {
                if let Some(val) = self.inbox.pop_front() {
                    self.registers.insert(*x, val);
                    self.waiting = false;
                    self.pc += 1;
                } else {
                    self.waiting = true;
                }
            }
            Instruction::Jgz(x, y) => {
                if self.get_value(x) > 0 {
                    self.pc += self.get_value(y) as isize;
                } else {
                    self.pc += 1;
                }
            }
        }
    }
}

fn simulate_duet(instructions: Vec<Instruction>) -> usize {
    let mut p0 = Program::new(0, instructions.clone());
    let mut p1 = Program::new(1, instructions);

    loop {
        p0.step();
        p1.step();

        while let Some(v) = p0.outbox.pop_front() {
            p1.inbox.push_back(v);
        }
        while let Some(v) = p1.outbox.pop_front() {
            p0.inbox.push_back(v);
        }

        if (p0.waiting || p0.terminated) && (p1.waiting || p1.terminated) {
            break;
        }
    }

    p1.send_count
}

fn main() {
    let path = "../input.txt";
    let sep = "=".repeat(20);
    let instructions = parse_input(path);
    println!("{sep} Part 1 {sep}");
    let pt1 = solve(instructions.clone());
    println!("The value of the first recovered frequency is: {pt1}");
    println!("{sep} Part 2 {sep}");
    let pt2 = simulate_duet(instructions);
    println!("Program 1 sent {pt2} messages");
}
