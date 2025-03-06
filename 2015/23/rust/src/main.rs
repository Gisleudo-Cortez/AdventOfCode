use std::{collections::HashMap, fs, io};

fn hlf(r: i32) -> i32 {
    r / 2
}

fn tpl(r: i32) -> i32 {
    r * 3
}

fn inc(r: i32) -> i32 {
    r + 1
}

fn jmp(v: i32) -> i32 {
    v
}

fn jie(r: i32, offset: i32) -> i32 {
    if r % 2 == 0 {
        offset
    } else {
        1
    }
}

fn jio(r: i32, offset: i32) -> i32 {
    if r == 1 {
        offset
    } else {
        1
    }
}

fn execute(data: &[String], reg: &mut HashMap<String, i32>) -> (i32, i32) {
    let mut i: i32 = 0;
    let len_data = data.len() as i32;

    while i >= 0 && i < len_data {
        let line = &data[i as usize];
        let instructions: Vec<&str> = line.split_whitespace().collect();
        match instructions[0] {
            "hlf" | "tpl" | "inc" => {
                let r = instructions[1];
                
                let current = *reg.get(r).unwrap_or(&0);
                let new_val = match instructions[0] {
                    "hlf" => hlf(current),
                    "tpl" => tpl(current),
                    "inc" => inc(current),
                    _ => unreachable!(),
                };
                reg.insert(r.to_string(), new_val);
                i += 1;
            }
            "jmp" => {
                
                let offset: i32 = instructions[1].parse().unwrap();
                i += jmp(offset);
            }
            "jie" => {
                
                let r = instructions[1].trim_end_matches(',');
                let offset: i32 = instructions[2].parse().unwrap();
                let current = *reg.get(r).unwrap_or(&0);
                i += jie(current, offset);
            }
            "jio" => {
                
                let r = instructions[1].trim_end_matches(',');
                let offset: i32 = instructions[2].parse().unwrap();
                let current = *reg.get(r).unwrap_or(&0);
                i += jio(current, offset);
            }
            _ => {
                
                break;
            }
        }
    }

    (*reg.get("a").unwrap_or(&0), *reg.get("b").unwrap_or(&0))
}

fn main() -> io::Result<()> {
    let contents = fs::read_to_string("../input.txt")?;
    let data: Vec<String> = contents.lines().map(|s| s.trim().to_string()).collect();
    let sep = "=".repeat(20);
    
    println!("{} Part 1 {}", sep,sep);
    let mut registers: HashMap<String, i32> = HashMap::new();
    registers.insert("a".to_string(), 0);
    registers.insert("b".to_string(), 0);
    let (a, b) = execute(&data, &mut registers);
    println!("register a: {}, b: {}", a, b);
    
    println!("{} Part 2 {}", sep, sep);
    let mut registers: HashMap<String, i32> = HashMap::new();
    registers.insert("a".to_string(), 1);
    registers.insert("b".to_string(), 0);
    let (a, b) = execute(&data, &mut registers);
    println!("register a: {}, b: {}", a, b);
    
    Ok(())
}

