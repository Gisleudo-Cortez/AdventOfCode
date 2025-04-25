use std::fs;

fn parse_input(path: &str) -> Vec<Vec<i32>> {
    let data = fs::read_to_string(path).expect("Failed to read input file");
    data.lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<i32>().expect("Failed to parse value into i32"))
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn calculate_check_sum(sheet: &Vec<Vec<i32>>) -> i32 {
    let mut checksum = 0;
    for line in sheet {
        let max = line.iter().max().unwrap();
        let min = line.iter().min().unwrap();
        checksum += max - min;
    }
    checksum
}

fn checksum_divide(sheet: Vec<Vec<i32>>) -> i32 {
    let mut checksum = 0;
    for row in sheet {
        for (i, &a) in row.iter().enumerate() {
            for &b in &row[i + 1..] {
                if a % b == 0 {
                    checksum += a / b;
                    break;
                } else if b % a == 0 {
                    checksum += b / a;
                    break;
                }
            }
        }
    }
    checksum
}

fn main() {
    let path = "../input.txt";
    let sep = "=".repeat(20);
    println!("{sep} Part 1 {sep}");
    let sheet = parse_input(path);
    let part1 = calculate_check_sum(&sheet);
    println!("The checksum of the spreadsheet is: {part1}");
    println!("{sep} Part 2 {sep}");
    let part2 = checksum_divide(sheet);
    println!("The checksum of the spreadsheet is: {part2}");
}
