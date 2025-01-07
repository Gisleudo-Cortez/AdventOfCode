use std::fs;
use std::io::Error;
use md5::{Md5, Digest};

fn brute_force_check(data: &str, n_zeros: i32) -> usize {
    let mut answer = 0;
    let target_start = "0".repeat(n_zeros as usize);
    
    loop {
        let mut hasher = Md5::new();
        let test_str = format!("{}{}", data, answer);
        hasher.update(test_str.as_bytes());
        let result = hasher.finalize();
        let hash_str = format!("{:x}", result);
        
        if hash_str.starts_with(&target_start) {
            return answer;
        }
        answer += 1;
    }
}

fn main() -> Result<(), Error> {
    // Read and trim the input data
    let data = fs::read_to_string("../input.txt")?.trim().to_string();
    
    let part1 = brute_force_check(&data, 5);
    println!("Part 1: {}", part1);
    
    let part2 = brute_force_check(&data, 6);
    println!("Part 2: {}", part2);
    
    Ok(())
}