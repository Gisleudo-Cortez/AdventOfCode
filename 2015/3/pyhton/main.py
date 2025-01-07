# from pathlib import Path

# with Path("../input.txt").open() as file:
#     data = file.read()

# directions = {
#     ">" : (1,0),
#     "<" : (-1,0),
#     "^" : (0,1),
#     "v" : (0,-1),
# }

# def calculate_unique_houses(movements: str) -> int:
#     origin = (0,0)
#     locations = set([origin])
#     curr = origin
#     for d in movements:
#         loc = directions[d]
#         curr = tuple(map(sum, zip(loc, curr)))
#         if curr not in locations:
#             locations.add(curr)
#     return len(locations)

# def calculate_unique_houses_robo_santa(movements: str) -> int:
#     origin = (0,0)
#     locations = set([origin])
#     santa_pos = origin
#     robo_pos = origin
#     santa = True

#     for d in movements:
#         loc = directions[d]
        
#         if santa:
#             santa_pos = tuple(map(sum, zip(loc, santa_pos)))
#             locations.add(santa_pos)
#             santa = not santa
#         else:
#             robo_pos = tuple(map(sum, zip(loc, robo_pos)))
#             locations.add(robo_pos)
#             santa = not santa
#     return len(locations)


# def main() -> None:
#     print(f"Part 1: ", calculate_unique_houses(data))
#     print(f"Part 2: ", calculate_unique_houses_robo_santa(data))

# if __name__ == "__main__":
#     main()

from pathlib import Path
from dataclasses import dataclass
from typing import Dict, Set, Tuple, List
from enum import Enum


class Direction(Enum):
    RIGHT = (1, 0)
    LEFT = (-1, 0)
    UP = (0, 1)
    DOWN = (0, -1)


@dataclass
class Position:
    x: int
    y: int
    
    def move(self, direction: Tuple[int, int]) -> 'Position':
        """Move to a new position based on direction."""
        dx, dy = direction
        return Position(self.x + dx, self.y + dy)
    
    def to_tuple(self) -> Tuple[int, int]:
        """Convert position to tuple for set storage."""
        return (self.x, self.y)


class DeliverySystem:
    def __init__(self):
        self.direction_map = {
            '>': Direction.RIGHT,
            '<': Direction.LEFT,
            '^': Direction.UP,
            'v': Direction.DOWN
        }
        self.visited: Set[Tuple[int, int]] = set()
        
    def _process_movement(self, position: Position, direction_char: str) -> Position:
        """Process a single movement and return new position."""
        direction = self.direction_map[direction_char].value
        new_position = position.move(direction)
        self.visited.add(new_position.to_tuple())
        return new_position

    def deliver_presents(self, movements: str) -> int:
        """Calculate unique houses visited by single Santa."""
        self.visited.clear()
        position = Position(0, 0)
        self.visited.add(position.to_tuple())
        
        for direction in movements:
            position = self._process_movement(position, direction)
            
        return len(self.visited)
    
    def deliver_presents_with_robo(self, movements: str) -> int:
        """Calculate unique houses visited by Santa and Robo-Santa."""
        self.visited.clear()
        santa_pos = Position(0, 0)
        robo_pos = Position(0, 0)
        self.visited.add(santa_pos.to_tuple())
        
        for i, direction in enumerate(movements):
            if i % 2 == 0:
                santa_pos = self._process_movement(santa_pos, direction)
            else:
                robo_pos = self._process_movement(robo_pos, direction)
                
        return len(self.visited)


def read_input(filepath: str) -> str:
    """Read movement instructions from input file."""
    return Path(filepath).read_text().strip()


def main() -> None:
    movements = read_input("../input.txt")
    delivery_system = DeliverySystem()
    
    print(f"Part 1: {delivery_system.deliver_presents(movements)}")
    print(f"Part 2: {delivery_system.deliver_presents_with_robo(movements)}")


if __name__ == "__main__":
    main()