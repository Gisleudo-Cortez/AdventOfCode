use std::{
    collections::{HashSet, VecDeque},
    fs,
};

fn is_wall(x: i32, y: i32, fav_number: i32) -> bool {
    let base = (x * x + 3 * x + 2 * x * y + y + y * y) + fav_number;
    let bin_rep = format!("{:b}", base);
    let mut n_ones = 0;
    for bit in bin_rep.chars() {
        if bit == '1' {
            n_ones += 1;
        }
    }
    n_ones % 2 != 0
}

fn gen_maze(fav_number: i32, max_x: usize, max_y: usize) -> Vec<Vec<char>> {
    let mut maze: Vec<Vec<char>> = Vec::new();
    for y in 0..=max_y {
        let mut row: Vec<char> = Vec::new();
        for x in 0..=max_x {
            if is_wall(x as i32, y as i32, fav_number) {
                row.push('#');
            } else {
                row.push('.');
            }
        }
        maze.push(row);
    }
    maze
}

fn solve_maze(maze: &[Vec<char>], start: (usize, usize), target: (usize, usize)) -> Option<usize> {
    let height = maze.len();
    if height == 0 {
        return None; // Empty maze
    }
    let width = maze[0].len();
    if width == 0 {
        return None; // Empty maze row
    }

    if start.1 >= height || start.0 >= width || maze[start.1][start.0] == '#' {
        return None; // Invalid start
    }
    if target.1 >= height || target.0 >= width || maze[target.1][target.0] == '#' {
        return None; // Invalid or blocked target
    }

    let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    queue.push_back((start, 0));
    visited.insert(start);

    let moves = [(0, -1), (0, 1), (-1, 0), (1, 0)];

    while let Some(((x, y), steps)) = queue.pop_front() {
        if (x, y) == target {
            return Some(steps); // Path found
        }

        for (dx, dy) in moves.iter() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                let nx_usize = nx as usize;
                let ny_usize = ny as usize;
                let next_coord = (nx_usize, ny_usize);

                if maze[ny_usize][nx_usize] != '#' && !visited.contains(&next_coord) {
                    visited.insert(next_coord);
                    queue.push_back((next_coord, steps + 1));
                }
            }
        }
    }

    None
}

fn count_reachable_locations(maze: &[Vec<char>], start: (usize, usize), max_steps: usize) -> usize {
    let height = maze.len();
    if height == 0 {
        return 0;
    }
    let width = maze[0].len();
    if width == 0 {
        return 0;
    }

    let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    queue.push_back((start, 0));
    visited.insert(start);

    let moves = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    while let Some(((x, y), steps)) = queue.pop_front() {
        if steps < max_steps {
            for (dx, dy) in moves.iter() {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                    let nx_usize = nx as usize;
                    let ny_usize = ny as usize;
                    let next_coord = (nx_usize, ny_usize);

                    if maze[ny_usize][nx_usize] != '#' && visited.insert(next_coord) {
                        queue.push_back((next_coord, steps + 1));
                    }
                }
            }
        }
    }

    visited.len()
}

fn main() {
    let input = fs::read_to_string("../input.txt").expect("Error reading input file");
    let fav_number = input.trim().parse::<i32>().expect("Failed parsing input");
    let target = (31, 39);
    let start = (1, 1);
    let sep = "=".repeat(20);
    println!("{sep} Part 1 {sep}");
    let maze = gen_maze(fav_number, target.0 + 5, target.1 + 5);
    let part1 = solve_maze(&maze, start, target)
        .expect("Failed to find a path to the target (target might be unreachable or blocked)");
    println!("The shortest path to the target is: {part1}");
    println!("{sep} Part 2 {sep}");
    let part2 = count_reachable_locations(&maze, start, 50);
    println!("The number of unique reachable locations in 50 steps is: {part2}");
}
