use std::fs;


fn min_houses(target: &i32) -> i32 {
    let max_house = target / 10;
    let mut presents = vec![0; (max_house + 1) as usize];

    for elf in 1..max_house + 1{
        for house in (elf..=max_house).step_by(elf as usize){
            presents[house as usize] += elf * 10;
        }
    }
    for (house, total) in presents.iter().enumerate(){
        if total >= &target{
            return house as i32;
        }
    }
    -1
}

fn min_11_houses(target: &i32) -> i32 {
    let max_house = target / 10;
    let mut presents = vec![0; (max_house + 1) as usize];

    for elf in 1..max_house + 1{
        let mut count = 0;
        for house in (elf..=max_house).step_by(elf as usize){
            presents[house as usize] += elf * 11;
            count += 1;
            if count == 50{
                break
            }
        }
    }
    for (house, total) in presents.iter().enumerate(){
        if total >= &target{
            return house as i32;
        }
    }
    -1
}

fn main() {
    let data = fs::read_to_string("../input.txt")
    .expect("Error reading input file")
    .trim()
    .parse::<i32>()
    .expect("Failed to parse input to i32");

    let sep = "=".repeat(20);
    let part_1 = min_houses(&data);
    let part_2 = min_11_houses(&data);
    println!("{} Part 1 {}\nThe first house to recive at least {} presents is house nº {}", sep, sep, data, part_1);
    println!("{} Part 2 {}\nThe first house to recive at least {} presents is house nº {}", sep, sep, data, part_2);
}
