import time
from functools import cache
from statistics import mean, stdev
import sys
from typing import Callable, List, Dict, Set

# Original version
class OriginalGenerator:
    @cache
    def check_if_valid(self, data: str) -> bool:
        alphabet_dict = {
            'a': 0, 'b': 1, 'c': 2, 'd': 3, 'e': 4, 'f': 5, 'g': 6, 'h': 7,
            'i': 8, 'j': 9, 'k': 10, 'l': 11, 'm': 12, 'n': 13, 'o': 14,
            'p': 15, 'q': 16, 'r': 17, 's': 18, 't': 19, 'u': 20, 'v': 21,
            'w': 22, 'x': 23, 'y': 24, 'z': 25
        }
        r1, r2, r3 = False, False, False
        
        # check if password contains three increasing letters
        for idx in range(len(data) - 2):
            vone = alphabet_dict[data[idx]]
            vtwo = alphabet_dict[data[idx + 1]]
            vthree = alphabet_dict[data[idx + 2]]
            if vtwo == vone + 1 and vthree == vtwo + 1:
                r1 = True
                break
        
        # check if the password contains i,o or l
        forbiden = ['i', 'o', 'l']
        for idx in range(len(data)):
            if data[idx] in forbiden:
                r2 = False
                break
        else:
            r2 = True
        
        # must contain two different pairs
        matches = set()
        for idx in range(len(data) - 1):
            if data[idx] == data[idx + 1]:
                matches.add((data[idx], data[idx + 1]))
            if len(matches) >= 2:
                r3 = True
        
        return r1 and r2 and r3

    @cache
    def generate_password(self, data: str) -> str:
        alphabet_dict = {
            'a': 0, 'b': 1, 'c': 2, 'd': 3, 'e': 4, 'f': 5, 'g': 6, 'h': 7,
            'i': 8, 'j': 9, 'k': 10, 'l': 11, 'm': 12, 'n': 13, 'o': 14,
            'p': 15, 'q': 16, 'r': 17, 's': 18, 't': 19, 'u': 20, 'v': 21,
            'w': 22, 'x': 23, 'y': 24, 'z': 25
        }
        
        valid = self.check_if_valid(data)
        while not valid:
            chars = list(data)
            idx = len(chars) - 1
            carry = True
            
            while carry and idx >= 0:
                if chars[idx] == 'z':
                    chars[idx] = 'a'
                    carry = True
                else:
                    current_value = alphabet_dict[chars[idx]]
                    next_letter = chr(ord('a') + ((current_value + 1) % 26))
                    chars[idx] = next_letter
                    carry = False
                idx -= 1
            
            data = ''.join(chars)
            valid = self.check_if_valid(data)
        return data

# Optimized version
class OptimizedGenerator:
    def __init__(self):
        self.FORBIDDEN = {'i', 'o', 'l'}
        self.ALPHABET = 'abcdefghijklmnopqrstuvwxyz'
        self.ALPHA_TO_NUM = {c: i for i, c in enumerate(self.ALPHABET)}
        self.NUM_TO_ALPHA = {i: c for i, c in enumerate(self.ALPHABET)}

    @cache
    def has_increasing_straight(self, password: str) -> bool:
        for i in range(len(password) - 2):
            if (self.ALPHA_TO_NUM[password[i + 1]] == self.ALPHA_TO_NUM[password[i]] + 1 and 
                self.ALPHA_TO_NUM[password[i + 2]] == self.ALPHA_TO_NUM[password[i]] + 2):
                return True
        return False

    @cache
    def has_two_pairs(self, password: str) -> bool:
        pairs = set()
        i = 0
        while i < len(password) - 1:
            if password[i] == password[i + 1]:
                pairs.add(password[i])
                i += 2
            else:
                i += 1
            if len(pairs) >= 2:
                return True
        return False

    def increment_password(self, password: str) -> str:
        chars = list(password)
        i = len(chars) - 1
        
        while i >= 0:
            current_char = chars[i]
            next_char = self.NUM_TO_ALPHA[(self.ALPHA_TO_NUM[current_char] + 1) % 26]
            
            if next_char in self.FORBIDDEN:
                next_char = self.NUM_TO_ALPHA[(self.ALPHA_TO_NUM[next_char] + 1) % 26]
            
            chars[i] = next_char
            
            if current_char != 'z':
                break
            i -= 1
        
        return ''.join(chars)

    def generate_password(self, password: str) -> str:
        password = self.increment_password(password)
        
        while True:
            if any(c in self.FORBIDDEN for c in password):
                idx = next(i for i, c in enumerate(password) if c in self.FORBIDDEN)
                password = (
                    password[:idx] + 
                    self.NUM_TO_ALPHA[(self.ALPHA_TO_NUM[password[idx]] + 1) % 26] + 
                    'a' * (len(password) - idx - 1)
                )
                continue
                
            if self.has_increasing_straight(password) and self.has_two_pairs(password):
                return password
                
            password = self.increment_password(password)

def benchmark_function(func: Callable, input_data: str, num_runs: int = 5) -> float:
    """Benchmark a function and return mean execution time."""
    times = []
    for _ in range(num_runs):
        start_time = time.perf_counter()
        func(input_data)
        end_time = time.perf_counter()
        times.append(end_time - start_time)
    return mean(times)

def print_row(cols, widths):
    """Print a row with proper spacing."""
    row = "| "
    for col, width in zip(cols, widths):
        row += str(col).ljust(width) + " | "
    print(row)

def print_separator(widths):
    """Print a separator line."""
    line = "+"
    for width in widths:
        line += "-" * (width + 2) + "+"
    print(line)

def main():
    test_cases = [
        "abcdefgh",  # Ex 1
        "ghijklmn",  # Ex 2
        "hepxcrrq"   # Input
    ]
    
    original = OriginalGenerator()
    optimized = OptimizedGenerator()
    
    # Define column headers and their widths
    headers = ["Input", "Original (s)", "Optimized (s)", "Improvement %"]
    widths = [10, 12, 12, 13]  # Adjust these based on your expected output lengths
    
    # Print table header
    print("\nPassword Generator Performance Comparison")
    print("=" * 55)
    
    # Print headers
    print_separator(widths)
    print_row(headers, widths)
    print_separator(widths)
    
    # Print data rows
    for test_case in test_cases:
        # Benchmark both versions
        orig_time = benchmark_function(original.generate_password, test_case)
        opt_time = benchmark_function(optimized.generate_password, test_case)
        
        # Calculate improvement
        improvement = (orig_time - opt_time) / orig_time * 100
        
        # Format row data
        row = [
            test_case,
            f"{orig_time:.6f}",
            f"{opt_time:.6f}",
            f"{improvement:.2f}%"
        ]
        
        print_row(row, widths)
    
    print_separator(widths)

if __name__ == "__main__":
    main()