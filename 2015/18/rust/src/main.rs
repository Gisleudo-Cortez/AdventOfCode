use std::fs;

fn gen_grid(path: &str) -> Vec<Vec<char>> {
    let data = fs::read_to_string(path)
        .expect("Error parsing input file")
        .trim()
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    data
}

fn simulate_step(grid: &Vec<Vec<char>>, stuck: bool) -> (Vec<Vec<char>>, i32) {
    let rows = grid.len();
    let cols = grid[0].len();
    
    let mut simulated = grid.clone();
    let mut new_grid = grid.clone();

    // Set corner lights if stuck
    if stuck {
        for &(r, c) in &[(0, 0), (0, cols - 1), (rows - 1, 0), (rows - 1, cols - 1)] {
            new_grid[r][c] = '#';
            simulated[r][c] = '#';
        }
    }

    let neighbors = [
        (-1, -1), // top-left
        (-1, 0),  // top
        (-1, 1),  // top-right
        (0, -1),  // left
        (0, 1),   // right
        (1, -1),  // bottom-left
        (1, 0),   // bottom
        (1, 1),   // bottom-right
    ];

    for r in 0..rows {
        for c in 0..cols {
            // Skip stuck corners
            if stuck && [(0, 0), (0, cols - 1), (rows - 1, 0), (rows - 1, cols - 1)]
                .contains(&(r, c))
            {
                continue;
            }

            let n_lights = neighbors
                .iter()
                .filter(|&&(dr, dc)| {
                    let rr = r as i32 + dr;
                    let cc = c as i32 + dc;
                    rr >= 0 && rr < rows as i32 && cc >= 0 && cc < cols as i32
                        && simulated[rr as usize][cc as usize] == '#'
                })
                .count();

            if simulated[r][c] == '#' && !(n_lights == 2 || n_lights == 3) {
                new_grid[r][c] = '.';
            } else if simulated[r][c] == '.' && n_lights == 3 {
                new_grid[r][c] = '#';
            }
        }
    }

    let lights_on = new_grid
        .iter()
        .map(|r| r.iter().filter(|&&c| c == '#').count() as i32)
        .sum();

    (new_grid, lights_on)
}

fn simulate_n_steps(grid: &Vec<Vec<char>>, n_steps: i32, stuck: bool) -> i32 {
    let mut grid = grid.clone();
    let mut final_count = 0;
    
    for _ in 0..n_steps {
        let (new_grid, count) = simulate_step(&grid, stuck);
        grid = new_grid;
        final_count = count;
    }
    
    final_count
}

fn main() {
    let grid = gen_grid("../input.txt");
    let sep = "=".repeat(20);
    
    let total_1 = simulate_n_steps(&grid, 100, false);
    let total_2 = simulate_n_steps(&grid, 100, true);
    
    println!("{} Part 1 {}\nTotal lights on after 100 steps and not stuck: {}", sep, sep, total_1);
    println!("{} Part 2 {}\nTotal lights on after 100 steps and stuck: {}", sep, sep, total_2);
}