from pathlib import Path
from typing import List

def get_combinations(data: List[int], liters: int) -> int:
    data = sorted(data, reverse=True)
    valid_combinations = []
    
    def backtrack(start: int, combination: List[int], current_sum: int):
        if current_sum == liters:
            valid_combinations.append(combination)
        if current_sum > liters:
            return
        for i in range(start, len(data)):
            backtrack(i + 1, combination + [data[i]], current_sum + data[i])
    backtrack(0, [], 0)
    return len(valid_combinations)

def get_min_combinations(data: List[int], liters: int) -> int:
    data = sorted(data, reverse=True)
    valid_combinations = []
    def backtrack(start: int, combination: List[int], current_sum: int):
        if current_sum == liters:
            valid_combinations.append(combination)
        if current_sum > liters:
            return
        for i in range(start, len(data)):
            backtrack(i + 1, combination + [data[i]], current_sum + data[i])
    backtrack(0, [], 0)
    valid_combinations =  sorted(valid_combinations, key=len)
    min_len = len(valid_combinations[0])
    total_min_len = len([x for x in valid_combinations if len(x) == min_len])
    return total_min_len

def main() -> None:
    data = [int(x) for x in Path("../input.txt").read_text().strip().splitlines()]
    example = [int(x) for x in Path("../example.txt").read_text().strip().splitlines()]

    print("=" * 20, "Part 1", "=" * 20)
    print(f"Total combinations: {get_combinations(data, 150)}")
    print("=" * 20, "Part 2", "=" * 20)
    print(f"Total combinations: {get_min_combinations(data, 150)}")

if __name__ == "__main__":
    main()