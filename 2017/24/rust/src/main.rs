use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Component(usize, usize);

impl Component {
    fn matches(&self, port: usize) -> bool {
        self.0 == port || self.1 == port
    }

    fn other(&self, port: usize) -> usize {
        if self.0 == port { self.1 } else { self.0 }
    }

    fn strength(&self) -> usize {
        self.0 + self.1
    }
}

fn parse_components(path: &str) -> Vec<Component> {
    fs::read_to_string(path)
        .expect("Error reading input file")
        .lines()
        .map(|line| {
            let parts: Vec<usize> = line.split('/').map(|x| x.parse().unwrap()).collect();
            Component(parts[0], parts[1])
        })
        .collect()
}

fn build_bridges(
    components: &[Component],
    used: &mut HashSet<Component>,
    port: usize,
    strength: usize,
    length: usize,
    max_strength: &mut usize,
    max_longest: &mut (usize, usize),
) {
    let mut extended = false;
    for &component in components {
        if !used.contains(&component) && component.matches(port) {
            extended = true;
            used.insert(component);
            let next_port = component.other(port);
            build_bridges(
                components,
                used,
                next_port,
                strength + component.strength(),
                length + 1,
                max_strength,
                max_longest,
            );
            used.remove(&component);
        }
    }

    if !extended {
        *max_strength = (*max_strength).max(strength);
        if length > max_longest.0 || (length == max_longest.0 && strength > max_longest.1) {
            *max_longest = (length, strength);
        }
    }
}

fn main() {
    let path = "../input.txt";
    let sep = "=".repeat(20);
    let components = parse_components(path);
    let mut used = HashSet::new();
    let mut max_strength = 0;
    let mut max_longest = (0, 0);

    build_bridges(
        &components,
        &mut used,
        0,
        0,
        0,
        &mut max_strength,
        &mut max_longest,
    );
    println!("{sep} Part 1 {sep}");
    println!("Part 1: Strongest bridge has strength {}", max_strength);
    println!("{sep} Part 2 {sep}");
    println!("Part 2: Longest bridge has strength {}", max_longest.1);
}
