from pathlib import Path

def solve_part(data: str , n_iter: int) -> int:
    text = data
    for _ in range(n_iter):
        out_str = ""
        count = 1
        for i in range(1, len(text)):
            if text[i] == text[i - 1]:
                count += 1
            else:
                out_str += str(count) + text[i - 1]
                count = 1
        out_str += str(count) + text[-1]
        text = out_str
    return len(text)

def main() -> None:
    data = Path("../input.txt").read_text().strip()
    #example = Path("../example.txt").read_text().strip()
    print("="*20," Part 1 ", "="*20)
    #print(f"Example: {solve_part(example,4)}")
    print(f"Data: {solve_part(data,40)}")
    print("="*20," Part 2 ", "="*20)
    print(f"Data: {solve_part(data,50)}")

if __name__ == "__main__":
    main()