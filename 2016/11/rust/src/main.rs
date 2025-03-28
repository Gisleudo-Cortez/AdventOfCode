use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum ItemType {
    Generator,
    Microchip,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Item {
    element_index: usize,
    kind: ItemType,
}

#[derive(Clone, Debug, Eq)]
struct State {
    elevator_floor: usize,
    item_floors: Vec<(usize, usize)>,
    steps: u32,
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.elevator_floor.hash(state);
        self.item_floors.hash(state);
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.elevator_floor == other.elevator_floor && self.item_floors == other.item_floors
    }
}

impl State {
    fn is_valid(&self) -> bool {
        for floor in 0..4 {
            let mut generators_on_floor = Vec::new();
            let mut microchips_on_floor = Vec::new();

            for (element_idx, &(gen_floor, chip_floor)) in self.item_floors.iter().enumerate() {
                if gen_floor == floor {
                    generators_on_floor.push(element_idx);
                }
                if chip_floor == floor {
                    microchips_on_floor.push(element_idx);
                }
            }

            if !generators_on_floor.is_empty() {
                for &chip_idx in &microchips_on_floor {
                    if !generators_on_floor.contains(&chip_idx) {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn is_goal(&self) -> bool {
        self.elevator_floor == 3 && self.item_floors.iter().all(|&(g, c)| g == 3 && c == 3)
    }

    // Generates all valid successor states reachable in one move
    fn get_next_states(&self) -> Vec<State> {
        let mut next_states = Vec::new();
        let current_floor = self.elevator_floor;

        // Identify all items currently on the elevator's floor
        let items_on_current_floor: Vec<Item> = self
            .item_floors
            .iter()
            .enumerate()
            .flat_map(|(idx, &(gen_f, chip_f))| {
                let mut items = Vec::new();
                if gen_f == current_floor {
                    items.push(Item {
                        element_index: idx,
                        kind: ItemType::Generator,
                    });
                }
                if chip_f == current_floor {
                    items.push(Item {
                        element_index: idx,
                        kind: ItemType::Microchip,
                    });
                }
                items
            })
            .collect();

        // Generate combinations of 1 or 2 items to move
        let item_combinations = items_on_current_floor
            .clone()
            .into_iter()
            .combinations(1)
            .chain(items_on_current_floor.clone().into_iter().combinations(2));

        for next_floor_delta in [-1, 1] {
            let next_floor = current_floor as i32 + next_floor_delta;

            if !(0..4).contains(&next_floor) {
                continue;
            }
            let next_floor = next_floor as usize;

            for items_to_move in item_combinations.clone() {
                // Clone needed because chain isn't cloneable easily
                let mut next_item_floors = self.item_floors.clone();

                for item in &items_to_move {
                    match item.kind {
                        ItemType::Generator => next_item_floors[item.element_index].0 = next_floor,
                        ItemType::Microchip => next_item_floors[item.element_index].1 = next_floor,
                    }
                }

                next_item_floors.sort_unstable();
                let potential_next_state = State {
                    elevator_floor: next_floor,
                    item_floors: next_item_floors,
                    steps: self.steps + 1,
                };

                if potential_next_state.is_valid() {
                    next_states.push(potential_next_state);
                }
            }
        }
        next_states
    }
}

fn parse_input(input: &str) -> State {
    let mut element_map: HashMap<String, usize> = HashMap::new();
    let mut element_count = 0;
    let mut initial_locations: Vec<(Option<usize>, Option<usize>)> = Vec::new();

    let gen_re = Regex::new(r"a (\w+) generator").unwrap();
    let chip_re = Regex::new(r"a (\w+)-compatible microchip").unwrap();

    for (floor_idx, line) in input.lines().enumerate() {
        // Find generators
        for cap in gen_re.captures_iter(line) {
            let name = cap[1].to_string();
            let idx = *element_map.entry(name).or_insert_with(|| {
                let current_idx = element_count;
                element_count += 1;
                if current_idx >= initial_locations.len() {
                    initial_locations.resize(current_idx + 1, (None, None));
                }
                current_idx
            });
            initial_locations[idx].0 = Some(floor_idx);
        }
        // Find microchips
        for cap in chip_re.captures_iter(line) {
            let name = cap[1].to_string();
            let idx = *element_map.entry(name).or_insert_with(|| {
                let current_idx = element_count;
                element_count += 1;
                if current_idx >= initial_locations.len() {
                    initial_locations.resize(current_idx + 1, (None, None));
                }
                current_idx
            });
            initial_locations[idx].1 = Some(floor_idx);
        }
    }

    let mut item_floors: Vec<(usize, usize)> = initial_locations
        .iter()
        .map(|(gen_opt, chip_opt)| {
            (
                gen_opt.expect("Generator location missing"),
                chip_opt.expect("Microchip location missing"),
            )
        })
        .collect();

    item_floors.sort_unstable();

    State {
        elevator_floor: 0,
        item_floors,
        steps: 0,
    }
}

fn solve(initial_state: State) -> Option<u32> {
    let mut queue: VecDeque<State> = VecDeque::new();
    let mut visited: HashSet<State> = HashSet::new();

    if !initial_state.is_valid() {
        eprintln!("Initial state is invalid!");
        return None;
    }

    queue.push_back(initial_state.clone());
    visited.insert(initial_state);

    while let Some(current_state) = queue.pop_front() {
        if current_state.is_goal() {
            return Some(current_state.steps);
        }

        for next_state in current_state.get_next_states() {
            if visited.insert(next_state.clone()) {
                queue.push_back(next_state);
            }
        }
    }

    None // No solution found
}

fn main() {
    let input = fs::read_to_string("../input.txt").expect("Error reading input file");
    let initial_state_part1 = parse_input(&input);
    let sep = "=".repeat(20);
    println!("{sep} Part 1 {sep}");
    match solve(initial_state_part1) {
        Some(steps) => println!("Minimum steps = {}", steps),
        None => println!("No solution found."),
    }

    let mut initial_state_part2 = parse_input(&input);
    // Add Elerium (gen=0, chip=0) and Dilithium (gen=0, chip=0)
    initial_state_part2.item_floors.push((0, 0)); // Elerium G, M on floor 0
    initial_state_part2.item_floors.push((0, 0)); // Dilithium G, M on floor 0
    initial_state_part2.item_floors.sort_unstable();
    println!("{sep} Part 2 {sep}");
    match solve(initial_state_part2) {
        Some(steps) => println!("Minimum steps = {}", steps),
        None => println!("No solution found."),
    }
}
