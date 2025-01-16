# from pathlib import Path
# from typing import List, Tuple
# from itertools import combinations

# def calc_qe(group: List[int]) -> int:
#     qe = 1
#     for weight in group:
#         qe *= weight
#     return qe

# def match_groups(presents: List[int], compartments: int) -> Tuple[int, List[int], List[int], List[int]]:
#     presents = sorted(presents, reverse=True)
#     total_weigth = sum(presents)
#     target_w = total_weigth // compartments
#     g1_possible = []
#     for size in range(1, len(presents) + 1):
#         for subset in combinations(presents, size):
#             if sum(subset) == target_w:
#                 g1_possible.append(subset)
#     g1_possible.sort(key=lambda gp: (len(gp), calc_qe(gp)))

#     for g1 in g1_possible:
#         remaining = [pkg for pkg in presents if pkg not in g1]

#         for size in range(1, len(remaining) + 1):
#             for g2 in combinations(remaining, size):
#                 if sum(g2) == target_w:
#                     remaining_after_g2 = [pkg for pkg in remaining if pkg not in g2]

#                     for size3 in range(1, len(remaining_after_g2) + 1):
#                         for g3 in combinations(remaining_after_g2, size3):
#                             if sum(g3) == target_w:
#                                 if compartments == 3:
#                                     return calc_qe(g1), g1, g2, g3

#                                 g4 = [
#                                     pkg for pkg in remaining_after_g2 if pkg not in g3
#                                 ]
#                                 if sum(g4) == target_w:
#                                     return calc_qe(g1), g1, g2, g3, g4

#     return None

# def main() -> None:
#     data = [int(x) for x in Path("../input.txt").read_text().splitlines()]
#     example = [int(x) for x in Path("../example.txt").read_text().splitlines()]
#     print("=" * 20, "Part 1", "=" * 20)
#     qe, g1, g2, g3 = match_groups(data, 3)
#     print(f"QE: {qe}, Group 1: {g1}, Group 2: {g2}, Group 3: {g3}")
#     print("=" * 20, "Part 2", "=" * 20)
#     qe, g1, g2, g3, g4 = match_groups(data, 4)
#     print(f"QE: {qe}, Group 1: {g1}, Group 2: {g2}, Group 3: {g3}, Group 4: {g4}")

# if __name__ == "__main__":
#     main()

from pathlib import Path
from typing import List, Tuple, Optional, Set
from itertools import combinations
from functools import lru_cache, reduce

def calc_qe(group: List[int]) -> int:
    # Using prod from math would be even faster, but keeping multiplication
    # to minimize imports
    return reduce(lambda x, y: x * y, group)

def find_valid_groups(numbers: Tuple[int, ...], target: int, max_size: int) -> List[Tuple[int, ...]]:
    valid_groups = []
    
    # Start with smaller groups first since we want minimal first group
    for size in range(1, max_size + 1):
        for group in combinations(numbers, size):
            if sum(group) == target:
                valid_groups.append(group)
        # Early exit if we found valid groups of this size
        if valid_groups:
            return [g for g in valid_groups if len(g) == len(valid_groups[0])]
    return valid_groups

def can_split_remaining(numbers: Tuple[int, ...], target: int, groups_left: int) -> bool:
    if groups_left == 0:
        return not numbers
    if not numbers or sum(numbers) != target * groups_left:
        return False
    if groups_left == 1:
        return sum(numbers) == target
    
    for size in range(1, len(numbers) + 1):
        for group in combinations(numbers, size):
            if sum(group) == target:
                remaining = tuple(x for x in numbers if x not in group)
                if can_split_remaining(remaining, target, groups_left - 1):
                    return True
    return False

def match_groups(presents: List[int], compartments: int) -> Optional[Tuple]:
    presents = tuple(sorted(presents, reverse=True))  # Convert to tuple for immutability
    total_weight = sum(presents)
    target_w = total_weight // compartments
    
    # Validate input
    if total_weight % compartments != 0:
        return None
    
    # Find all possible first groups that are minimal in size
    max_group_size = len(presents) // compartments
    first_groups = find_valid_groups(presents, target_w, max_group_size)
    
    # Sort by quantum entanglement
    first_groups.sort(key=calc_qe)
    
    for g1 in first_groups:
        remaining = tuple(x for x in presents if x not in g1)
        if can_split_remaining(remaining, target_w, compartments - 1):
            # We only need to find the first valid solution since it will have
            # the lowest QE due to our sorting
            return calc_qe(g1), g1

def main() -> None:
    data = tuple(int(x) for x in Path("../input.txt").read_text().splitlines())
    
    print("=" * 20, "Part 1", "=" * 20)
    result = match_groups(data, 3)
    if result:
        qe, group = result
        print(f"QE: {qe}, First Group: {group}")
    
    print("=" * 20, "Part 2", "=" * 20)
    result = match_groups(data, 4)
    if result:
        qe, group = result
        print(f"QE: {qe}, First Group: {group}")

if __name__ == "__main__":
    main()