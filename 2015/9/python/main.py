from pathlib import Path
from typing import List, Dict, Tuple
from itertools import permutations

def parse_input(data: List[str]) -> Dict[str, Dict[str, int]]:
    nodes = {}
    for item in data:
        parts = item.split()
        origin = parts[0]
        destination = parts[2]
        cost = int(parts[-1])
        if origin not in nodes:
            nodes[origin] = {}
        if destination not in nodes:
            nodes[destination] = {}
        nodes[origin][destination] = cost
        nodes[destination][origin] = cost #  the cost form dest to origin is the same as the oposite
    return nodes

# Part 1
def shortest_hamiltonian_path(nodes: Dict[str, Dict[str, int]]) -> Tuple[list[str], int]:
    shortest_distance = float("inf")
    shortest_route = []
    cities = list(nodes.keys())
    
    
    for route in permutations(cities):
        current_distance = 0
        for i in range(len(route) - 1):
            city = route[i]
            next_city = route[i+1]

            if city in nodes and next_city in nodes[city]:
                current_distance += nodes[city][next_city]
            elif next_city in nodes and city in nodes[next_city]:
                current_distance += nodes[next_city][city]
            else:
                # Just in case there is no node relating to the city
                print(f"Error, no distance from {city} to {next_city}")
                break
        if current_distance < shortest_distance:
            shortest_distance = current_distance
            shortest_route = route
    
    return shortest_route, shortest_distance

def longest_hamiltonian_path(nodes: Dict[str, Dict[str, int]]) -> Tuple[list[str], int]:
    longest_distance = float("-inf")
    longest_route = []
    cities = list(nodes.keys())
    
    
    for route in permutations(cities):
        current_distance = 0
        for i in range(len(route) - 1):
            city = route[i]
            next_city = route[i+1]

            if city in nodes and next_city in nodes[city]:
                current_distance += nodes[city][next_city]
            elif next_city in nodes and city in nodes[next_city]:
                current_distance += nodes[next_city][city]
            else:
                # Just in case there is no node relating to the city
                print(f"Error, no distance from {city} to {next_city}")
                break
        if current_distance > longest_distance:
            longest_distance = current_distance
            longest_route = route
    
    return longest_route, longest_distance


def main() -> None:
    data = Path("../input.txt").read_text().strip().splitlines()
    example = Path("../example.txt").read_text().strip().splitlines()
    print("="*25,"Part 1","="*25)
    nodes = parse_input(data)
    route, distance = shortest_hamiltonian_path(nodes)
    print(f"The Shortest route in the data is {" -> ".join(route)}. with the distance of {distance}")

    nodes = parse_input(example)
    route, distance = shortest_hamiltonian_path(nodes)
    print(f"The Shortest route in the example is {" -> ".join(route)}. with the distance of {distance}")

    print("="*25,"Part 2","="*25)
    nodes = parse_input(data)
    route, distance = longest_hamiltonian_path(nodes)
    print(f"The Longest route in the data is {" -> ".join(route)}. with the distance of {distance}")

    nodes = parse_input(example)
    route, distance = longest_hamiltonian_path(nodes)
    print(f"The Longest route in the example is {" -> ".join(route)}. with the distance of {distance}")

if __name__ == "__main__":
    main()