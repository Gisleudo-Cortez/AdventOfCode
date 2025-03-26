import sys

def solve_part1(data):
    decompressed_length = 0
    i = 0
    n = len(data)
    while i < n:
        if data[i] == '(':
            close_paren_idx = data.find(')', i)
            if close_paren_idx == -1:
                raise ValueError(f"Unmatched parenthesis starting at index {i}")

            marker_content = data[i + 1:close_paren_idx]
            try:
                length, repetitions = map(int, marker_content.split('x'))
            except ValueError:
                raise ValueError(f"Invalid marker format: ({marker_content})")

            decompressed_length += length * repetitions
            i = close_paren_idx + 1 + length
        else:
            decompressed_length += 1
            i += 1
    return decompressed_length

def calculate_decompressed_length_recursive(data, start, end):
    decompressed_length = 0
    i = start
    while i < end:
        if data[i] == '(':
            close_paren_idx = data.find(')', i)
            if close_paren_idx == -1 or close_paren_idx >= end:
                 raise ValueError(f"Unmatched or out-of-bounds parenthesis starting at index {i}")

            marker_content = data[i + 1:close_paren_idx]
            try:
                length, repetitions = map(int, marker_content.split('x'))
            except ValueError:
                 raise ValueError(f"Invalid marker format: ({marker_content})")

            sub_start = close_paren_idx + 1
            sub_end = sub_start + length

            if sub_end > end:
                raise ValueError(f"Marker at {i} references data beyond segment end {end}")

            sub_length = calculate_decompressed_length_recursive(data, sub_start, sub_end)
            decompressed_length += sub_length * repetitions
            i = sub_end
        else:
            decompressed_length += 1
            i += 1
    return decompressed_length

def solve_part2(data):
    return calculate_decompressed_length_recursive(data, 0, len(data))

input_file_path = "../input.txt"

try:
    with open(input_file_path, 'r') as f:
        input_data = f.read().strip()
except FileNotFoundError:
    print(f"Error: {input_file_path} not found.")
    input_data = None
except Exception as e:
    print(f"Error reading file: {e}")
    input_data = None


if input_data:
    try:
        part1_length = solve_part1(input_data)
        print("="*20," Part 1", "="*20, f"\nDecompressed length = {part1_length}")
        part2_length = solve_part2(input_data)
        print("="*20," Part 2", "="*20, f"\nDecompressed length = {part2_length}")
    except ValueError as e:
        print(f"Processing Error: {e}")
    except RecursionError:
         print("Error: Maximum recursion depth exceeded. The input might be extremely deeply nested.")
