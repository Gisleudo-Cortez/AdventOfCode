use std::{collections::HashMap, fs};

struct Reindeer {
    name: String,
    speed: i32,
    time: i32,
    rest: i32,
}

fn parse_line(line: &str) -> Reindeer {
    let parts: Vec<&str> = line.split_whitespace().collect();
    Reindeer {
        name: parts[0].to_string(),
        speed: parts[3].parse::<i32>().expect("Error parsing speed on index 3"),
        time: parts[6].parse::<i32>().expect("Error parsing time on index 6"),
        rest: parts[13].trim_matches('.').parse::<i32>().expect("Error parsing rest on index 13"),
    }
}

fn calculate_distance(reindeer: &Reindeer, seconds: i32) -> i32 {
    let cycle_time = reindeer.time + reindeer.rest;
    let full_cycles = seconds / cycle_time;
    let remaining_time = seconds % cycle_time;

    let mut total = full_cycles * (reindeer.speed * reindeer.time);
    total += reindeer.speed * remaining_time.min(reindeer.time);
    
    total
}

fn get_fastest(reindeers: &Vec<String>, seconds: i32) -> String {
    let mut reindeer_list: Vec<(String, i32)> = Vec::new();
    
    for reindeer in reindeers {
        let info = parse_line(reindeer);
        let distance = calculate_distance(&info, seconds);
        reindeer_list.push((info.name.clone(), distance));
    }
    
    reindeer_list.sort_by_key(|&(_, distance)| distance);

    let fastest = reindeer_list.last().unwrap();
    format!("{}: {}", fastest.0, fastest.1)
}

fn calculate_points(reindeers: &Vec<String>,seconds: i32) -> HashMap<String,i32> {
    let reindeers_info: Vec<Reindeer> = reindeers.iter().map(|r| parse_line(&r)).collect();
    let mut points: HashMap<String, i32> = HashMap::new();

    for reindeer in &reindeers_info{
        points.insert(reindeer.name.clone(), 0);
    }

    for second in 1..=seconds {
        let mut distances = Vec::new();

        for reindeer in &reindeers_info{
            let distance = calculate_distance(reindeer, second);
            distances.push((reindeer.name.clone(), distance));
        }

        let max_distance = distances.iter().map(|(_,d)| *d).max().unwrap_or(0);

        for (name, distance) in distances {
            if distance == max_distance {
                *points.entry(name).or_insert(0) += 1;
            }
        }
    }
    points
}

fn main() {
    let data: Vec<String> = fs::read_to_string("../input.txt")
    .expect("Error reading input file")
    .trim()
    .lines()
    .map(|line| line.to_string())
    .collect();

    let sep = "=".repeat(20);
    let fastest_reindeer = get_fastest(&data, 2503);
    println!("{} Part 1 {}\nThe fastest reindeer is {}", sep, sep, fastest_reindeer);

    let points = calculate_points(&data, 2503);
    if let Some((name, max_points)) = points.iter().max_by_key(|&(_, points)| points) {
        println!("{} Part 2 {}\nThe Winner is {} with {} points", sep, sep, name, max_points);
    } else {
        println!("{} Part 2 {}\nNo winner found", sep, sep);
    }
    
}
