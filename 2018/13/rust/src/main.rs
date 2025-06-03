use std::collections::HashMap;
use std::fs;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Turn {
    Left,
    Straight,
    Right,
}

#[derive(Clone, Debug)]
struct Cart {
    x: usize,
    y: usize,
    dir: Direction,
    next_turn: Turn,
    active: bool,
}

impl Cart {
    fn new(x: usize, y: usize, dir: Direction) -> Self {
        Cart {
            x,
            y,
            dir,
            next_turn: Turn::Left,
            active: true,
        }
    }
}

fn parse_input(path: &str) -> (Vec<Vec<char>>, Vec<Cart>) {
    let input = fs::read_to_string(path).expect("Error reading input file");
    let mut grid = Vec::new();
    let mut carts = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            match c {
                '^' => {
                    carts.push(Cart::new(x, y, Direction::Up));
                    row.push('|');
                }
                'v' => {
                    carts.push(Cart::new(x, y, Direction::Down));
                    row.push('|');
                }
                '<' => {
                    carts.push(Cart::new(x, y, Direction::Left));
                    row.push('-');
                }
                '>' => {
                    carts.push(Cart::new(x, y, Direction::Right));
                    row.push('-');
                }
                _ => row.push(c),
            }
        }
        grid.push(row);
    }
    (grid, carts)
}

fn move_cart(cart: &mut Cart, grid: &Vec<Vec<char>>) {
    // Move one step forward
    match cart.dir {
        Direction::Up => {
            if cart.y > 0 {
                cart.y -= 1;
            } else {
                panic!("Cart moved out of bounds (up)");
            }
        }
        Direction::Down => {
            if cart.y + 1 < grid.len() {
                cart.y += 1;
            } else {
                panic!("Cart moved out of bounds (down)");
            }
        }
        Direction::Left => {
            if cart.x > 0 {
                cart.x -= 1;
            } else {
                panic!("Cart moved out of bounds (left)");
            }
        }
        Direction::Right => {
            if cart.x + 1 < grid[0].len() {
                cart.x += 1;
            } else {
                panic!("Cart moved out of bounds (right)");
            }
        }
    }

    // Update direction based on track
    let track_char = grid[cart.y][cart.x];
    match track_char {
        '/' => {
            cart.dir = match cart.dir {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Right => Direction::Up,
            };
        }
        '\\' => {
            cart.dir = match cart.dir {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            };
        }
        '+' => {
            // Handle intersection
            cart.dir = match (cart.dir, cart.next_turn) {
                // Turn Left
                (Direction::Up, Turn::Left) => Direction::Left,
                (Direction::Down, Turn::Left) => Direction::Right,
                (Direction::Left, Turn::Left) => Direction::Down,
                (Direction::Right, Turn::Left) => Direction::Up,

                // Go Straight
                (dir, Turn::Straight) => dir,

                // Turn Right
                (Direction::Up, Turn::Right) => Direction::Right,
                (Direction::Down, Turn::Right) => Direction::Left,
                (Direction::Left, Turn::Right) => Direction::Up,
                (Direction::Right, Turn::Right) => Direction::Down,
            };

            // Cycle to next turn
            cart.next_turn = match cart.next_turn {
                Turn::Left => Turn::Straight,
                Turn::Straight => Turn::Right,
                Turn::Right => Turn::Left,
            };
        }
        '-' | '|' => {
            // Straight track, no direction change
        }
        _ => panic!(
            "Cart at ({},{}) encountered invalid track: '{}'",
            cart.x, cart.y, track_char
        ),
    }
}

fn part1(initial_carts: Vec<Cart>, grid: &Vec<Vec<char>>) -> (usize, usize) {
    let mut carts = initial_carts;

    loop {
        // Sort carts by position (top-to-bottom, left-to-right)
        carts.sort_by_key(|c| (c.y, c.x));

        // Track positions after each cart moves
        let mut positions_after_move: HashMap<(usize, usize), usize> = HashMap::new();

        for i in 0..carts.len() {
            // Move the cart
            move_cart(&mut carts[i], grid);

            let pos = (carts[i].x, carts[i].y);

            // Check for collision with any cart that has already moved this tick
            if positions_after_move.contains_key(&pos) {
                // Collision detected
                return pos;
            }

            // Check for collision with carts that haven't moved yet this tick
            for j in (i + 1)..carts.len() {
                if carts[j].x == carts[i].x && carts[j].y == carts[i].y {
                    return pos;
                }
            }

            positions_after_move.insert(pos, i);
        }
    }
}

fn part2(initial_carts: Vec<Cart>, grid: &Vec<Vec<char>>) -> (usize, usize) {
    let mut carts = initial_carts;

    loop {
        // Count active carts
        let active_count = carts.iter().filter(|c| c.active).count();
        if active_count <= 1 {
            break;
        }

        // Create sorted order of active cart indices
        let mut cart_order: Vec<usize> = Vec::new();
        for (i, cart) in carts.iter().enumerate() {
            if cart.active {
                cart_order.push(i);
            }
        }
        cart_order.sort_by_key(|&i| (carts[i].y, carts[i].x));

        // Process each cart in order
        for &cart_idx in &cart_order {
            if !carts[cart_idx].active {
                continue; // Skip if cart was removed in a collision earlier this tick
            }

            // Move the cart
            move_cart(&mut carts[cart_idx], grid);

            let pos = (carts[cart_idx].x, carts[cart_idx].y);

            // Check for collisions with other active carts
            let mut collision_indices = Vec::new();
            for (i, other_cart) in carts.iter().enumerate() {
                if i != cart_idx
                    && other_cart.active
                    && other_cart.x == pos.0
                    && other_cart.y == pos.1
                {
                    collision_indices.push(i);
                }
            }

            if !collision_indices.is_empty() {
                // Mark all collided carts as inactive
                carts[cart_idx].active = false;
                for &idx in &collision_indices {
                    carts[idx].active = false;
                }
            }
        }
    }

    // Find the last remaining cart
    let last_cart = carts
        .iter()
        .find(|c| c.active)
        .expect("No carts remain active");

    (last_cart.x, last_cart.y)
}

fn main() {
    let path = "../input.txt";
    let sep = "=".repeat(20);
    let (grid, initial_carts) = parse_input(path);

    println!("{sep} Part 1 {sep}");
    let (x1, y1) = part1(initial_carts.clone(), &grid);
    println!("First collision occurs at: {x1},{y1}");

    println!("{sep} Part 2 {sep}");
    let (x2, y2) = part2(initial_carts, &grid);
    println!("Last remaining cart is at: {x2},{y2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cart_movement() {
        // Test basic movement
        let mut cart = Cart::new(1, 1, Direction::Right);
        let grid = vec![
            vec!['-', '-', '-'],
            vec!['-', '-', '-'],
            vec!['-', '-', '-'],
        ];

        move_cart(&mut cart, &grid);
        assert_eq!(cart.x, 2);
        assert_eq!(cart.y, 1);
    }

    #[test]
    fn test_intersection_turns() {
        let mut cart = Cart::new(0, 1, Direction::Right);
        let grid = vec![
            vec!['-', '|', '-'],
            vec!['-', '+', '-'],
            vec!['-', '|', '-'],
        ];

        // Move to intersection and turn left (which should be Up from Right)
        move_cart(&mut cart, &grid);
        assert_eq!(cart.x, 1);
        assert_eq!(cart.y, 1);
        assert_eq!(cart.dir, Direction::Up);
        assert_eq!(cart.next_turn, Turn::Straight);
    }
}
