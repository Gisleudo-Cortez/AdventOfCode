from __future__ import annotations
from dataclasses import dataclass
from collections import defaultdict
import heapq
from typing import Dict, List, Tuple

@dataclass
class Spell:
    name: str
    cost: int
    damage: int = 0
    heal: int = 0
    armor: int = 0
    mana_gain: int = 0
    duration: int = 0

    def __lt__(self, other):
        return self.cost < other.cost

SPELLS = [
    Spell("Magic Missile", 53, damage=4),
    Spell("Drain", 73, damage=2, heal=2),
    Spell("Shield", 113, armor=7, duration=6),
    Spell("Poison", 173, damage=3, duration=6),
    Spell("Recharge", 229, mana_gain=101, duration=5),
]

class GameState:
    def __init__(self, player_hp: int, player_mana: int, boss_hp: int, boss_damage: int, 
                 active_effects: Dict[str, int], mana_spent: int = 0, hard_mode: bool = False):
        self.player_hp = player_hp
        self.player_mana = player_mana
        self.boss_hp = boss_hp
        self.boss_damage = boss_damage
        self.active_effects = active_effects.copy()
        self.mana_spent = mana_spent
        self.hard_mode = hard_mode
        self.armor = 0
    
    def __lt__(self, other):
        return self.mana_spent < other.mana_spent

    def apply_effects(self) -> None:
        self.armor = 0
        for spell in SPELLS:
            if spell.name in self.active_effects:
                if spell.damage:
                    self.boss_hp -= spell.damage
                if spell.armor:
                    self.armor = spell.armor
                if spell.mana_gain:
                    self.player_mana += spell.mana_gain
                self.active_effects[spell.name] -= 1
                if self.active_effects[spell.name] <= 0:
                    del self.active_effects[spell.name]

    def get_valid_spells(self) -> List[Spell]:
        return [spell for spell in SPELLS 
                if spell.cost <= self.player_mana 
                and spell.name not in self.active_effects]

    def clone(self) -> GameState:
        return GameState(
            self.player_hp,
            self.player_mana,
            self.boss_hp,
            self.boss_damage,
            self.active_effects,
            self.mana_spent,
            self.hard_mode
        )

def find_least_mana_to_win(player_hp: int, player_mana: int, boss_hp: int, boss_damage: int, hard_mode: bool = False) -> int:
    initial_state = GameState(player_hp, player_mana, boss_hp, boss_damage, {}, 0, hard_mode)
    queue = [(0, initial_state)]
    seen = set()

    while queue:
        _, state = heapq.heappop(queue)
        
        # Player's turn
        if state.hard_mode:
            state.player_hp -= 1
            if state.player_hp <= 0:
                continue

        # Apply effects at start of player's turn
        state.apply_effects()
        if state.boss_hp <= 0:
            return state.mana_spent

        # Try each possible spell
        for spell in state.get_valid_spells():
            new_state = state.clone()
            new_state.player_mana -= spell.cost
            new_state.mana_spent += spell.cost

            if spell.duration:
                new_state.active_effects[spell.name] = spell.duration
            else:
                new_state.boss_hp -= spell.damage
                new_state.player_hp += spell.heal

            if new_state.boss_hp <= 0:
                return new_state.mana_spent

            # Boss's turn
            new_state.apply_effects()
            if new_state.boss_hp <= 0:
                return new_state.mana_spent

            damage_taken = max(1, state.boss_damage - new_state.armor)
            new_state.player_hp -= damage_taken

            if new_state.player_hp > 0:
                state_tuple = (new_state.player_hp, new_state.player_mana, new_state.boss_hp,
                             tuple(sorted(new_state.active_effects.items())))
                if state_tuple not in seen:
                    seen.add(state_tuple)
                    heapq.heappush(queue, (new_state.mana_spent, new_state))

    return float('inf')

def main() -> None:
    print("=" * 20, "Part 1", "=" * 20)
    mana_spent = find_least_mana_to_win(50, 500, 55, 8)
    print(f"Total mana spent: {mana_spent}")
    
    print("=" * 20, "Part 2", "=" * 20)
    mana_spent = find_least_mana_to_win(50, 500, 55, 8, hard_mode=True)
    print(f"Total mana spent: {mana_spent}")

if __name__ == "__main__":
    main()