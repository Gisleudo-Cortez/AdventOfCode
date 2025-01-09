from pathlib import Path
from typing import Dict

def parse_sue_info(line: str) -> tuple[int, Dict[str, int]]:
    first_split = line.split(': ', 1)
    sue_number = int(first_split[0].split()[1])
    
    props = {}
    items = first_split[1].split(', ')
    for item in items:
        key, value = item.split(': ')
        props[key] = int(value)
    
    return sue_number, props

def main() -> None:
    aunt = {
        "children": 3,
        "cats": 7,
        "samoyeds": 2,
        "pomeranians": 3,
        "akitas": 0,
        "vizslas": 0,
        "goldfish": 5,
        "trees": 3,
        "cars": 2,
        "perfumes": 1
    }
    
    data = Path("../input.txt").read_text().strip().splitlines()
    print("=" * 20, " Part 1 ", "=" * 20)
    for line in data:
        sue_number, properties = parse_sue_info(line)
        if all(aunt[key] == value for key, value in properties.items()):
            print(f"The number of the Sue that got you the gift is: {sue_number}")
            return
    print("=" * 20, " Part 2 ", "=" * 20)

if __name__ == "__main__":
    main()