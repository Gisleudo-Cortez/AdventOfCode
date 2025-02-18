from pathlib import Path
from typing import List, Tuple


def gen_grid(data: str) -> List[List[str]]:
    """ Generate a grid and return it and its rows and columns numbers"""
    grid = [list(line) for line in data.split("\n")]
    return grid

def simulate_step(data: List[List[str]], stuck: bool = False) -> Tuple[List[List[str]],int]:
    """ Simulate the behaviour of the ligths and return the grid and the number of ligths on at the final step"""
    # A light which is on stays on when 2 or 3 neighbors are on, and turns off otherwise.
    # A light which is off turns on if exactly 3 neighbors are on, and stays off otherwise.
    rows = len(data)
    cols = len(data[0])
    simulated = [[cell for cell in row] for row in data]
    new_grid = [[cell for cell in row] for row in data]

    # If lights are stuck, ensure corners are on in both grids
    if stuck:
        for grid in [simulated, new_grid]:
            grid[0][0] = "#"  # top-left
            grid[0][cols-1] = "#"  # top-right
            grid[rows-1][0] = "#"  # bottom-left
            grid[rows-1][cols-1] = "#"  # bottom-right

    neighbors = [(-1, -1),# top left
                 (0, -1), # top
                 (1, -1), # top right
                 (1, 0),  # right
                 (1, 1),  # bottom right
                 (0, 1),  # bottom
                 (-1, 1), # bottom left
                 (-1, 0)] # left

    for r in range(rows):
        for c in range(cols):

            curr_pos = (r,c)
            n_on_lights = 0

            # Skip if it's a stuck corner light
            if stuck and ((r == 0 and c == 0) or 
                         (r == 0 and c == cols-1) or 
                         (r == rows-1 and c == 0) or 
                         (r == rows-1 and c == cols-1)):
                continue

            for x, y in neighbors:
                rx = r+x
                cy= c+y
                # check if neighbor is within bounds
                if not (0 <= rx < rows and 0 <= cy < cols):
                    continue
                if simulated[rx][cy] == "#":
                    n_on_lights += 1

            # switch logic
            if simulated[curr_pos[0]][curr_pos[1]] == "#" and not n_on_lights in [2,3]:
                new_grid[curr_pos[0]][curr_pos[1]] = "."

            if simulated[curr_pos[0]][curr_pos[1]] == "." and n_on_lights == 3:
                new_grid[curr_pos[0]][curr_pos[1]] = "#"

    lights_on = sum(row.count("#") for row in new_grid)
    return new_grid, lights_on

def simulate_n_step(grid: List[List[str]], n_sim: int, stuck: bool = False) -> Tuple[List[List[str]],int]:
    out = grid
    for _ in range(n_sim):
        new = [[cell for cell in row] for row in out]
        out, n = simulate_step(new, stuck)
    return out, n

def main() -> None:
    data = Path("../input.txt").read_text().strip()
    example = Path("../example.txt").read_text().strip()
    grid = gen_grid(data)
    print("=" * 20, "Part 1", "=" * 20)
    _, total = simulate_n_step(grid,100)
    print(f"Total ligth on after 100 steps: {total}")
    print("=" * 20, "Part 2", "=" * 20)
    _, total = simulate_n_step(grid,100, True)
    print(f"Total ligth on after 100 steps: {total}")


if __name__ == "__main__":
    main()