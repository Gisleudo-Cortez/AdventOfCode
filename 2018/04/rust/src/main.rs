use std::collections::HashMap;
use std::fs;

fn parse_logs(path: &str) -> Vec<(String, String)> {
    let mut lines: Vec<_> = fs::read_to_string(path)
        .expect("Error reading input file")
        .lines()
        .map(|line| {
            let ts = &line[1..17];
            let event = &line[19..];
            (ts.to_string(), event.to_string())
        })
        .collect();
    lines.sort_by_key(|entry| entry.0.clone());
    lines
}

fn analyze_guard_sleeps(logs: Vec<(String, String)>) -> (usize, usize, usize) {
    let mut guard = 0;
    let mut asleep_min = 0;
    let mut sleep_map: HashMap<usize, [usize; 60]> = HashMap::new();

    for (timestamp, event) in logs {
        if event.contains("Guard") {
            guard = event
                .split('#')
                .nth(1)
                .unwrap()
                .split_whitespace()
                .next()
                .unwrap()
                .parse()
                .unwrap();
        } else if event == "falls asleep" {
            asleep_min = timestamp[14..16].parse::<usize>().unwrap();
        } else if event == "wakes up" {
            let wake_min = timestamp[14..16].parse::<usize>().unwrap();
            let entry = sleep_map.entry(guard).or_insert([0; 60]);
            for m in asleep_min..wake_min {
                entry[m] += 1;
            }
        }
    }

    let part1_guard = sleep_map
        .iter()
        .max_by_key(|&(_, mins)| mins.iter().sum::<usize>())
        .map(|(id, _)| *id)
        .unwrap();

    let part1_minute = sleep_map[&part1_guard]
        .iter()
        .enumerate()
        .max_by_key(|&(_, &v)| v)
        .map(|(i, _)| i)
        .unwrap();

    let (part2_guard, part2_minute, _) = sleep_map
        .iter()
        .flat_map(|(&id, minutes)| minutes.iter().enumerate().map(move |(i, &v)| (id, i, v)))
        .max_by_key(|&(_, _, v)| v)
        .unwrap();

    (
        part1_guard * part1_minute,
        part2_guard * part2_minute,
        part2_guard,
    )
}

fn main() {
    let path = "../input.txt";
    let sep = "=".repeat(20);
    let logs = parse_logs(path);
    let (part1_result, part2_result, guard_id) = analyze_guard_sleeps(logs);

    println!("{sep} Part 1 {sep}");
    println!("Checksum (Guard ID * Minute) = {}", part1_result);
    println!("{sep} Part 2 {sep}");
    println!(
        "Checksum (Guard ID * Minute) = {} (Guard {})",
        part2_result, guard_id
    );
}
