# from pathlib import Path
# from typing import Dict, List
# from itertools import permutations

# def parse(data:List[str]) -> Dict[str, Dict[str,int]]:
#     out_dict = {}
#     for r in data:
#         vals = r.split()
#         p1 = vals[0]
#         p2 = vals[-1].strip(".")
#         hap = int(vals[3])

#         if vals[2] == "lose":
#             hap = -hap

#         if p1 not in out_dict:
#             out_dict[p1] = {}
            
#         out_dict[p1].update({p2:hap})
#     return out_dict

# def parse_2(data:List[str]) -> Dict[str, Dict[str,int]]:
#     out_dict = {}
#     for r in data:
#         vals = r.split()
#         p1 = vals[0]
#         p2 = vals[-1].strip(".")
#         hap = int(vals[3])

#         if vals[2] == "lose":
#             hap = -hap

#         if p1 not in out_dict:
#             out_dict[p1] = {}

#         out_dict[p1].update({p2:hap})
#         all_guests = set(out_dict.keys()).union(
#         {guest for subdict in out_dict.values() for guest in subdict.keys()})
    
#         for guest in all_guests:
#             if guest not in out_dict:
#                 out_dict[guest] = {}
#             out_dict[guest]["you"] = 0
        
#     out_dict["you"] = {guest: 0 for guest in all_guests}
#     return out_dict

# def validate_seating(arrangement, preferences):
#     total_happiness = 0
#     n = len(arrangement)

#     for i in range(n):
#         current_person = arrangement[i]
#         left_neighbor = arrangement[(i - 1) % n]
#         right_neighbor = arrangement[(i + 1) % n]

#         if left_neighbor in preferences[current_person]:
#             total_happiness += int(preferences[current_person][left_neighbor])
#         else:
#             return None

#         if right_neighbor in preferences[current_person]:
#             total_happiness += int(preferences[current_person][right_neighbor])
#         else:
#             return None

#     return total_happiness

# def find_best_seating(preferences):
#     people = list(preferences.keys())
#     max_happiness = None

#     for arrangement in permutations(people):
#         happiness = validate_seating(arrangement, preferences)
#         if happiness is not None:
#             if max_happiness is None or happiness > max_happiness:
#                 max_happiness = happiness

#     return max_happiness




# def main() -> None:
#     data = Path("../input.txt").read_text().strip().splitlines()
#     example = Path("../example.txt").read_text().strip().splitlines()
#     values = parse(data)
#     print("=" * 20, " Part 1 ", "=" * 20)
#     print(f"Max happiness score: {find_best_seating(values)}")
#     print("=" * 20, " Part 2 ", "=" * 20)
#     values = parse_2(data)
#     print(f"Max happiness score: {find_best_seating(values)}")

    
# if __name__ == "__main__":
#     main()

from pathlib import Path
from typing import Dict, List
from itertools import permutations

def parse(data: List[str]) -> Dict[str, Dict[str, int]]:
    """Parses the input data into a dictionary of happiness preferences."""
    out_dict = {}
    for r in data:
        parts = r.split()
        person1 = parts[0]
        person2 = parts[-1].rstrip('.')
        happiness = int(parts[3])
        if parts[2] == "lose":
            happiness = -happiness
        out_dict.setdefault(person1, {})[person2] = happiness
    return out_dict

def parse_2(data: List[str]) -> Dict[str, Dict[str, int]]:
    """Parses the input data and adds 'you' with neutral happiness."""
    preferences = parse(data)
    all_guests = set(preferences.keys())
    for prefs in preferences.values():
        all_guests.update(prefs.keys())

    preferences["you"] = {guest: 0 for guest in all_guests}
    for guest in all_guests:
        preferences.setdefault(guest, {})["you"] = 0
    return preferences

def calculate_happiness(arrangement: tuple, preferences: Dict[str, Dict[str, int]]) -> int:
    """Calculates the total happiness for a given seating arrangement."""
    total_happiness = 0
    num_people = len(arrangement)
    for i in range(num_people):
        person = arrangement[i]
        left_neighbor = arrangement[(i - 1) % num_people]
        right_neighbor = arrangement[(i + 1) % num_people]
        total_happiness += preferences[person][left_neighbor]
        total_happiness += preferences[person][right_neighbor]
    return total_happiness

def find_best_seating(preferences: Dict[str, Dict[str, int]]) -> int:
    """Finds the maximum happiness score for all possible seating arrangements."""
    people = tuple(preferences.keys())
    max_happiness = float('-inf')
    for arrangement in permutations(people):
        max_happiness = max(max_happiness, calculate_happiness(arrangement, preferences))
    return max_happiness

def main() -> None:
    data = Path("../input.txt").read_text().strip().splitlines()
    example = Path("../example.txt").read_text().strip().splitlines()

    print("=" * 20, " Part 1 ", "=" * 20)
    values = parse(data)
    print(f"Max happiness score: {find_best_seating(values)}")

    print("=" * 20, " Part 2 ", "=" * 20)
    values_part2 = parse_2(data)
    print(f"Max happiness score: {find_best_seating(values_part2)}")

if __name__ == "__main__":
    main()