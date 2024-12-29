# with open("../input.txt") as file:
#     data = file.read().splitlines()

# def part1(data:list[str]) -> int:
#     total = 0
#     for line in data:
#         l, w, h = map(int ,line.split("x"))
#         smallest = min(l*w, w*h, h*l)
#         area = ((2*l*w) + (2*w*h) + (2*h*l)) + smallest
#         total += area
#     return total

# def part2(data:list[str]) -> int:
#     total = 0
#     for line in data:
#         l, w, h = map(int ,line.split("x"))
#         warp = sorted([l,w,h])
#         warp = warp[0] * 2 + warp[1] * 2
#         bow = l*w*h
#         total += (warp + bow)
#     return total

# def main():
#     print(f"Part 1: {part1(data)}\n")
#     print(f"Part 2: {part2(data)}\n")


# if __name__ == "__main__":
#     main()
    
from dataclasses import dataclass
from typing import List
from pathlib import Path


@dataclass
class Present:
    length: int
    width: int
    height: int
    
    @classmethod
    def from_str(cls, dimensions: str) -> "Present":
        """Create a Present instance from a dimension string."""
        l, w, h = map(int, dimensions.split("x"))
        return cls(l, w, h)
    
    @property
    def surface_area(self) -> int:
        """Calculate the required wrapping paper area."""
        sides = [
            self.length * self.width,
            self.width * self.height,
            self.height * self.length
        ]
        return 2 * sum(sides) + min(sides)
    
    @property
    def ribbon_length(self) -> int:
        """Calculate the required ribbon length."""
        dimensions = sorted([self.length, self.width, self.height])
        perimeter = 2 * (dimensions[0] + dimensions[1])
        volume = self.length * self.width * self.height
        return perimeter + volume


def read_presents(filepath: str) -> List[Present]:
    """Read and parse the input file into Present objects."""
    with Path(filepath).open() as file:
        return [Present.from_str(line) for line in file]


def calculate_total_paper(presents: List[Present]) -> int:
    """Calculate total wrapping paper needed."""
    return sum(present.surface_area for present in presents)


def calculate_total_ribbon(presents: List[Present]) -> int:
    """Calculate total ribbon needed."""
    return sum(present.ribbon_length for present in presents)


def main() -> None:
    presents = read_presents("../input.txt")
    
    print(f"Part 1: {calculate_total_paper(presents)}")
    print(f"Part 2: {calculate_total_ribbon(presents)}")


if __name__ == "__main__":
    main()