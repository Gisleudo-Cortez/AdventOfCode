use std::collections::HashMap;
use std::fs;

fn rotate(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let n = grid.len();
    (0..n)
        .map(|i| (0..n).rev().map(|j| grid[j][i]).collect())
        .collect()
}

fn flip(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    grid.iter()
        .map(|row| row.iter().rev().cloned().collect())
        .collect()
}

fn variations(grid: &[Vec<char>]) -> Vec<Vec<Vec<char>>> {
    let mut variants = Vec::new();
    let mut current = grid.to_vec();
    for _ in 0..4 {
        variants.push(current.clone());
        variants.push(flip(&current));
        current = rotate(&current);
    }
    variants
}

fn parse_pattern(s: &str) -> Vec<Vec<char>> {
    s.split('/')
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>()
}

fn pattern_to_string(pat: &[Vec<char>]) -> String {
    pat.iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("/")
}

fn enhance(grid: &[Vec<char>], rules: &HashMap<String, Vec<Vec<char>>>) -> Vec<Vec<char>> {
    let size = grid.len();
    let block_size = if size % 2 == 0 { 2 } else { 3 };
    let new_block_size = block_size + 1;
    let blocks_per_side = size / block_size;
    let mut new_grid =
        vec![vec![' '; blocks_per_side * new_block_size]; blocks_per_side * new_block_size];

    for i in 0..blocks_per_side {
        for j in 0..blocks_per_side {
            let block: Vec<Vec<char>> = (0..block_size)
                .map(|x| {
                    (0..block_size)
                        .map(|y| grid[i * block_size + x][j * block_size + y])
                        .collect()
                })
                .collect();

            let key = variations(&block)
                .into_iter()
                .map(|v| pattern_to_string(&v))
                .find_map(|k| rules.get(&k).cloned())
                .expect("No matching pattern found");

            for x in 0..new_block_size {
                for y in 0..new_block_size {
                    new_grid[i * new_block_size + x][j * new_block_size + y] = key[x][y];
                }
            }
        }
    }
    new_grid
}

fn count_pixels(grid: &[Vec<char>]) -> usize {
    grid.iter().flatten().filter(|&&c| c == '#').count()
}

fn load_rules(path: &str) -> HashMap<String, Vec<Vec<char>>> {
    let content = fs::read_to_string(path).expect("Failed to read input file");
    let mut rules = HashMap::new();
    for line in content.lines() {
        let (input, output) = line.split_once(" => ").unwrap();
        let output_grid = parse_pattern(output);
        for variant in variations(&parse_pattern(input)) {
            rules.insert(pattern_to_string(&variant), output_grid.clone());
        }
    }
    rules
}

fn solve(path: &str, iterations: usize) -> usize {
    let rules = load_rules(path);
    let mut grid = parse_pattern(".#./..#/###");
    for _ in 0..iterations {
        grid = enhance(&grid, &rules);
    }
    count_pixels(&grid)
}

fn main() {
    let path = "../input.txt";
    let sep = "=".repeat(20);
    let part1 = solve(path, 5);
    let part2 = solve(path, 18);
    println!("{sep} Part 1 {sep}");
    println!("Pixels after 5 iterations: {part1}");
    println!("{sep} Part 2 {sep}");
    println!("Pixels after 18 iterations: {part2}");
}
