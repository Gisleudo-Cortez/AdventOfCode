use std::collections::HashMap;
use std::fs;
use itertools::Itertools;

#[derive(Debug)]
struct Node {
    connections: HashMap<String, i32>
}

impl Node {
    fn new() -> Self {
        Node {
            connections: HashMap::new(),
        }
    }

    fn add_neighbor(&mut self, destination: String, cost: i32){
        self.connections.insert(destination, cost);
    }

    fn get_distance(&self, destination: &str) -> Option<&i32> {
        self.connections.get(destination)
    }
}

fn parse_input(path: &str) -> HashMap<String, Node>{
    let mut nodes = HashMap::new();
    let data = fs::read_to_string(path).expect("Unable to read file");

    for line in data.lines(){

        let parts: Vec<&str> = line.split_whitespace().collect();
        let origin = parts[0].to_string();
        let destination = parts[2].to_string();
        let cost = parts[4].parse().expect("Could not parse into number");

        nodes.entry(origin.clone()).or_insert_with(Node::new).add_neighbor(destination.clone(), cost);

        nodes.entry(destination.clone()).or_insert_with(Node::new).add_neighbor(origin.clone(), cost);
    }
    nodes
}

fn find_shortest_path(nodes: &HashMap<String, Node>) -> (Vec<String>, i32) {
    let cities: Vec<String> = nodes.keys().cloned().collect();
    let mut shortest_distance = i32::MAX;
    let mut best_route = Vec::new();

    for route in cities.iter().permutations(cities.len()){
        let mut current_distance = 0;
        let mut valid = true;

        for i in 0..route.len() -1 {
            let city = route[i];
            let next_city = route[i+1];

            if let Some(node) = nodes.get(city) {
                if let Some(&dist) = node.get_distance(next_city) {
                    current_distance += dist;
                } else {
                    valid = false;
                    break;
                }
            }
        }
        if valid && current_distance < shortest_distance {
            shortest_distance = current_distance;
            best_route = route.iter().map(|s| s.to_string()).collect();
        }
    }
    (best_route, shortest_distance)
}

fn find_longest_path(nodes: &HashMap<String, Node>) -> (Vec<String>, i32) {
    let cities: Vec<String> = nodes.keys().cloned().collect();
    let mut longest_distance = i32::MIN;
    let mut best_route = Vec::new();

    for route in cities.iter().permutations(cities.len()){
        let mut current_distance = 0;
        let mut valid = true;

        for i in 0..route.len() -1 {
            let city = route[i];
            let next_city = route[i+1];

            if let Some(node) = nodes.get(city) {
                if let Some(&dist) = node.get_distance(next_city) {
                    current_distance += dist;
                } else {
                    valid = false;
                    break;
                }
            }
        }
        if valid && current_distance > longest_distance {
            longest_distance = current_distance;
            best_route = route.iter().map(|s| s.to_string()).collect();
        }
    }
    (best_route, longest_distance)
}

fn main() {
    let nodes = parse_input("../input.txt");
    let (route, distance) = find_shortest_path(&nodes);

    println!("================= Part 1 =================");
    println!("The shortest route in the data is {} with a distance of {}", route.join(" -> "), distance);

    println!("================= Part 2 =================");
    let (route, distance) = find_longest_path(&nodes);
    println!("The longest route in the data is {} with a distance of {}", route.join(" -> "), distance);
}
