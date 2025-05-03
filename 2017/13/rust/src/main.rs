use std::{collections::HashMap, fs};

#[derive(PartialEq, Eq, Hash)]
struct Firewall {
    depth: usize,
    scan_pos: usize,
}

fn parse_input(path: &str) -> HashMap<usize, Firewall> {
    let input = fs::read_to_string(path).expect("Failed reading input");
    let mut layers: HashMap<usize, Firewall> = HashMap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let layer = parts[0]
            .trim()
            .parse::<usize>()
            .expect("Failed parsing layer");
        let depth = parts[1].parse::<usize>().expect("Failed parsing depth");
        let fw = Firewall { depth, scan_pos: 0 };
        layers.insert(layer, fw);
    }
    // add empty layers
    let max_layers = layers.keys().max().unwrap();
    for l in 0..(*max_layers + 1) {
        layers.entry(l).or_insert(Firewall {
            depth: 0,
            scan_pos: 0,
        });
    }
    layers
}

fn compute_severity(layers: &HashMap<usize, Firewall>) -> usize {
    layers
        .iter()
        .filter(|(_layer, fw)| fw.depth > 0)
        .filter(|(layer, fw)| *layer % (2 * (fw.depth - 1)) == 0)
        .map(|(layer, fw)| layer * fw.depth)
        .sum()
}

fn is_safe(delay: usize, layers: &HashMap<usize, Firewall>) -> bool {
    for (layer, fw) in layers {
        if fw.depth == 0 {
            continue;
        }

        let period = 2 * (fw.depth - 1);
        if (layer + delay) % period == 0 {
            return false;
        }
    }
    true
}

fn solve_pt2(layers: &HashMap<usize, Firewall>) -> usize {
    let mut delay: usize = 0;
    while !is_safe(delay, layers) {
        delay += 1;
    }
    delay
}

fn main() {
    let path = "../input.txt";
    let sep = "=".repeat(20);
    let layers = parse_input(path);
    println!("{sep} Part 1 {sep}");
    let pt1 = compute_severity(&layers);
    println!("The total severity is: {pt1}");
    println!("{sep} Part 2 {sep}");
    let pt2 = solve_pt2(&layers);
    println!("The minimum delay to safely cross the Firewall is : {pt2} picoseconds");
}
