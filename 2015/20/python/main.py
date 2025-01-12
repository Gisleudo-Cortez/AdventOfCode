from pathlib import Path

def min_houses(target: int) -> int:
    max_house = target // 10
    presents = [0] * (max_house + 1)
    
    for elf in range(1, max_house + 1):
        for house in range(elf, max_house + 1, elf):
            presents[house] += elf * 10
    for house, total in enumerate(presents):
        if total >= target:
            return house
    return -1

def min_11_houses(target: int) -> int:
    max_house = target // 11
    presents = [0] * (max_house + 1)
    
    for elf in range(1, max_house + 1):
        count = 0
        for house in range(elf, max_house + 1, elf):
            presents[house] += elf * 11
            count += 1
            if count == 50:
                break
    for house, total in enumerate(presents):
        if total >= target:
            return house
    return -1

def main() -> None:
    data = int(Path("../input.txt").read_text().strip())
    
    print("=" * 20, "Part 1", "=" * 20)
    house = min_houses(data)
    print(f"The first house to receive at least {data} presents is house {house}")
    
    print("=" * 20, "Part 2", "=" * 20)
    house = min_11_houses(data)
    print(f"The first house to receive at least {data} presents is house {house}")

if __name__ == "__main__":
    main()
