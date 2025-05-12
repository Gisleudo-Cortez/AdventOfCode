use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, Clone)]
struct Particle {
    position: (i32, i32, i32),
    velocity: (i32, i32, i32),
    acceleration: (i32, i32, i32),
}

fn parse_vector(components: &str) -> (i32, i32, i32) {
    let start = components.find("<").unwrap() + 1;
    let end = components.find(">").unwrap();
    let coords: Vec<i32> = components[start..end]
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    (coords[0], coords[1], coords[2])
}

fn parse_input(path: &str) -> Vec<Particle> {
    let binding = fs::read_to_string(path).expect("Error reading input file");
    let mut particles: Vec<Particle> = Vec::new();
    for l in binding.lines() {
        let parts: Vec<&str> = l.split(", ").collect();
        let pos = parse_vector(parts[0]);
        let vel = parse_vector(parts[1]);
        let acc = parse_vector(parts[2]);
        particles.push(Particle {
            position: pos,
            velocity: vel,
            acceleration: acc,
        });
    }
    particles
}

fn abs_norm(p: &Particle) -> (i32, i32, i32) {
    let pos = p.position.0.abs() + p.position.1.abs() + p.position.2.abs();
    let vel = p.velocity.0.abs() + p.velocity.1.abs() + p.velocity.2.abs();
    let acc = p.acceleration.0.abs() + p.acceleration.1.abs() + p.acceleration.2.abs();
    (pos, vel, acc)
}

fn closest_to_origin(particles: &[Particle]) -> usize {
    let mut vec_part: Vec<(usize, i32, i32, i32)> = Vec::new();
    for (i, p) in particles.iter().enumerate() {
        let (pos, vel, acc) = abs_norm(p);
        vec_part.push((i, pos, vel, acc));
    }
    // sort handling ties acc > vel > pos
    vec_part.sort_unstable_by(|a, b| (a.3, a.2, a.1).cmp(&(b.3, b.2, b.1)));
    // index of the original particle closest to the origin
    vec_part[0].0
}

fn tick(particles: &mut [Particle]) -> HashSet<usize> {
    for part in &mut *particles {
        part.velocity = (
            part.velocity.0 + part.acceleration.0,
            part.velocity.1 + part.acceleration.1,
            part.velocity.2 + part.acceleration.2,
        );
        part.position = (
            part.position.0 + part.velocity.0,
            part.position.1 + part.velocity.1,
            part.position.2 + part.velocity.2,
        );
    }
    let mut position_map: HashMap<(i32, i32, i32), Vec<usize>> = HashMap::new();
    for (i, p) in particles.iter().enumerate() {
        position_map.entry(p.position).or_default().push(i);
    }
    let mut colliding_indices: HashSet<usize> = HashSet::new();
    for ind in position_map.values() {
        if ind.len() > 1 {
            for index in ind {
                colliding_indices.insert(*index);
            }
        }
    }
    colliding_indices
}

fn simulate(particles: &mut Vec<Particle>) -> usize {
    let max_ticks = 1000;
    let mut no_colision_ticks = 0;

    loop {
        let collision = tick(particles);
        *particles = particles
            .iter()
            .enumerate()
            .filter(|(i, _)| !collision.contains(i))
            .map(|(_, p)| p.clone())
            .collect();

        if collision.is_empty() {
            no_colision_ticks += 1;
        } else {
            no_colision_ticks = 0;
        }
        if no_colision_ticks > max_ticks {
            break;
        }
    }
    particles.len()
}

fn main() {
    let path = "../input.txt";
    let sep = "=".repeat(20);
    let particles = parse_input(path);
    println!("{sep} Part 1 {sep}");
    let pt1 = closest_to_origin(&particles);
    println!("The paticle closest to the origin will be particle: {pt1}");
    println!("{sep} Part 2 {sep}");
    let mut part2 = particles.clone();
    let pt2 = simulate(&mut part2);
    println!("The number of remaining particles after stable: {pt2}");
}
