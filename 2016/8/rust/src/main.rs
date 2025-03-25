use std::fs;

const WIDTH: usize = 50;
const HEIGHT: usize = 6;

fn count_lit_pixels(grid: Vec<Vec<char>>) -> i32 {
    grid.iter().flatten().filter(|&&c| c == '#').count() as i32
}

fn update_grid(instruction: &str, mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    if instruction.starts_with("rect") {
        let dims = instruction[5..]
            .split('x')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let (a, b) = (dims[0], dims[1]);
        for y in 0..b {
            for x in 0..a {
                grid[y][x] = '#';
            }
        }
    } else if instruction.starts_with("rotate row") {
        let parts: Vec<_> = instruction.split_whitespace().collect();
        let row = parts[2][2..].parse::<usize>().unwrap();
        let by = parts[4].parse::<usize>().unwrap();
        let mut new_row = vec!['.'; WIDTH];
        for x in 0..WIDTH {
            new_row[(x + by) % WIDTH] = grid[row][x];
        }
        grid[row] = new_row;
    } else if instruction.starts_with("rotate column") {
        let parts: Vec<_> = instruction.split_whitespace().collect();
        let col = parts[2][2..].parse::<usize>().unwrap();
        let by = parts[4].parse::<usize>().unwrap();
        let mut new_col = ['.'; HEIGHT];
        for y in 0..HEIGHT {
            new_col[(y + by) % HEIGHT] = grid[y][col];
        }
        for y in 0..HEIGHT {
            grid[y][col] = new_col[y];
        }
    }
    grid
}

fn read_grid(grid: &[Vec<char>]) {
    println!("\nDisplay:");
    for row in grid {
        let line: String = row.iter().collect();
        println!("{line}");
    }
}

fn main() {
    let binding = fs::read_to_string("../input.txt").expect("Error reading input");
    let instructions: Vec<_> = binding.trim().lines().collect();
    let mut grid = vec![vec!['.'; WIDTH]; HEIGHT];
    let sep = "=".repeat(20);
    println!("{sep} Part 1 {sep}");
    for instruction in instructions {
        grid = update_grid(instruction, grid)
    }
    let part1 = count_lit_pixels(grid.clone());
    println!("Total number of lit pixels: {part1}");
    println!("{sep} Part 2 {sep}");
    read_grid(&grid);
}
