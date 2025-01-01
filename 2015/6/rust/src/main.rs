use std::fs;
use std::path::Path;
use regex::Regex;

#[derive(Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Operation {
    start: Coordinate,
    end: Coordinate,
    op_type: OperationType,
}

#[derive(Debug)]
enum OperationType {
    TurnOn,
    TurnOff,
    Toggle,
}

fn split_at_first_number(s: &str) -> (String, String) {
    let re = Regex::new(r"\d").unwrap();
    if let Some(mat) = re.find(s) {
        let idx = mat.start();
        (s[..idx].trim().to_string(), s[idx..].trim().to_string())
    } else {
        (s.trim().to_string(), String::new())
    }
}

fn parse_coords(s: &str) -> Result<(Coordinate, Coordinate), Box<dyn std::error::Error>> {
    if s.is_empty() {
        return Ok((Coordinate { x: 0, y: 0 }, Coordinate { x: 0, y: 0 }));
    }

    let parts: Vec<&str> = s.split_whitespace().collect();
    let start: Vec<usize> = parts[0]
        .split(',')
        .map(|n| n.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;
    let end: Vec<usize> = parts[parts.len() - 1]
        .split(',')
        .map(|n| n.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;

    Ok((
        Coordinate {
            x: start[0],
            y: start[1],
        },
        Coordinate { x: end[0], y: end[1] },
    ))
}

fn parse_operation(line: &str) -> Result<Operation, Box<dyn std::error::Error>> {
    let (op_str, coords_str) = split_at_first_number(line);
    let (start, end) = parse_coords(&coords_str)?;

    let op_type = match op_str.as_str() {
        "turn on" => OperationType::TurnOn,
        "turn off" => OperationType::TurnOff,
        _ => OperationType::Toggle,
    };

    Ok(Operation {
        start,
        end,
        op_type,
    })
}

fn toggle_cell_part1(grid: &mut [Vec<u32>], op: &Operation) {
    let end_x = op.end.x.min(grid.len() - 1);
    let end_y = op.end.y.min(grid[0].len() - 1);

    grid.iter_mut()
        .skip(op.start.x)
        .take(end_x - op.start.x + 1)
        .for_each(|row| {
            row.iter_mut()
                .skip(op.start.y)
                .take(end_y - op.start.y + 1)
                .for_each(|cell| {
                    *cell = match op.op_type {
                        OperationType::TurnOn => 1,
                        OperationType::TurnOff => 0,
                        OperationType::Toggle => 1 - *cell,
                    };
                });
        });
}

fn toggle_cell_part2(grid: &mut [Vec<u32>], op: &Operation) {
    let end_x = op.end.x.min(grid.len() - 1);
    let end_y = op.end.y.min(grid[0].len() - 1);

    grid.iter_mut()
        .skip(op.start.x)
        .take(end_x - op.start.x + 1)
        .for_each(|row| {
            row.iter_mut()
                .skip(op.start.y)
                .take(end_y - op.start.y + 1)
                .for_each(|cell| {
                    match op.op_type {
                        OperationType::TurnOn => *cell += 1,
                        OperationType::TurnOff => *cell = cell.saturating_sub(1),// lock the result to 0 in case of overflow
                        OperationType::Toggle => *cell += 2,
                    };
                });
        });
}

fn process_lights<F>(data: &[String], grid: &mut [Vec<u32>], toggle_func: F) -> u32
where
    F: Fn(&mut [Vec<u32>], &Operation),
{
    for line in data {
        match parse_operation(line) {
            Ok(op) => toggle_func(grid, &op),
            Err(e) => eprintln!("Error processing task '{}': {}", line, e),
        }
    }

    grid.iter()
        .flat_map(|row| row.iter())
        .sum()
}

fn read_input<P: AsRef<Path>>(path: P) -> Result<Vec<String>, std::io::Error> {
    fs::read_to_string(path)
        .map(|content| {
            content
                .lines()
                .map(String::from)
                .collect()
        })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const GRID_SIZE: usize = 1000;
    let mut grid = vec![vec![0; GRID_SIZE]; GRID_SIZE];

    // Read input files
    let data = read_input("../input.txt")?;
    let example = read_input("../example.txt")?;
    let example_2 = read_input("../example_2.txt")?;

    // Process part 1
    let answer_1 = process_lights(&data, &mut grid, toggle_cell_part1);
    grid = vec![vec![0; GRID_SIZE]; GRID_SIZE]; // Reset grid
    let test_1 = process_lights(&example, &mut grid, toggle_cell_part1);

    println!("Part 1 - Main input result: {}", answer_1);
    println!("Part 1 - Example input result: {}", test_1);

    // Process part 2
    grid = vec![vec![0; GRID_SIZE]; GRID_SIZE]; // Reset grid
    let answer_2 = process_lights(&data, &mut grid, toggle_cell_part2);
    grid = vec![vec![0; GRID_SIZE]; GRID_SIZE]; // Reset grid
    let test_2 = process_lights(&example_2, &mut grid, toggle_cell_part2);

    println!("Part 2 - Main input result: {}", answer_2);
    println!("Part 2 - Example input result: {}", test_2);

    Ok(())
}