from pathlib import Path
import ast

def parse_calculate(data: list[str]) -> tuple[int,int]:
    total_raw = 0
    total_parsed = 0
    for raw_string in data:
        raw_length = len(raw_string)
        parsed_string = ast.literal_eval(f'{raw_string}')
        parsed_length = len(parsed_string)
        total_raw += raw_length
        total_parsed += parsed_length
    return (total_raw, total_parsed)

def parse_calculate_2(data: list[str]) -> tuple[int,int]:
    total_raw = 0
    total_encoded = 0
    for raw_string in data:
        raw_length = len(raw_string)
        encoded_string = '"' + raw_string.replace('\\', '\\\\').replace('"', '\\"') + '"'
        encoded_length = len(encoded_string)
        total_raw += raw_length
        total_encoded += encoded_length
    return (total_raw, total_encoded)

def main():
    data = Path("../input.txt").read_text().strip().splitlines()
    raw, parsed = parse_calculate(data)
    print("-"*25,"PART 1","-"*25)
    print(f"Total raw length: {raw}")
    print(f"Total parsed length: {parsed}")
    print(f"Difference: {raw - parsed}")
    print("-"*25,"PART 2","-"*25)
    raw, parsed = parse_calculate_2(data)
    print(f"Total raw length: {raw}")
    print(f"Total parsed length: {parsed}")
    print(f"Difference: {abs(raw - parsed)}")

if __name__ == "__main__":
    main()