from pathlib import Path
from typing import Tuple, List, Dict

def parse_line(data:str) -> Tuple[str, int, int, int]:
    parts = data.split()
    return parts[0], int(parts[3]), int(parts[6]), int(parts[-2])


def calculate_distance(data: Tuple[str, int, int, int], seconds: int) ->int:
    name, speed, active_time, rest_time = data
    cycle_time = active_time + rest_time
    full_cycles = seconds // cycle_time
    remaining_time = seconds % cycle_time
    total = full_cycles * (speed * active_time)
    total += speed * min(remaining_time, active_time)
    return total

def get_fastest(data: List[str], seconds: int) -> str:
    reindeers = []
    for reindeer in data:
        info = parse_line(reindeer)
        name = info[0]
        distance = calculate_distance(info, seconds)
        reindeers.append([name,distance])
    fastest = sorted(reindeers, key=lambda x: x[1])[-1]
    return f"{fastest[0]}: {fastest[1]}"

def calculate_points(data: List[str], total_seconds: int) -> Dict[str, int]:
    reindeers = [parse_line(r) for r in data]
    points = {r[0]: 0 for r in reindeers}
    
    for second in range(1, total_seconds + 1):
        current_distances = []
        for reindeer in reindeers:
            distance = calculate_distance(reindeer, second)
            current_distances.append((reindeer[0], distance))
        
        max_distance = max(d[1] for d in current_distances)
        
        for name, distance in current_distances:
            if distance == max_distance:
                points[name] += 1
    
    return points

def main() -> None:
    data = Path("../input.txt").read_text().strip().splitlines()
    example = Path("../example.txt").read_text().strip().splitlines()
    print("=" * 20, " Part 1 ", "=" * 20)
    print(f"The fastest reindeer is {get_fastest(data, 2503)}")
    print("=" * 20, " Part 2 ", "=" * 20)
    points = calculate_points(data, 2503)
    winner = max(points.items(), key=lambda x: x[1])
    print(f"The winner reindeer is {winner[0]} with {winner[1]} points")
    

if __name__ == "__main__":
    main()