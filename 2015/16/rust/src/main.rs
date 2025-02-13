use std::{collections::HashMap, fs};

struct Sue {
    number: i32,
    properties: HashMap<String, i32>,
}

struct GiftAnalyzer {
    sues: Vec<Sue>,
}

impl GiftAnalyzer {
    const MFCSAM_READING: [(&'static str, i32); 10] = [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ];

    const GREATER_THAN_PROPS: [&'static str; 2] = ["cats", "trees"];
    const LESS_THAN_PROPS: [&'static str; 2] = ["pomeranians", "goldfish"];

    pub fn new(path: &str) -> Self {
        let mut sues = Vec::new();

        let data = fs::read_to_string(path)
            .expect("Error reading input file")
            .trim()
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<_>>();

        for line in data {
            let sue: Vec<&str> = line.split_whitespace().collect();

            if sue.len() < 8 {
                eprintln!("Skipping malformed line: {}", line);
                continue;
            }

            let number = sue[1].trim_end_matches(':').parse::<i32>().unwrap();
            let item_1_nm = sue[2].trim_end_matches(':').to_string();
            let item_1_qtd = sue[3].trim_end_matches(',').parse::<i32>().unwrap();

            let item_2_nm = sue[4].trim_end_matches(':').to_string();
            let item_2_qtd = sue[5].trim_end_matches(',').parse::<i32>().unwrap();

            let item_3_nm = sue[6].trim_end_matches(':').to_string();
            let item_3_qtd = sue[7].trim_end_matches(',').parse::<i32>().unwrap();

            let mut properties = HashMap::new();
            properties.insert(item_1_nm, item_1_qtd);
            properties.insert(item_2_nm, item_2_qtd);
            properties.insert(item_3_nm, item_3_qtd);

            sues.push(Sue {
                number,
                properties,
            });
        }

        Self { sues }
    }

    fn is_exact_match(&self, sue: &Sue) -> bool {
        for (prop, &value) in sue.properties.iter() {
            if let Some(expected) = Self::MFCSAM_READING
                .iter()
                .find(|&&(k, _)| k == prop)
                .map(|&(_, v)| v)
            {
                if value != expected {
                    return false;
                }
            }
        }
        true
    }

    fn is_range_match(&self, sue: &Sue) -> bool {
        for (prop, value) in sue.properties.iter() {
           
            if let Some(expected) = Self::MFCSAM_READING
                .iter()
                .find(|&&(k, _)| k == prop)
                .map(|&(_, v)| v)
            {
                
                if Self::GREATER_THAN_PROPS.contains(&prop.as_str()) {
                    if value <= &expected {
                        return false;
                    }

                } else if Self::LESS_THAN_PROPS.contains(&prop.as_str()) {
                    if value >= &expected {
                        return false;
                    }
                    
                } else {
                
                    if value != &expected {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn find_exact_match(&self) -> Option<i32> {
        for sue in &self.sues {
            if self.is_exact_match(sue) {
                return Some(sue.number);
            }
        }
        None
    }

    pub fn find_range_match(&self) -> Option<i32> {
        for sue in &self.sues {
            if self.is_range_match(sue) {
                return Some(sue.number);
            }
        }
        None
    }
}

fn main() {
    let analyzer = GiftAnalyzer::new("../input.txt");
    let sep = "=".repeat(20);
    let exact_match = analyzer.find_exact_match().unwrap();
    println!(
        "{} Part 1 {}\nThe aunt sue that got you the gift is {}",
        sep, sep, exact_match
    );

    let range_match = analyzer.find_range_match().unwrap();
    println!(
        "{} Part 2 {}\nThe aunt sue that got you the gift is {}",
        sep, sep, range_match
    );
}
