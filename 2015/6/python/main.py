from pathlib import Path
import numpy as np
import re
from typing import Callable, Union, List

def split_at_first_number(s: str) -> tuple[str, str]:
    """Split string into operation and coordinates parts."""
    match = re.search(r'\d', s)
    if not match:
        return s.strip(), ""
    idx = match.start()
    return s[:idx].strip(), s[idx:].strip()

def get_coords(s: str) -> list[list[int, int]]:
    """Parse coordinates string into beginning and end points."""
    if not s:
        return [[0, 0], [0, 0]]
    try:
        data = s.split()
        beginning = list(map(int, data[0].split(",")))
        end = list(map(int, data[-1].split(",")))
        return [beginning, end]
    except (ValueError, IndexError):
        raise ValueError(f"Invalid coordinate format: {s}")

def toggle_cell_part1(grid: np.ndarray, bx: int, by: int, ex: int, ey: int, target: int) -> np.ndarray:
    """Modify grid cells based on operation type for part 1 (binary lights)."""
    # Add 1 to end coordinates to make range inclusive
    ex, ey = ex + 1, ey + 1
    
    # Ensure coordinates are within grid bounds
    bx = np.clip(bx, 0, grid.shape[0])
    by = np.clip(by, 0, grid.shape[1])
    ex = np.clip(ex, 0, grid.shape[0])
    ey = np.clip(ey, 0, grid.shape[1])
    
    if target == 1:  # turn on
        grid[bx:ex, by:ey] = 1
    elif target == 0:  # turn off
        grid[bx:ex, by:ey] = 0
    else:  # toggle
        grid[bx:ex, by:ey] = 1 - grid[bx:ex, by:ey]
    
    return grid

def toggle_cell_part2(grid: np.ndarray, bx: int, by: int, ex: int, ey: int, target: int) -> np.ndarray:
    """Modify grid cells based on operation type for part 2 (brightness levels)."""
    # Add 1 to end coordinates to make range inclusive
    ex, ey = ex + 1, ey + 1
    
    # Ensure coordinates are within grid bounds
    bx = np.clip(bx, 0, grid.shape[0])
    by = np.clip(by, 0, grid.shape[1])
    ex = np.clip(ex, 0, grid.shape[0])
    ey = np.clip(ey, 0, grid.shape[1])
    
    if target == 1:  # turn on: increase by 1
        grid[bx:ex, by:ey] += 1
    elif target == 0:  # turn off: decrease by 1 but not below 0
        grid[bx:ex, by:ey] = np.maximum(0, grid[bx:ex, by:ey] - 1)
    else:  # toggle: increase by 2
        grid[bx:ex, by:ey] += 2
    
    return grid

def process_lights(data: List[str], grid: np.ndarray, toggle_func: Callable) -> int:
    """Process all light operations using the specified toggle function."""
    grid = grid.copy()  # Create a copy to avoid modifying the original
    
    for task in data:
        try:
            operation, coords = split_at_first_number(task)
            beginning, end = get_coords(coords)
            
            bx, by = beginning[0], beginning[1]
            ex, ey = end[0], end[1]
            
            if operation == "turn on":
                grid = toggle_func(grid, bx, by, ex, ey, 1)
            elif operation == "turn off":
                grid = toggle_func(grid, bx, by, ex, ey, 0)
            else:  # toggle operation
                grid = toggle_func(grid, bx, by, ex, ey, -1)
        
        except (ValueError, IndexError) as e:
            print(f"Error processing task '{task}': {e}")
            continue
    
    return int(grid.sum())

def read_input(file_path: Union[str, Path]) -> List[str]:
    """Read and parse input file."""
    try:
        return Path(file_path).read_text().strip().splitlines()
    except FileNotFoundError:
        print(f"Input file not found: {file_path}")
        return []
    except Exception as e:
        print(f"Error reading input file {file_path}: {e}")
        return []

def main() -> None:
    rows, cols = 1000, 1000
    grid = np.zeros((rows, cols))
    
    # Read input files
    data = read_input("../input.txt")
    example = read_input("../example.txt")
    example_2 = read_input("../example_2.txt")
    
    if not data or not example:
        print("Error: Could not process input files")
        return
    
    # Process part 1
    answer_1 = process_lights(data, grid, toggle_cell_part1)
    test_1 = process_lights(example, grid, toggle_cell_part1)
    
    print(f"Part 1 - Main input result: {answer_1}")
    print(f"Part 1 - Example input result: {test_1}")
    
    # Process part 2
    answer_2 = process_lights(data, grid, toggle_cell_part2)
    test_2 = process_lights(example_2, grid, toggle_cell_part2)
    
    print(f"Part 2 - Main input result: {answer_2}")
    print(f"Part 2 - Example input result: {test_2}")

if __name__ == "__main__":
    main()