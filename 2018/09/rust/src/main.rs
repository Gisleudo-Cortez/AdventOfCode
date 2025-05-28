use std::collections::VecDeque;
use std::fs;

fn parse_game_parameters(content: &str) -> Option<(usize, usize)> {
    let parts: Vec<&str> = content.split_whitespace().collect();
    if parts.len() >= 8 {
        let players = parts[0].parse().ok()?;
        let last_marble = parts[6].parse().ok()?;
        Some((players, last_marble))
    } else {
        None
    }
}

fn simulate_marble_game(num_players: usize, last_marble: usize) -> usize {
    let mut circle = VecDeque::new();
    circle.push_back(0);
    let mut scores = vec![0; num_players];

    for marble in 1..=last_marble {
        let player = (marble - 1) % num_players;

        if marble % 23 == 0 {
            for _ in 0..7 {
                let back = circle.pop_back().unwrap();
                circle.push_front(back);
            }
            let removed = circle.pop_front().unwrap();
            scores[player] += marble + removed;
        } else {
            for _ in 0..2 {
                let front = circle.pop_front().unwrap();
                circle.push_back(front);
            }
            circle.push_front(marble);
        }
    }

    *scores.iter().max().unwrap_or(&0)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = "../input.txt";
    let content = fs::read_to_string(input_path)?;
    let (num_players, last_marble) =
        parse_game_parameters(&content).ok_or("Failed to parse game parameters")?;
    let separator = "=".repeat(20);

    println!("{} Part 1 {}", separator, separator);
    let part1 = simulate_marble_game(num_players, last_marble);
    println!("Winning Elf's score: {}", part1);

    println!("{} Part 2 {}", separator, separator);
    let part2 = simulate_marble_game(num_players, last_marble * 100);
    println!("Winning Elf's score (100x marbles): {}", part2);

    Ok(())
}
