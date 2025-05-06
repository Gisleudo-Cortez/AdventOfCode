use std::fs;

fn parse_input(path: &str) -> Vec<Vec<char>> {
    fs::read_to_string(path)
        .expect("Failed parsing input")
        .split(",")
        .map(|s| s.to_string())
        .map(|m| m.chars().collect())
        .collect()
}

fn simulate_dance(moves: Vec<Vec<char>>, loops: u64) -> (String, String) {
    let mut programs: Vec<char> = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    ]
    .to_vec();
    let mut seen: Vec<String> = Vec::new();
    loop {
        for mv in moves.clone() {
            let m: char = mv[0];
            match m {
                's' => {
                    let n: usize = mv[1..].iter().collect::<String>().parse().unwrap();
                    let rotated = [&programs[16 - n..], &programs[..16 - n]].concat();
                    programs = rotated;
                }
                'x' => {
                    let str_mv: String = mv[1..].iter().collect();
                    let parts: Vec<&str> = str_mv.split("/").collect();
                    let a: usize = parts[0].trim().parse().unwrap();
                    let b: usize = parts[1].trim().parse().unwrap();
                    programs.swap(a, b);
                }
                'p' => {
                    let p_a = programs.iter().position(|&c| c == mv[1]).unwrap();
                    let p_b = programs.iter().position(|&c| c == mv[3]).unwrap();
                    programs.swap(p_a, p_b);
                }
                _ => {
                    panic!("Invalid move: {}", m);
                }
            }
        }
        let out = programs.iter().collect::<String>();
        if seen.contains(&out) {
            break;
        }
        seen.push(out);
    }
    let seen_len = seen.len();
    let idx: usize = (loops as usize - 1) % seen_len;
    let out_2 = seen[idx].clone();
    (seen[0].clone(), out_2)
}

fn main() {
    let path = "../input.txt";
    let sep = "=".repeat(20);
    let moves = parse_input(path);
    println!("{sep} Part 1 {sep}");
    let (pt1, pt2) = simulate_dance(moves, 1_000_000_000);
    println!("The final order of the programs is: {pt1}");
    println!("{sep} Part 2 {sep}");
    println!("The order after 1 Billion iterations is: {pt2}");
}
