with open("../input.txt", "r") as file:
    data = file.read()

def part1(data: str) -> int:
    counter = 0
    for c in data:
        match c:
            case "(":
                counter += 1
            case ")":
                counter -= 1
    return counter

def part2(data: str) -> int:
    counter = 0
    for i, c in enumerate(data):
        match c:
            case "(":
                counter += 1
            case ")":
                counter -= 1
        if counter < 0:
            return i + 1

def main():
    print(f"Part 1: {part1(data)}")
    print(f"Part 2: {part2(data)}")

if __name__ == "__main__":
    main()