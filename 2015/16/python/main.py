# from pathlib import Path
# from typing import Dict

# def parse_sue_info(line: str) -> tuple[int, Dict[str, int]]:
#     first_split = line.split(': ', 1)
#     sue_number = int(first_split[0].split()[1])
    
#     props = {}
#     items = first_split[1].split(', ')
#     for item in items:
#         key, value = item.split(': ')
#         props[key] = int(value)
    
#     return sue_number, props

# def is_matching_property(key: str, sue_value: int, aunt_value: int) -> bool:
#     # For cats and trees, the reading should be greater than the value
#     if key in ['cats', 'trees']:
#         return sue_value > aunt_value
#     # For pomeranians and goldfish, the reading should be less than the value
#     elif key in ['pomeranians', 'goldfish']:
#         return sue_value < aunt_value
#     # For all other properties, exact match is required
#     else:
#         return sue_value == aunt_value

# def main() -> None:
#     aunt = {
#         "children": 3,
#         "cats": 7,
#         "samoyeds": 2,
#         "pomeranians": 3,
#         "akitas": 0,
#         "vizslas": 0,
#         "goldfish": 5,
#         "trees": 3,
#         "cars": 2,
#         "perfumes": 1
#     }
    
#     data = Path("../input.txt").read_text().strip().splitlines()
    
#     print("=" * 20, " Part 1 ", "=" * 20)
#     for line in data:
#         sue_number, properties = parse_sue_info(line)
#         if all(aunt[key] == value for key, value in properties.items()):
#             print(f"The number of the Sue that got you the gift is: {sue_number}")
#             break
            
#     print("=" * 20, " Part 2 ", "=" * 20)
#     for line in data:
#         sue_number, properties = parse_sue_info(line)
#         if all(is_matching_property(key, value, aunt[key]) 
#                for key, value in properties.items()):
#             print(f"The number of the real Aunt Sue is: {sue_number}")
#             break

# if __name__ == "__main__":
#     main()

from dataclasses import dataclass
from pathlib import Path
from typing import Dict, Iterator


@dataclass
class Sue:
    number: int
    properties: Dict[str, int]


class GiftAnalyzer:
    MFCSAM_READINGS = {
        "children": 3,
        "cats": 7,
        "samoyeds": 2,
        "pomeranians": 3,
        "akitas": 0,
        "vizslas": 0,
        "goldfish": 5,
        "trees": 3,
        "cars": 2,
        "perfumes": 1,
    }

    GREATER_THAN_PROPS = {'cats', 'trees'}
    LESS_THAN_PROPS = {'pomeranians', 'goldfish'}

    def __init__(self, input_file: Path):
        self.sues = self._parse_input(input_file)

    def _parse_input(self, input_file: Path) -> list[Sue]:
        sues = []
        for line in input_file.read_text().strip().splitlines():
            sue_part, props_part = line.split(': ', 1)
            number = int(sue_part.split()[1])
            
            properties = {}
            for prop in props_part.split(', '):
                name, value = prop.split(': ')
                properties[name] = int(value)
            
            sues.append(Sue(number, properties))
        return sues

    def _is_exact_match(self, sue: Sue) -> bool:
        return all(
            self.MFCSAM_READINGS[prop] == value
            for prop, value in sue.properties.items()
        )

    def _is_range_match(self, sue: Sue) -> bool:
        for prop, value in sue.properties.items():
            reading = self.MFCSAM_READINGS[prop]
            
            if prop in self.GREATER_THAN_PROPS and value <= reading:
                return False
            elif prop in self.LESS_THAN_PROPS and value >= reading:
                return False
            elif prop not in self.GREATER_THAN_PROPS | self.LESS_THAN_PROPS and value != reading:
                return False
        
        return True

    def find_exact_match(self) -> int:
        for sue in self.sues:
            if self._is_exact_match(sue):
                return sue.number
        return -1

    def find_range_match(self) -> int:
        for sue in self.sues:
            if self._is_range_match(sue):
                return sue.number
        return -1


def main() -> None:
    analyzer = GiftAnalyzer(Path("../input.txt"))

    print("=" * 20, "Part 1", "=" * 20)
    if exact_match := analyzer.find_exact_match():
        print(f"The number of the Sue that got you the gift is: {exact_match}")

    print("=" * 20, "Part 2", "=" * 20)
    if range_match := analyzer.find_range_match():
        print(f"The number of the real Aunt Sue is: {range_match}")


if __name__ == "__main__":
    main()