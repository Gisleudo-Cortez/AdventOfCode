from pathlib import Path
import re

def count_vowels(s: str) -> int:
    return sum(1 for c in s if c in 'aeiou')

def has_double_letter(s: str) -> bool:
    return any(s[i] == s[i+1] for i in range(len(s)-1))

def has_forbidden_strings(s: str) -> bool:
    return any(bad in s for bad in ['ab', 'cd', 'pq', 'xy'])

def is_nice_string_part1(s: str) -> bool:
    return (count_vowels(s) >= 3 and 
            has_double_letter(s) and 
            not has_forbidden_strings(s))

def has_repeating_pair(s: str) -> bool:
    for i in range(len(s)-1):
        pair = s[i:i+2]
        if pair in s[i+2:]:
            return True
    return False

def has_repeat_with_one_between(s: str) -> bool:
    return any(s[i] == s[i+2] for i in range(len(s)-2))

def is_nice_string_part2(s: str) -> bool:
    return has_repeating_pair(s) and has_repeat_with_one_between(s)

def part_1(strings: list[str]) -> int:
    return sum(1 for s in strings if is_nice_string_part1(s))

def part_2(strings: list[str]) -> int:
    return sum(1 for s in strings if is_nice_string_part2(s))

def main():
    strings = Path("../input.txt").read_text().strip().split('\n')
    
    nice_count_part1 = part_1(strings)
    print(f"Part 1 - Number of nice strings: {nice_count_part1}")
    
    nice_count_part2 = part_2(strings)
    print(f"Part 2 - Number of nice strings: {nice_count_part2}")

if __name__ == "__main__":
    main()