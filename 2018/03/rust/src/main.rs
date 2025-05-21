use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug)]
struct Claim {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

fn parse_claims(path: &str) -> Vec<Claim> {
    let input = fs::read_to_string(path).expect("Error reading input file");
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let id = parts[0][1..].parse().unwrap();
            let coords: Vec<usize> = parts[2][..parts[2].len() - 1]
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect();
            let dims: Vec<usize> = parts[3].split('x').map(|x| x.parse().unwrap()).collect();
            Claim {
                id,
                left: coords[0],
                top: coords[1],
                width: dims[0],
                height: dims[1],
            }
        })
        .collect()
}

fn mark_fabric(
    claims: &[Claim],
) -> (
    HashMap<(usize, usize), usize>,
    HashMap<usize, HashSet<(usize, usize)>>,
) {
    let mut fabric = HashMap::new();
    let mut claim_map = HashMap::new();

    for claim in claims {
        let mut positions = HashSet::new();
        for i in claim.left..claim.left + claim.width {
            for j in claim.top..claim.top + claim.height {
                *fabric.entry((i, j)).or_insert(0) += 1;
                positions.insert((i, j));
            }
        }
        claim_map.insert(claim.id, positions);
    }

    (fabric, claim_map)
}

fn count_overlaps(fabric: &HashMap<(usize, usize), usize>) -> usize {
    fabric.values().filter(|&&count| count > 1).count()
}

fn find_non_overlapping(
    claim_map: &HashMap<usize, HashSet<(usize, usize)>>,
    fabric: &HashMap<(usize, usize), usize>,
) -> usize {
    for (id, positions) in claim_map {
        if positions.iter().all(|pos| fabric[pos] == 1) {
            return *id;
        }
    }
    0
}
fn main() {
    let path = "../input.txt";
    let sep = "=".repeat(20);
    let claims = parse_claims(path);
    let (fabric, claim_map) = mark_fabric(&claims);

    let part1_result = count_overlaps(&fabric);
    let part2_result = find_non_overlapping(&claim_map, &fabric);

    println!("{sep} Part 1 {sep}");
    println!("Overlapping square inches: {}", part1_result);
    println!("{sep} Part 2 {sep}");
    println!("Non-overlapping claim ID: {}", part2_result);
}
