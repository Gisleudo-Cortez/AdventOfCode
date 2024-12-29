use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new() -> Self {
        Position { x: 0, y: 0 }
    }

    fn move_direction(&self, direction: Direction) -> Self {
        let (dx, dy) = direction.as_offset();
        Position {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        }
    }

    fn as_offset(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

struct DeliverySystem {
    visited: HashSet<Position>,
}

impl DeliverySystem {
    fn new() -> Self {
        let mut visited = HashSet::new();
        visited.insert(Position::new());
        DeliverySystem { visited }
    }

    fn deliver_presents(&mut self, movements: &str) -> usize {
        let mut current_pos = Position::new();
        
        for c in movements.chars() {
            if let Some(direction) = Direction::from_char(c) {
                current_pos = current_pos.move_direction(direction);
                self.visited.insert(current_pos);
            }
        }
        
        self.visited.len()
    }

    fn deliver_presents_with_robo(&mut self, movements: &str) -> usize {
        let mut santa_pos = Position::new();
        let mut robo_pos = Position::new();
        
        for (i, c) in movements.chars().enumerate() {
            if let Some(direction) = Direction::from_char(c) {
                let current_pos = if i % 2 == 0 {
                    santa_pos = santa_pos.move_direction(direction);
                    santa_pos
                } else {
                    robo_pos = robo_pos.move_direction(direction);
                    robo_pos
                };
                self.visited.insert(current_pos);
            }
        }
        
        self.visited.len()
    }
}

fn read_input(file_path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}

fn main() -> Result<(), std::io::Error> {
    let movements = read_input("../input.txt")?;

    // Part 1: Santa alone
    let mut delivery_system = DeliverySystem::new();
    let houses_visited = delivery_system.deliver_presents(&movements);
    println!("Part 1: {}", houses_visited);

    // Part 2: Santa and Robo-Santa
    let mut delivery_system = DeliverySystem::new();
    let houses_visited = delivery_system.deliver_presents_with_robo(&movements);
    println!("Part 2: {}", houses_visited);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_santa() {
        let mut delivery_system = DeliverySystem::new();
        assert_eq!(delivery_system.deliver_presents(">"), 2);
        
        let mut delivery_system = DeliverySystem::new();
        assert_eq!(delivery_system.deliver_presents("^>v<"), 4);
    }

    #[test]
    fn test_santa_and_robo() {
        let mut delivery_system = DeliverySystem::new();
        assert_eq!(delivery_system.deliver_presents_with_robo("^v"), 3);
        
        let mut delivery_system = DeliverySystem::new();
        assert_eq!(delivery_system.deliver_presents_with_robo("^>v<"), 3);
    }

    #[test]
    fn test_position_movement() {
        let pos = Position::new();
        assert_eq!(pos.move_direction(Direction::Right), Position { x: 1, y: 0 });
        assert_eq!(pos.move_direction(Direction::Up), Position { x: 0, y: 1 });
    }
}