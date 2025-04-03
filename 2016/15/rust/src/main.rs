use std::fs;
#[derive(Debug, Clone)]
struct Disc {
    positions: i32,
    initial: i32,
}

fn parse_input(input: String) -> Vec<Disc> {
    input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split_whitespace().collect();
            let positions = parts[3].parse::<i32>().expect("Failed to parse positions");
            let initial = parts[11]
                .trim_end_matches('.')
                .parse::<i32>()
                .expect("Failed to parse initial positions");
            Disc { positions, initial }
        })
        .collect()
}

fn check_time(discs: &[Disc], time: i32) -> bool {
    discs.iter().enumerate().all(|(index, disc)| {
        let time_at_disc = time + (index as i32) + 1;
        let position_at_time = (disc.initial + time_at_disc) % disc.positions;
        position_at_time == 0
    })
}

fn solve(discs: &[Disc]) -> i32 {
    (0..).find(|&time| check_time(discs, time)).unwrap()
}

fn main() {
    let input = fs::read_to_string("../input.txt").expect("Error reading input file");
    let mut discs = parse_input(input);
    let sep = "=".repeat(20);

    println!("{sep} Part 1 {sep}");
    let part1 = solve(&discs);
    println!(
        "The first time you can press the button is: {} seconds",
        part1
    );
    println!("{sep} Part 2 {sep}");
    discs.push(Disc {
        positions: 11,
        initial: 0,
    });
    let part2 = solve(&discs);
    println!(
        "The second time you can press the button is: {} seconds",
        part2
    );
}
