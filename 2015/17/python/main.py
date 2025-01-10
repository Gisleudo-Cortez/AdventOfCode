# from pathlib import Path
# from typing import List

# def get_combinations(data: List[int], liters: int) -> int:
#     data = sorted(data, reverse=True)
#     valid_combinations = []
    
#     def backtrack(start: int, combination: List[int], current_sum: int):
#         if current_sum == liters:
#             valid_combinations.append(combination)
#         if current_sum > liters:
#             return
#         for i in range(start, len(data)):
#             backtrack(i + 1, combination + [data[i]], current_sum + data[i])
#     backtrack(0, [], 0)
#     return len(valid_combinations)

# def get_min_combinations(data: List[int], liters: int) -> int:
#     data = sorted(data, reverse=True)
#     valid_combinations = []
#     def backtrack(start: int, combination: List[int], current_sum: int):
#         if current_sum == liters:
#             valid_combinations.append(combination)
#         if current_sum > liters:
#             return
#         for i in range(start, len(data)):
#             backtrack(i + 1, combination + [data[i]], current_sum + data[i])
#     backtrack(0, [], 0)
#     valid_combinations =  sorted(valid_combinations, key=len)
#     min_len = len(valid_combinations[0])
#     total_min_len = len([x for x in valid_combinations if len(x) == min_len])
#     return total_min_len

# def main() -> None:
#     data = [int(x) for x in Path("../input.txt").read_text().strip().splitlines()]
#     example = [int(x) for x in Path("../example.txt").read_text().strip().splitlines()]

#     print("=" * 20, "Part 1", "=" * 20)
#     print(f"Total combinations: {get_combinations(data, 150)}")
#     print("=" * 20, "Part 2", "=" * 20)
#     print(f"Total combinations: {get_min_combinations(data, 150)}")

# if __name__ == "__main__":
#     main()

from pathlib import Path
from typing import List, Set, Tuple
from collections import defaultdict

def find_combinations(containers: List[int], target: int) -> Tuple[int, int]:
    """
    Find all valid combinations of containers that sum to target volume.
    Returns tuple of (total combinations, minimum container combinations).
    
    Uses dynamic programming approach to track combinations by length.
    """
    containers = sorted(containers, reverse=True)  # Sort once for optimization
    combinations_by_length = defaultdict(int)
    
    def backtrack(index: int, remaining: int, combo_length: int) -> None:
        # Base cases
        if remaining == 0:
            combinations_by_length[combo_length] += 1
            return
        if remaining < 0 or index >= len(containers):
            return
            
        # Try including current container
        backtrack(index + 1, remaining - containers[index], combo_length + 1)
        # Try excluding current container
        backtrack(index + 1, remaining, combo_length)
    
    backtrack(0, target, 0)
    
    # If no valid combinations found, return (0, 0)
    if not combinations_by_length:
        return 0, 0
        
    # Calculate results
    total_combinations = sum(combinations_by_length.values())
    min_length = min(combinations_by_length.keys())
    min_length_combinations = combinations_by_length[min_length]
    
    return total_combinations, min_length_combinations

def load_data(filepath: str) -> List[int]:
    """Load container sizes from file."""
    try:
        return [int(x) for x in Path(filepath).read_text().strip().splitlines()]
    except (FileNotFoundError, ValueError) as e:
        print(f"Error loading data from {filepath}: {e}")
        return []

def main() -> None:
    target_volume = 150
    
    # Load data
    data = load_data("../input.txt")
    if not data:
        return
        
    # Calculate results
    total_combinations, min_combinations = find_combinations(data, target_volume)
    
    # Print results
    print("=" * 20, "Results", "=" * 20)
    print(f"Part 1 - Total combinations: {total_combinations}")
    print(f"Part 2 - Minimum container combinations: {min_combinations}")

if __name__ == "__main__":
    main()