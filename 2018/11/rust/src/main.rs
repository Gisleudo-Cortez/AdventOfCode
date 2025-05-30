use std::fs;

const GRID_SIZE: usize = 300;

fn power_level(x: usize, y: usize, serial: i32) -> i32 {
    let rack_id = x as i32 + 10;
    let mut power = rack_id * y as i32;
    power += serial;
    power *= rack_id;
    (power / 100 % 10) - 5
}

fn build_summed_area_table(serial: i32) -> Vec<Vec<i32>> {
    let mut sat = vec![vec![0; GRID_SIZE + 1]; GRID_SIZE + 1];
    for y in 1..=GRID_SIZE {
        for x in 1..=GRID_SIZE {
            let pl = power_level(x, y, serial);
            sat[y][x] = pl + sat[y - 1][x] + sat[y][x - 1] - sat[y - 1][x - 1];
        }
    }
    sat
}

fn total_power(sat: &Vec<Vec<i32>>, x: usize, y: usize, size: usize) -> i32 {
    let x1 = x - 1;
    let y1 = y - 1;
    let x2 = x + size - 1;
    let y2 = y + size - 1;
    sat[y2][x2] - sat[y1][x2] - sat[y2][x1] + sat[y1][x1]
}

fn part1(sat: &Vec<Vec<i32>>) -> (usize, usize) {
    let mut max_power = i32::MIN;
    let mut coord = (0, 0);
    for y in 1..=GRID_SIZE - 2 {
        for x in 1..=GRID_SIZE - 2 {
            let power = total_power(sat, x, y, 3);
            if power > max_power {
                max_power = power;
                coord = (x, y);
            }
        }
    }
    coord
}

fn part2(sat: &Vec<Vec<i32>>) -> (usize, usize, usize) {
    let mut max_power = i32::MIN;
    let mut result = (0, 0, 0);
    for size in 1..=GRID_SIZE {
        for y in 1..=GRID_SIZE - size + 1 {
            for x in 1..=GRID_SIZE - size + 1 {
                let power = total_power(sat, x, y, size);
                if power > max_power {
                    max_power = power;
                    result = (x, y, size);
                }
            }
        }
    }
    result
}

fn main() {
    let path = "../input.txt";
    let sep = "=".repeat(20);
    let input = fs::read_to_string(path).expect("Error reading input file");
    let serial: i32 = input.trim().parse().expect("Invalid serial number");

    let sat = build_summed_area_table(serial);

    println!("{sep} Part 1 {sep}");
    let (x1, y1) = part1(&sat);
    println!("Top-left coordinate of the 3x3 square with the largest total power: {x1},{y1}");

    println!("{sep} Part 2 {sep}");
    let (x2, y2, size) = part2(&sat);
    println!(
        "Top-left coordinate and size of the square with the largest total power: {x2},{y2},{size}"
    );
}
