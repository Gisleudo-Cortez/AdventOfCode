use std::fs;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn distance_to(&self, other: &Point) -> i32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = -dx - dy;
        dx.abs().max(dy.abs()).max(dz.abs())
    }
}

fn parse_input(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect("Error reading input file")
        .trim()
        .split(',')
        .map(|s| s.trim().to_string())
        .collect()
}

fn solve(map: &[String]) -> (i32, i32) {
    let mut current = Point::new(0, 0);
    let origin = Point::new(0, 0);
    let mut max_distance = 0;

    for dir in map {
        match &**dir {
            "n" => current.y -= 1,
            "ne" => {
                current.x += 1;
                current.y -= 1
            }
            "nw" => current.x -= 1,
            "s" => current.y += 1,
            "se" => current.x += 1,
            "sw" => {
                current.x -= 1;
                current.y += 1
            }
            _ => eprintln!("Invalid direction found: {}", dir),
        }

        let dist = origin.distance_to(&current);
        max_distance = max_distance.max(dist);
    }

    (origin.distance_to(&current), max_distance)
}

fn main() {
    let map = parse_input("../input.txt");
    let sep = "=".repeat(20);
    let (part1, part2) = solve(&map);
    println!("{sep} Part 1 {sep}");
    println!("The distance to the child process is: {part1}");
    println!("{sep} Part 2 {sep}");
    println!("The furthest distance is: {part2}");
}
