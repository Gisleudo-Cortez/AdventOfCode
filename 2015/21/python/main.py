from pathlib import Path
from typing import Dict, Tuple, List
from itertools import combinations

class Entity:
    def __init__(self, hp: int, armor: int, dmg: int):
        self.hp = hp
        self.armor = armor
        self.dmg = dmg
    
    def is_alive(self) -> bool:
        return self.hp > 0
    
    def take_damage(self, incoming_damage: int) -> None:
        actual_damage = max(1, incoming_damage - self.armor)
        self.hp -= actual_damage
    
    def attack(self, target: 'Entity') -> None:
        target.take_damage(self.dmg)

def simulate_combat(player_stats: Dict[str, int]) -> bool:
    
    player = Entity(
        hp=100,  
        armor=player_stats['Armor'],
        dmg=player_stats['Damage']
    )
    
    boss = Entity(
        hp=109,  
        armor=2,
        dmg=8
    )
    
    while True:
        player.attack(boss)
        if not boss.is_alive():
            return True  # Player wins
            
        boss.attack(player)
        if not player.is_alive():
            return False  # Boss wins

def parse_items(path: str) -> Tuple[Dict[str, Dict[str, int]], Dict[str, Dict[str, int]], Dict[str, Dict[str, int]]]:
    data = Path("../items.txt").read_text().strip().split("\n\n")
    weapons = {}
    armors = {}
    rings = {}
    for i, cat in enumerate(data):
        if i == 0:
            data = cat.splitlines()
            for itm in data[1:]:
                info = itm.split()
                if info[0].strip() not in weapons: weapons[info[0].strip()] = {}
                weapons[info[0].strip()]["Cost"] = int(info[1].strip())
                weapons[info[0].strip()]["Damage"] = int(info[2].strip())
                weapons[info[0].strip()]["Armor"] = int(info[-1].strip())
        elif i == 1:
            data = cat.splitlines()
            for itm in data[1:]:
                info = itm.split()
                if info[0].strip() not in armors: armors[info[0].strip()] = {}
                armors[info[0].strip()]["Cost"] = int(info[1].strip())
                armors[info[0].strip()]["Damage"] = int(info[2].strip())
                armors[info[0].strip()]["Armor"] = int(info[-1].strip())
        elif i == 2:
            data = cat.splitlines()
            for itm in data[1:]:
                info = itm.split("   ")
                if info[0].strip() not in rings: rings[info[0].strip()] = {}
                rings[info[0].strip()]["Cost"] = int(info[1].strip())
                rings[info[0].strip()]["Damage"] = int(info[2].strip())
                rings[info[0].strip()]["Armor"] = int(info[-1].strip())
    return weapons, armors, rings

def gen_item_combinations(weapons: Dict[str, Dict[str, int]], 
                        armors: Dict[str, Dict[str, int]], 
                        rings: Dict[str, Dict[str, int]]) -> List[Tuple[str, ...]]:
    valid_combinations = []
    
    # Generate ring combinations (0, 1, or 2 rings)
    ring_combinations = [()]
    ring_combinations.extend(combinations(rings.keys(), 1))  # One ring
    ring_combinations.extend(combinations(rings.keys(), 2))  # Two rings
    
    # Generate armor combinations (0 or 1 armor)
    armor_combinations = [()]
    armor_combinations.extend((armor_name,) for armor_name in armors.keys())
    
    for weapon_name in weapons.keys():
        for armor_combo in armor_combinations:
            for ring_combo in ring_combinations:
                combination = (weapon_name,) + armor_combo + ring_combo
                valid_combinations.append(combination)
    
    return valid_combinations

def calculate_stats(combination: Tuple[str, ...], 
                   weapons: Dict[str, Dict[str, int]], 
                   armors: Dict[str, Dict[str, int]], 
                   rings: Dict[str, Dict[str, int]]) -> Dict[str, int]:
    total_stats = {'Cost': 0, 'Damage': 0, 'Armor': 0}
    
    for item in combination:
        if item in weapons:
            stats = weapons[item]
        elif item in armors:
            stats = armors[item]
        elif item in rings:
            stats = rings[item]
        else:
            continue
            
        for stat in total_stats:
            total_stats[stat] += stats[stat]
            
    return total_stats

def find_optimal_equipment(weapons: Dict[str, Dict[str, int]],
                         armors: Dict[str, Dict[str, int]],
                         rings: Dict[str, Dict[str, int]],
                         optimize_for_cost: bool = True) -> Tuple[Tuple[str, ...], Dict[str, int]]:
    combinations = gen_item_combinations(weapons, armors, rings)
    optimal_combo = None
    optimal_stats = None
    optimal_cost = float('inf') if optimize_for_cost else float('-inf')
    
    for combo in combinations:
        stats = calculate_stats(combo, weapons, armors, rings)
        player_wins = simulate_combat(stats)
        
        if optimize_for_cost:
            if player_wins and stats['Cost'] < optimal_cost:
                optimal_combo = combo
                optimal_stats = stats
                optimal_cost = stats['Cost']
        else:
            if not player_wins and stats['Cost'] > optimal_cost:
                optimal_combo = combo
                optimal_stats = stats
                optimal_cost = stats['Cost']
    
    return optimal_combo, optimal_stats

def main() -> None:
    item_path = "../items.txt"
    weapons, armors, rings = parse_items(item_path)
    
    winning_combo, winning_stats = find_optimal_equipment(weapons, armors, rings, optimize_for_cost=True)
    print("=" * 20, "Part 1", "=" * 20)
    print("Cheapest winning combination:")
    print(f"Items: {winning_combo}")
    print(f"Stats: {winning_stats}")
    print(f"Total cost: {winning_stats['Cost']} gold\n")

    losing_combo, losing_stats = find_optimal_equipment(weapons, armors, rings, optimize_for_cost=False)
    print("=" * 20, "Part 2", "=" * 20)
    print("Most expensive losing combination:")
    print(f"Items: {losing_combo}")
    print(f"Stats: {losing_stats}")
    print(f"Total cost: {losing_stats['Cost']} gold")

if __name__ == "__main__":
    main()