from pathlib import Path
import re
import json

def total_sum_1(data:str) -> int:
    pattern = r"-?\d+"
    matches = re.findall(pattern,data)
    total = 0
    for n in matches:
        total += int(n)
    return total

def sum_non_red(data):
    if isinstance(data, int):
        return data
    
    elif isinstance(data, list):
        return sum(sum_non_red(item) for item in data)
    
    elif isinstance(data, dict):
        if "red" in data.values():
            return 0
        return sum(sum_non_red(value) for value in data.values())
    
    # String or other non-numeric values
    return 0

def total_sum_2(data: str) -> int:
    json_data = json.loads(data)
    return sum_non_red(json_data)

def main() -> None:
    data = Path("../input.txt").read_text().strip()
    example = Path("../example.txt").read_text().strip()
    print("=" * 20, " Part 1 ", "=" * 20)
    print(f"sum: {total_sum_1(data)}")
    print("=" * 20, " Part 2 ", "=" * 20)
    print(f"sum: {total_sum_2(data)}")

if __name__ == "__main__":
    main()