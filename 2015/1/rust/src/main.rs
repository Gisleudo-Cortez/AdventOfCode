use std::fs;
use std::error::Error;

fn part_1() -> Result<(), Box<dyn Error>> {
    let file_path: String = String::from("../input.txt");
    let  contents = fs::read_to_string(file_path)?;
    let mut counter = 0; // level counter
    for c in contents.chars() {
        match c {
            '(' => counter += 1,
            ')'=> counter -=1,
            _ => continue,
        }
        }
    println!("{counter}");
    Ok(())
    
}

fn part_2() -> Result<(), Box<dyn Error>> {
    let file_path: String = String::from("../input.txt");
    let  contents = fs::read_to_string(file_path)?;
    let mut counter = 0; // first character to reach the basement (-1)
    for (i, c) in contents.chars().enumerate() {
        if counter < 0{
            println!("{i}");
            break
        }
        match c {
            '(' => counter += 1,
            ')'=> counter -=1,
            _ => continue,
        }
        }
    Ok(())
    
}




fn main(){
    let sep = "=".repeat(20);
    println!("{} Part 1 {}", sep, sep);
    let _ = part_1();
    println!("{} Part 2 {}", sep, sep);
    let _ = part_2();
    }
  