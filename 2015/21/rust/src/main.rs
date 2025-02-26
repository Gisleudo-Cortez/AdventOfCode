use std::{collections::HashMap, fs};
use itertools::Itertools;

#[derive(Copy,Clone)]
struct Entity {
    hp: i32,
    armor: i32,
    dmg: i32,
}

impl Entity {
    pub fn new(hp: i32, armor: i32, dmg: i32) -> Entity {
        Entity {hp, armor, dmg}
    }
    pub fn is_alive(self) -> bool {
        self.hp > 0 
    }
    pub fn take_damage(&mut self, dmg: i32) {
        let damage = dmg - self.armor;
        let actual_damage = damage.max(1);
        self.hp -= actual_damage;
    }
    pub fn attack(self, target: &mut Entity) {
        target.take_damage(self.dmg);
    }
}

fn simulate_combat(player_stats: HashMap<String, i32>) -> bool {
    let mut player = Entity::new(
        100,
        *player_stats.get("Armor").unwrap_or(&0), 
        *player_stats.get("Damage").unwrap_or(&0));
    let mut boss = Entity::new(109, 2, 8);
    
    loop {
        player.attack(&mut boss);
        if !boss.is_alive(){
            return true;
        }

        boss.attack(&mut player);
        if !player.is_alive(){
            return false
        }
    }
}

fn parse_items(path: &str) ->
(HashMap<String, HashMap<String, i32>>, 
HashMap<String, HashMap<String, i32>>, 
HashMap<String, HashMap<String, i32>>) {
    let binding = fs::read_to_string(path)
    .expect("Error parsing input");
    let data: Vec<&str> = binding
    .trim()
    .split("\n\n")
    .collect();

    let mut weapons: HashMap<String, HashMap<String, i32>> = HashMap::new();
    let mut armors: HashMap<String, HashMap<String, i32>> = HashMap::new();
    let mut rings: HashMap<String, HashMap<String, i32>> = HashMap::new();

    for (i,cat) in data.iter().enumerate(){
        let lines: Vec<&str> = cat.lines().collect();

        for item in &lines[1..]{
            let info: Vec<&str> = if i == 2 {
                item.split("   ").collect()
            } else {
                item.split_whitespace().collect()
            };

            if info.len() < 4 {continue;}

            let name = info[0].trim().to_string();
            let cost = info[1].trim().parse::<i32>().unwrap_or(0);
            let damage = info[2].trim().parse::<i32>().unwrap_or(0);
            let armor = info[3].trim().parse::<i32>().unwrap_or(0);

            let entry = HashMap::from([
                ("Cost".to_string(), cost),
                ("Damage".to_string(), damage),
                ("Armor".to_string(), armor),
            ]);

            match i {
                0 => {weapons.insert(name, entry);}
                1 => {armors.insert(name, entry);}
                2 => {rings.insert(name, entry);}
                _ => {}
            }
        }
    }
    (weapons, armors, rings)
}

fn gen_item_combinations(
    weapons: &HashMap<String, HashMap<String, i32>>,
    armors: &HashMap<String, HashMap<String, i32>>,
    rings: &HashMap<String, HashMap<String, i32>>,
) -> Vec<Vec<String>> {
    
    let mut valid_combinations = Vec::new();

    // Generate ring combinations (0, 1, or 2 rings)
    let ring_combinations: Vec<Vec<String>> = vec![vec![]]
        .into_iter()
        .chain(rings.keys().map(|r| vec![r.clone()]))
        .chain(
            rings.keys()
                .combinations(2)
                .map(|r| r.into_iter().cloned().collect())
        )
        .collect();

    // Generate armor combinations (0 or 1 armor)
    let armor_combinations: Vec<Vec<String>> = vec![vec![]]
        .into_iter()
        .chain(armors.keys().map(|a| vec![a.clone()]))
        .collect();

    // Generate all valid combinations
    for weapon in weapons.keys() {
        for armor in &armor_combinations {
            for ring in &ring_combinations {
                let mut comb = vec![weapon.clone()];
                comb.extend_from_slice(armor);
                comb.extend_from_slice(ring);
                valid_combinations.push(comb);
            }
        }
    }

    valid_combinations
}

fn calculate_stats(
    combination: &[String],
    weapons: &HashMap<String, HashMap<String, i32>>, 
    armors: &HashMap<String, HashMap<String, i32>>, 
    rings: &HashMap<String, HashMap<String, i32>>,
) -> HashMap<String, i32> {
    let mut total = HashMap::from([
        ("Cost".to_string(), 0),
        ("Damage".to_string(), 0),
        ("Armor".to_string(), 0),
    ]);
    
    for item in combination {
        let stats = weapons.get(item).or_else(|| armors.get(item)).or_else(|| rings.get(item));
        
        if let Some(stats) = stats {
            for (stat, value) in stats {
                if let Some(entry) = total.get_mut(stat) {
                    *entry += value;
                }
            }
        }
    }
    
    total
}


fn find_optimal_equipment(
weapons: &HashMap<String, HashMap<String, i32>>,
armors: &HashMap<String, HashMap<String, i32>>,
rings: &HashMap<String, HashMap<String, i32>>,
optimize_for_cost: bool,
) -> (Vec<String>, HashMap<String, i32>){
    let mut optimal_comb = Vec::new();
    let mut optimal_stats = HashMap::new();
    let mut optimal_cost = if optimize_for_cost {i32::MAX} else {i32::MIN};

    for combo in gen_item_combinations(weapons, armors, rings){
        let stats = calculate_stats(&combo, weapons, armors, rings);
        let player_wins = simulate_combat(stats.clone());

        if (optimize_for_cost && player_wins && stats["Cost"] < optimal_cost)
        || (!optimize_for_cost && !player_wins && stats["Cost"] > optimal_cost) 
        {
            optimal_cost = stats["Cost"];
            optimal_comb = combo.clone();
            optimal_stats = stats;
        }
    }
    (optimal_comb, optimal_stats)
}

fn main() {
    let sep = "=".repeat(20);
    let (weapons, armors, rings) = parse_items("../items.txt");
    
    let (winning_combo, winning_stats) = find_optimal_equipment(&weapons, &armors, &rings, true);
    
    println!("{} Part 1 {}", sep, sep);
    println!("Cheapest winning combination: {:?}", winning_combo);
    println!("Stats: {:?}", winning_stats);
    println!("Total cost: {} gold\n", winning_stats["Cost"]);
    
    println!("{} Part 2 {}", sep, sep);
    let (losing_combo, losing_stats) = find_optimal_equipment(&weapons, &armors, &rings, false);
    println!("Most expensive losing combination: {:?}", losing_combo);
    println!("Stats: {:?}", losing_stats);
    println!("Total cost: {} gold", losing_stats["Cost"]);
}
