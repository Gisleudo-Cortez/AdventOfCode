# from pathlib import Path
# from typing import Tuple, List, Dict, Optional

# def parse(data: str) -> Tuple[str, int, int, int, int, int]:
#     """
#     input: Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
#     output: ('Butterscotch', -1, -2, 6, 3, 8)
#     """
#     data = [x.strip(":,") for x in data.split()]
#     return (data[0], int(data[2]), int(data[4]), int(data[6]), int(data[8]), int(data[10]))

# def calculate_max_score(ingredients: List[Tuple[str, int, int, int, int, int]], 
#                        n: int = 100, 
#                        target_calories: Optional[int] = None) -> Tuple[int, Dict[str, int]]:
#     max_score = 0
#     num_ingredients = len(ingredients)
#     best_amounts: Dict[str, int] = {}

#     def calculate_score(amounts: List[int]) -> int:
#         capacity = sum(amounts[i] * ingredients[i][1] for i in range(num_ingredients))
#         durability = sum(amounts[i] * ingredients[i][2] for i in range(num_ingredients))
#         flavor = sum(amounts[i] * ingredients[i][3] for i in range(num_ingredients))
#         texture = sum(amounts[i] * ingredients[i][4] for i in range(num_ingredients))
#         calories = sum(amounts[i] * ingredients[i][5] for i in range(num_ingredients))

#         if target_calories is not None and calories != target_calories:
#             return 0

#         properties = [capacity, durability, flavor, texture]
#         return max(0, properties[0]) * max(0, properties[1]) * max(0, properties[2]) * max(0, properties[3])

#     def find_best_combination(index: int, remaining_teaspoons: int, current_amounts: List[int]):
#         nonlocal max_score, best_amounts
        
#         if index == num_ingredients - 1:
#             current_amounts[index] = remaining_teaspoons
#             score = calculate_score(current_amounts)
#             if score > max_score:
#                 max_score = score
#                 best_amounts = {ingredients[i][0]: current_amounts[i] for i in range(num_ingredients)}
#             return

#         for teaspoons in range(remaining_teaspoons + 1):
#             current_amounts[index] = teaspoons
#             find_best_combination(index + 1, remaining_teaspoons - teaspoons, current_amounts)

#     find_best_combination(0, n, [0] * num_ingredients)
#     return max_score, best_amounts

# def main() -> None:
#     data = Path("../input.txt").read_text().strip().splitlines()
#     datas = [parse(line) for line in data]
#     example = Path("../example.txt").read_text().strip().splitlines()
#     examples = [parse(line) for line in example]

#     print("=" * 20, " Part 1 ", "=" * 20)
#     max_score_part1, best_ingredients_part1 = calculate_max_score(datas)
#     print(f"The max score is {max_score_part1}")
#     print(f"Best ingredients: {best_ingredients_part1}")

#     print("=" * 20, " Part 2 ", "=" * 20)
#     max_score_part2, best_ingredients_part2 = calculate_max_score(datas, target_calories=500)
#     print(f"The max score (500 calories) is {max_score_part2}")
#     print(f"Best ingredients: {best_ingredients_part2}")

# if __name__ == "__main__":
#     main()

from pathlib import Path
from typing import Tuple, List, Dict, Optional

def parse(data: str) -> Tuple[str, int, int, int, int, int]:
    """
    input: Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
    output: ('Butterscotch', -1, -2, 6, 3, 8)
    """
    data = [x.strip(":,") for x in data.split()]
    return (data[0], int(data[2]), int(data[4]), int(data[6]), int(data[8]), int(data[10]))

def calculate_max_score(ingredients: List[Tuple[str, int, int, int, int, int]], 
                       n: int = 100, 
                       target_calories: Optional[int] = None) -> Tuple[int, Dict[str, int]]:
    max_score = 0
    num_ingredients = len(ingredients)
    best_amounts: Dict[str, int] = {}

    def calculate_score(amounts: List[int]) -> int:
        capacity = sum(amounts[i] * ingredients[i][1] for i in range(num_ingredients))
        durability = sum(amounts[i] * ingredients[i][2] for i in range(num_ingredients))
        flavor = sum(amounts[i] * ingredients[i][3] for i in range(num_ingredients))
        texture = sum(amounts[i] * ingredients[i][4] for i in range(num_ingredients))
        calories = sum(amounts[i] * ingredients[i][5] for i in range(num_ingredients))

        if target_calories is not None and calories != target_calories:
            return 0

        properties = [capacity, durability, flavor, texture]
        return max(0, properties[0]) * max(0, properties[1]) * max(0, properties[2]) * max(0, properties[3])

    def find_best_combination(index: int, remaining_teaspoons: int, current_amounts: List[int]):
        nonlocal max_score, best_amounts
        
        if index == num_ingredients - 1:
            current_amounts[index] = remaining_teaspoons
            score = calculate_score(current_amounts)
            if score > max_score:
                max_score = score
                best_amounts = {ingredients[i][0]: current_amounts[i] for i in range(num_ingredients)}
            return

        for teaspoons in range(remaining_teaspoons + 1):
            current_amounts[index] = teaspoons
            find_best_combination(index + 1, remaining_teaspoons - teaspoons, current_amounts)

    find_best_combination(0, n, [0] * num_ingredients)
    return max_score, best_amounts

def main() -> None:
    data = Path("../input.txt").read_text().strip().splitlines()
    datas = [parse(line) for line in data]
    example = Path("../example.txt").read_text().strip().splitlines()
    examples = [parse(line) for line in example]

    print("=" * 20, " Part 1 ", "=" * 20)
    max_score_part1, best_ingredients_part1 = calculate_max_score(datas)
    print(f"The max score is {max_score_part1}")
    print(f"Best ingredients: {best_ingredients_part1}")

    print("=" * 20, " Part 2 ", "=" * 20)
    max_score_part2, best_ingredients_part2 = calculate_max_score(datas, target_calories=500)
    print(f"The max score (500 calories) is {max_score_part2}")
    print(f"Best ingredients: {best_ingredients_part2}")

if __name__ == "__main__":
    main()