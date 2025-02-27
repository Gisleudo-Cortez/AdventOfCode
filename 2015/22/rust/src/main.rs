use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Clone, Eq, PartialEq)]
struct Spell {
    name: &'static str,
    cost: i32,
    damage: i32,
    heal: i32,
    armor: i32,
    mana_gain: i32,
    duration: i32,
}

const SPELLS: [Spell; 5] = [
    Spell { name: "Magic Missile", cost: 53, damage: 4, heal: 0, armor: 0, mana_gain: 0, duration: 0 },
    Spell { name: "Drain", cost: 73, damage: 2, heal: 2, armor: 0, mana_gain: 0, duration: 0 },
    Spell { name: "Shield", cost: 113, damage: 0, heal: 0, armor: 7, mana_gain: 0, duration: 6 },
    Spell { name: "Poison", cost: 173, damage: 3, heal: 0, armor: 0, mana_gain: 0, duration: 6 },
    Spell { name: "Recharge", cost: 229, damage: 0, heal: 0, armor: 0, mana_gain: 101, duration: 5 },
];

#[derive(Clone, Eq, PartialEq)]
struct GameState {
    player_hp: i32,
    player_mana: i32,
    boss_hp: i32,
    boss_damage: i32,
    active_effects: HashMap<&'static str, i32>,
    mana_spent: i32,
    hard_mode: bool,
    armor: i32,
}

impl Hash for GameState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.player_hp.hash(state);
        self.player_mana.hash(state);
        self.boss_hp.hash(state);
        self.active_effects.iter().for_each(|(k, v)| {
            k.hash(state);
            v.hash(state);
        });
    }
}

impl Ord for GameState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.mana_spent.cmp(&self.mana_spent)
    }
}

impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl GameState {
    fn apply_effects(&mut self) {
        self.armor = 0;
        let mut to_remove = Vec::new();
        
        for spell in SPELLS.iter() {
            if let Some(duration) = self.active_effects.get_mut(spell.name) {
                if spell.damage > 0 {
                    self.boss_hp -= spell.damage;
                }
                if spell.armor > 0 {
                    self.armor = spell.armor;
                }
                if spell.mana_gain > 0 {
                    self.player_mana += spell.mana_gain;
                }
                *duration -= 1;
                if *duration == 0 {
                    to_remove.push(spell.name);
                }
            }
        }
        for spell_name in to_remove {
            self.active_effects.remove(spell_name);
        }
    }

    fn get_valid_spells(&self) -> Vec<&Spell> {
        SPELLS.iter()
            .filter(|s| s.cost <= self.player_mana && !self.active_effects.contains_key(s.name))
            .collect()
    }
}

fn find_least_mana_to_win(player_hp: i32, player_mana: i32, boss_hp: i32, boss_damage: i32, hard_mode: bool) -> i32 {
    let initial_state = GameState {
        player_hp,
        player_mana,
        boss_hp,
        boss_damage,
        active_effects: HashMap::new(),
        mana_spent: 0,
        hard_mode,
        armor: 0,
    };

    let mut queue = BinaryHeap::new();
    let mut seen = HashMap::new();
    queue.push(initial_state);
    
    while let Some(mut state) = queue.pop() {
        if state.hard_mode {
            state.player_hp -= 1;
            if state.player_hp <= 0 {
                continue;
            }
        }

        state.apply_effects();
        if state.boss_hp <= 0 {
            return state.mana_spent;
        }

        for spell in state.get_valid_spells() {
            let mut new_state = state.clone();
            new_state.player_mana -= spell.cost;
            new_state.mana_spent += spell.cost;

            if spell.duration > 0 {
                new_state.active_effects.insert(spell.name, spell.duration);
            } else {
                new_state.boss_hp -= spell.damage;
                new_state.player_hp += spell.heal;
            }

            if new_state.boss_hp <= 0 {
                return new_state.mana_spent;
            }

            new_state.apply_effects();
            if new_state.boss_hp <= 0 {
                return new_state.mana_spent;
            }

            let damage_taken = (new_state.boss_damage - new_state.armor).max(1);
            new_state.player_hp -= damage_taken;

            if new_state.player_hp > 0 {
                if !seen.contains_key(&new_state) {
                    seen.insert(new_state.clone(), new_state.mana_spent);
                    queue.push(new_state);
                }
            }
        }
    }
    i32::MAX
}

fn main() {
    let sep = "=".repeat(20);
    
    println!("{} Part 1 {}", sep, sep);
    let mana_spent = find_least_mana_to_win(50, 500, 55, 8, false);
    println!("Total mana spent: {}", mana_spent);

    println!("{} Part 2 {}", sep, sep);
    let mana_spent = find_least_mana_to_win(50, 500, 55, 8, true);
    println!("Total mana spent: {}", mana_spent);
}