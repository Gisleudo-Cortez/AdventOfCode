use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug)]
struct StringChecker;

impl StringChecker {
    fn count_vowels(s: &str) -> usize {
        s.chars()
            .filter(|&c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
            .count()
    }

    fn has_double_letter(s: &str) -> bool {
        s.as_bytes()
            .windows(2)
            .any(|window| window[0] == window[1])
    }

    fn has_forbidden_strings(s: &str) -> bool {
        ["ab", "cd", "pq", "xy"]
            .iter()
            .any(|&forbidden| s.contains(forbidden))
    }

    fn is_nice_string_part1(s: &str) -> bool {
        Self::count_vowels(s) >= 3 
            && Self::has_double_letter(s) 
            && !Self::has_forbidden_strings(s)
    }

    fn has_repeating_pair(s: &str) -> bool {
        let bytes = s.as_bytes();
        bytes
            .windows(2)
            .enumerate()
            .any(|(i, pair)| {
                bytes[i+2..]
                    .windows(2)
                    .any(|window| window == pair)
            })
    }

    fn has_repeat_with_one_between(s: &str) -> bool {
        s.as_bytes()
            .windows(3)
            .any(|window| window[0] == window[2])
    }

    fn is_nice_string_part2(s: &str) -> bool {
        Self::has_repeating_pair(s) && Self::has_repeat_with_one_between(s)
    }
}

fn part1(strings: &[String]) -> usize {
    strings
        .iter()
        .filter(|s| StringChecker::is_nice_string_part1(s))
        .count()
}

fn part2(strings: &[String]) -> usize {
    strings
        .iter()
        .filter(|s| StringChecker::is_nice_string_part2(s))
        .count()
}

fn main() -> std::io::Result<()> {
    let input_path = Path::new("../input.txt");
    let content = read_to_string(input_path)?;
    let strings: Vec<String> = content.lines().map(String::from).collect();

    let nice_count_part1 = part1(&strings);
    println!("Part 1 - Number of nice strings: {}", nice_count_part1);

    let nice_count_part2 = part2(&strings);
    println!("Part 2 - Number of nice strings: {}", nice_count_part2);

    Ok(())
}