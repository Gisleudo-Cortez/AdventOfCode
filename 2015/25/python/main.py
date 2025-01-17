import re
from pathlib import Path

def get_code(r: int, c: int) -> int:
    current_value = 20151125
    value = lambda x: (x * 252533) % 33554393

    # Calculate the target position in the sequence
    target_index = sum(range(1, r + c)) - (r - 1)

    # Generate values until the target index
    for _ in range(1, target_index):
        current_value = value(current_value)

    return current_value

def main() -> None:
    rows, cols = [int(x) for x in re.findall(r"\d+", Path("../input.txt").read_text().strip())]
    code = get_code(rows, cols)
    print(f"The code to start the machine is: {code}")

if __name__ == "__main__":
    main()
