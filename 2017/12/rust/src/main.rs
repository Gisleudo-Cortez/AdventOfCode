use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn parse_input(path: &str) -> HashMap<usize, Vec<usize>> {
    let data = fs::read_to_string(path).expect("Error reading input file");
    let mut pipes: HashMap<usize, Vec<usize>> = HashMap::new();
    for line in data.lines() {
        let parts: Vec<&str> = line.split(" <-> ").collect();
        let id = parts[0]
            .trim()
            .parse::<usize>()
            .expect("Failed to parse id");
        let connected: Vec<usize> = if parts.len() > 1 {
            parts[1]
                .split(", ")
                .map(|id| {
                    id.trim()
                        .parse::<usize>()
                        .expect("Failed to parse connected id")
                })
                .collect()
        } else {
            Vec::new()
        };
        pipes.insert(id, connected);
    }
    pipes
}

fn reachable_form(start: usize, graph: &HashMap<usize, Vec<usize>>) -> HashSet<usize> {
    let mut visited = HashSet::new();
    dfs(start, graph, &mut visited);
    visited
}

fn dfs(node: usize, graph: &HashMap<usize, Vec<usize>>, visited: &mut HashSet<usize>) {
    if visited.contains(&node) {
        return;
    }
    visited.insert(node);

    if let Some(neighbors) = graph.get(&node) {
        for &neighbor in neighbors {
            dfs(neighbor, graph, visited);
        }
    }
}

fn find_groups(graph: &HashMap<usize, Vec<usize>>) -> usize {
    let mut visited: HashSet<usize> = HashSet::new();
    let mut group_count: usize = 0;
    for node in graph.keys() {
        if !visited.contains(node) {
            let group = reachable_form(*node, graph);
            visited.extend(&group);
            group_count += 1;
        }
    }
    group_count
}

fn main() {
    let path = "../input.txt";
    let sep = "=".repeat(20);
    let data = parse_input(path);
    let pt1 = reachable_form(0, &data);
    println!("{sep} Part 1 {sep}");
    println!(
        "The total number of programs in group id 0 is: {}",
        pt1.len()
    );
    let pt2 = find_groups(&data);
    println!("{sep} Part 2 {sep}");
    println!("The total amount of groups is: {pt2}");
}
