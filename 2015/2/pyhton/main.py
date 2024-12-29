with open("../input.txt") as file:
    data = file.read().splitlines()

def part1(data:list[str]) -> int:
    total = 0
    for line in data:
        l, w, h = map(int ,line.split("x"))
        smallest = min(l*w, w*h, h*l)
        area = ((2*l*w) + (2*w*h) + (2*h*l)) + smallest
        total += area
    return total

def part2(data:list[str]):
    total = 0
    for line in data:
        l, w, h = map(int ,line.split("x"))
        warp = sorted([l,w,h])
        warp = warp[0] * 2 + warp[1] * 2
        bow = l*w*h
        total += (warp + bow)
    return total

def main():
    print(f"Part 1: {part1(data)}\n")
    print(f"Part 2: {part2(data)}\n")


if __name__ == "__main__":
    main()
    