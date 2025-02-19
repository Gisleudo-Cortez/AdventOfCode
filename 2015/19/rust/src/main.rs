use std::{collections::HashSet, fs};

fn parse(path: &str) -> (Vec<(String, String)>, String) {
    let input = fs::read_to_string(path).expect("Error reading input file");
    let parts: Vec<&str> = input.trim().split("\n\n").collect();
    
    let rules_data = parts[0];
    let molecule = parts[1].to_string();

    let rules: Vec<(String, String)> = rules_data
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(" => ").collect();
            (parts[0].to_string(), parts[1].to_string())
        })
        .collect();
    
    (rules, molecule)
}

fn gen_replacements(molecule: &str, rules: &[(String, String)]) -> usize {
    let mut molecules: HashSet<String> = HashSet::new();

    for (from, to) in rules {
        let mut start_index = 0;
        while let Some(pos) = molecule[start_index..].find(from) {
            let pos = start_index + pos;
            let new_molecule = format!(
                "{}{}{}",
                &molecule[..pos],
                to,
                &molecule[pos + from.len()..],
            );
            molecules.insert(new_molecule);
            start_index = pos + 1;
        }
    }

    molecules.len()
}

fn gen_from_e(molecule: &str, rules: &[(String, String)]) -> i32 {
    let mut steps = 0;
    let mut current = molecule.to_string();

    while current != "e" {
        let mut replaced = false;
        
        for (from, to) in rules {
            if let Some(pos) = current.find(to) {
                current = format!(
                    "{}{}{}",
                    &current[..pos],
                    from,
                    &current[pos + to.len()..]
                );
                steps += 1;
                replaced = true;
                break;
            }
        }
        if !replaced {
            return -1;
        }
    }
    steps
}

fn main() {
    let (rules, molecule) = parse("../input.txt");
    let sep = "=".repeat(20);
    let part1 = gen_replacements(&molecule, &rules);
    let part2 = gen_from_e(&molecule, &rules);
    println!("{} Part 1 {}\nTotal number of unique combinations is: {}", sep, sep, part1);
    println!("{} Part 2 {}\nTotal number of steps required is: {}", sep, sep, part2);
}
