use std::{
    collections::{HashMap, HashSet},
    fs,
};

struct Node {
    name: String,
    weight: usize,
    leafs: Option<Vec<String>>,
}

fn parse_input(path: &str) -> Vec<Node> {
    let data = fs::read_to_string(path).expect("Error reading input file");

    let mut nodes: Vec<Node> = Vec::new();

    for line in data.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let name = parts[0].to_string();
        let weight: usize = parts[1]
            .trim_matches(|c| c == '(' || c == ')')
            .parse()
            .expect("Failed to parse weight");

        let leafs = if parts.len() > 2 {
            Some(
                parts[3..]
                    .iter()
                    .map(|s| s.trim_end_matches(',').to_string())
                    .collect(),
            )
        } else {
            None
        };

        nodes.push(Node {
            name,
            weight,
            leafs,
        });
    }
    nodes
}

fn build_map(nodes: Vec<Node>) -> HashMap<String, Node> {
    let mut map = HashMap::new();
    for node in nodes {
        map.insert(node.name.clone(), node);
    }
    map
}

fn total_weight(name: &str, map: &HashMap<String, Node>) -> usize {
    let node = map.get(name).expect("Node not found");
    let mut total: usize = node.weight;

    if let Some(leafs) = node.leafs.as_ref() {
        let mut child_weights = Vec::new();
        for child in leafs {
            let w = total_weight(child, map);
            child_weights.push((child.clone(), w));
            total += w;
        }
        let mut weight_counts = HashMap::new();
        for (_, w) in &child_weights {
            *weight_counts.entry(w).or_insert(0) += 1;
        }
        if weight_counts.len() > 1 {
            println!("Imbalance detected at node {name}");
            let bad_w = weight_counts
                .iter()
                .filter(|&(_, &count)| count == 1)
                .map(|(&weight, _)| weight)
                .next()
                .unwrap();
            let good_w = weight_counts
                .iter()
                .filter(|&(_, &count)| count > 1)
                .map(|(&weight, _)| weight)
                .next()
                .unwrap();
            let bad_child_name = child_weights
                .iter()
                .find(|(_, w)| *w == *bad_w)
                .map(|(name, _)| name)
                .unwrap();

            let bad_node = map.get(bad_child_name).unwrap();

            let corrected = bad_node.weight as isize + (*good_w as isize - *bad_w as isize);
            println!("Correct weight for {bad_child_name} is {corrected}");
        }
    }
    total
}

fn main() {
    let sep = "=".repeat(20);
    let path = "../input.txt";
    let nodes = parse_input(path);
    let all_names: Vec<String> = nodes.iter().map(|n| n.name.clone()).collect();
    let all_leafs: Vec<String> = nodes
        .iter()
        .filter_map(|n| n.leafs.as_ref())
        .flatten()
        .cloned()
        .collect();
    let leaf_set: HashSet<_> = all_leafs.into_iter().collect();

    println!("{sep} Part 1 {sep}");
    let part1 = all_names
        .iter()
        .find(|name| !leaf_set.contains(*name))
        .unwrap();
    println!("The bottom node is: {part1}");
    println!("{sep} Part 2 {sep}");
    let map = build_map(nodes);
    total_weight(part1, &map);
}
