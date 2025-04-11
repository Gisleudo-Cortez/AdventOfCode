use std::fs;

enum Instruction {
    SwapPos(usize, usize),
    SwapLetter(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateBased(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

fn parse_instruction(instruction: &str) -> Instruction {
    let parts: Vec<_> = instruction.split_whitespace().collect();
    let inst_type = format!("{} {}", parts[0], parts[1]);
    match inst_type.as_str() {
        "swap position" => {
            let x = parts[2]
                .parse::<usize>()
                .expect("Failed to unwrap x on swap position");
            let y = parts[5]
                .parse::<usize>()
                .expect("Failed to unwrap y on swap position");
            Instruction::SwapPos(x, y)
        }
        "swap letter" => {
            let x = parts[2].chars().next().unwrap();
            let y = parts[5].chars().next().unwrap();
            Instruction::SwapLetter(x, y)
        }
        "rotate left" => {
            let x = parts[2]
                .parse::<usize>()
                .expect("Failed to unwrap x on rotate left");
            Instruction::RotateLeft(x)
        }
        "rotate right" => {
            let x = parts[2]
                .parse::<usize>()
                .expect("Failed to unwrap x on rotate right");
            Instruction::RotateRight(x)
        }
        "rotate based" => {
            let x = parts[6]
                .chars()
                .next()
                .expect("Failed to unwrap x on rotate based");
            Instruction::RotateBased(x)
        }
        "reverse positions" => {
            let x = parts[2]
                .parse::<usize>()
                .expect("Failed to unwrap x on reverse positions");
            let y = parts[4]
                .parse::<usize>()
                .expect("Failed to unwrap y on reverse positions");
            Instruction::Reverse(x, y)
        }
        "move position" => {
            let x = parts[2]
                .parse::<usize>()
                .expect("Failed to unwrap x on move position");
            let y = parts[5]
                .parse::<usize>()
                .expect("Failed to unwrap y on move position");
            Instruction::Move(x, y)
        }
        _ => {
            println!("Invalid Instruction: {}", inst_type);
            panic!("BOOOOMM")
        }
    }
}

fn apply_instruction(s: &str, instr: &Instruction) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    match instr {
        Instruction::SwapPos(x, y) => {
            chars.swap(*x, *y);
            chars.iter().collect()
        }
        Instruction::SwapLetter(a, b) => {
            let x = chars.iter().position(|&c| c == *a).unwrap();
            let y = chars.iter().position(|&c| c == *b).unwrap();
            chars.swap(x, y);
            chars.iter().collect()
        }
        Instruction::RotateLeft(step) => {
            let steps = step % len;
            chars.rotate_left(steps);
            chars.iter().collect()
        }
        Instruction::RotateRight(step) => {
            let steps = step % len;
            chars.rotate_right(steps);
            chars.iter().collect()
        }
        Instruction::RotateBased(c) => {
            let idx = chars.iter().position(|&x| x == *c).unwrap();
            let mut rotation = 1 + idx;
            if idx >= 4 {
                rotation += 1;
            }
            chars.rotate_right(rotation % len);
            chars.iter().collect()
        }
        Instruction::Reverse(x, y) => {
            let (s, e) = (*x.min(y), *x.max(y));
            chars[s..=e].reverse();
            chars.iter().collect()
        }
        Instruction::Move(x, y) => {
            let ch = chars.remove(*x);
            chars.insert(*y, ch);
            chars.iter().collect()
        }
    }
}

fn reverse_instruction(s: &str, instr: &Instruction) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    match instr {
        Instruction::SwapPos(x, y) => {
            chars.swap(*x, *y);
            chars.iter().collect()
        }
        Instruction::SwapLetter(a, b) => {
            let x = chars.iter().position(|&c| c == *a).unwrap();
            let y = chars.iter().position(|&c| c == *b).unwrap();
            chars.swap(x, y);
            chars.iter().collect()
        }
        Instruction::RotateLeft(step) => {
            let steps = step % len;
            chars.rotate_right(steps);
            chars.iter().collect()
        }
        Instruction::RotateRight(step) => {
            let steps = step % len;
            chars.rotate_left(steps);
            chars.iter().collect()
        }
        Instruction::RotateBased(_) => {
            for i in 0..len {
                let candidate: String = {
                    let mut candidate_chars = chars.clone();
                    candidate_chars.rotate_left(i);
                    candidate_chars.iter().collect()
                };
                if apply_instruction(&candidate, instr) == s {
                    return candidate;
                }
            }
            panic!("Could not reverse rotate based instruction");
        }
        Instruction::Reverse(x, y) => {
            let (s_idx, e_idx) = (*x.min(y), *x.max(y));
            chars[s_idx..=e_idx].reverse();
            chars.iter().collect()
        }
        Instruction::Move(x, y) => {
            let ch = chars.remove(*y);
            chars.insert(*x, ch);
            chars.iter().collect()
        }
    }
}

fn main() {
    let input: Vec<String> = fs::read_to_string("../input.txt")
        .expect("Error reading input")
        .lines()
        .map(|line| line.to_string())
        .collect();

    let instructions: Vec<Instruction> = input.iter().map(|line| parse_instruction(line)).collect();

    let base_pass = "abcdefgh";
    let to_unscramble = "fbgdceah";
    let sep = "=".repeat(20);
    println!("{sep} Part 1 {sep}");

    let part1 = instructions
        .iter()
        .fold(base_pass.to_string(), |acc, instr| {
            apply_instruction(&acc, instr)
        });

    println!("The scrambled password is: {part1}");

    println!("\n{sep} Part 2 {sep}");

    let part2 = instructions
        .iter()
        .rev()
        .fold(to_unscramble.to_string(), |acc, instr| {
            reverse_instruction(&acc, instr)
        });

    println!("The unscrambled password is: {part2}");
}
