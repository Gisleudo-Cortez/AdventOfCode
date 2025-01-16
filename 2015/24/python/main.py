from pathlib import Path
from typing import List, Tuple
from itertools import combinations

def calc_qe(group: List[int]) -> int:
    qe = 1
    for weight in group:
        qe *= weight
    return qe

def match_groups(presents: List[int], compartments: int) -> Tuple[int, List[int], List[int], List[int]]:
    presents = sorted(presents, reverse=True)
    total_weigth = sum(presents)
    target_w = total_weigth // compartments
    g1_possible = []
    for size in range(1, len(presents) + 1):
        for subset in combinations(presents, size):
            if sum(subset) == target_w:
                g1_possible.append(subset)
    g1_possible.sort(key=lambda gp: (len(gp), calc_qe(gp)))

    for g1 in g1_possible:
        remaining = [pkg for pkg in presents if pkg not in g1]

        for size in range(1, len(remaining) + 1):
            for g2 in combinations(remaining, size):
                if sum(g2) == target_w:
                    remaining_after_g2 = [pkg for pkg in remaining if pkg not in g2]

                    for size3 in range(1, len(remaining_after_g2) + 1):
                        for g3 in combinations(remaining_after_g2, size3):
                            if sum(g3) == target_w:
                                if compartments == 3:
                                    return calc_qe(g1), g1, g2, g3

                                g4 = [
                                    pkg for pkg in remaining_after_g2 if pkg not in g3
                                ]
                                if sum(g4) == target_w:
                                    return calc_qe(g1), g1, g2, g3, g4

    return None

def main() -> None:
    data = [int(x) for x in Path("../input.txt").read_text().splitlines()]
    example = [int(x) for x in Path("../example.txt").read_text().splitlines()]
    print("=" * 20, "Part 1", "=" * 20)
    qe, g1, g2, g3 = match_groups(data, 3)
    print(f"QE: {qe}, Group 1: {g1}, Group 2: {g2}, Group 3: {g3}")
    print("=" * 20, "Part 2", "=" * 20)
    qe, g1, g2, g3, g4 = match_groups(data, 4)
    print(f"QE: {qe}, Group 1: {g1}, Group 2: {g2}, Group 3: {g3}, Group 4: {g4}")

if __name__ == "__main__":
    main()