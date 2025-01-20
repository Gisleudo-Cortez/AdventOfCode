package main

import (
	"fmt"
	"os"
)

type Position struct {
	x int
	y int
}

type PositionKey struct {
	x, y int
}

type positionSet map[PositionKey]struct{}

func (s positionSet) add(pos Position) {
	key := PositionKey{x: pos.x, y: pos.y}
	s[key] = struct{}{}
}

func (s positionSet) has(pos Position) bool {
	key := PositionKey{x: pos.x, y: pos.y}
	_, ok := s[key]
	return ok
}

func calculate_unique_houses(movements string) (int, error) {
	current := Position{x: 0, y: 0}

	visited := make(positionSet)
	visited.add(current)

	directions := map[rune]Position{
		'>': {x: 1, y: 0},
		'<': {x: -1, y: 0},
		'^': {x: 0, y: 1},
		'v': {x: 0, y: -1},
	}
	for _, move := range movements {
		dir, ok := directions[move]
		if !ok {
			return 0, fmt.Errorf("invalid direction %c", move)
		}
		current.x += dir.x
		current.y += dir.y
		visited.add(current)
	}
	return len(visited), nil
}

func calculate_unique_houses_robo_santa(movements string) (int, error) {
	santaPos := Position{x: 0, y: 0}
	roboPos := Position{x: 0, y: 0}

	visited := make(positionSet)
	visited.add(santaPos)

	directions := map[rune]Position{
		'>': {x: 1, y: 0},
		'<': {x: -1, y: 0},
		'^': {x: 0, y: 1},
		'v': {x: 0, y: -1},
	}
	santaTurn := true
	for _, move := range movements {
		dir, ok := directions[move]
		if !ok {
			return 0, fmt.Errorf("invalid direction %c", move)
		}
		if santaTurn {
			santaPos.x += dir.x
			santaPos.y += dir.y
			visited.add(santaPos)
		} else {
			roboPos.x += dir.x
			roboPos.y += dir.y
			visited.add(roboPos)
		}
		santaTurn = !santaTurn
	}
	return len(visited), nil
}

func main() {
	data, err := os.ReadFile("../input.txt")
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error reading file %v\n", err)
		os.Exit(1)
	}
	houses, err := calculate_unique_houses(string(data))
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error solving Part 1 %v\n", err)
		os.Exit(1)
	}
	fmt.Printf("Part 1: %d\n", houses)

	robo, err := calculate_unique_houses_robo_santa(string(data))
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error solving Part 2 %v\n", err)
		os.Exit(1)
	}
	fmt.Printf("Part 1: %d\n", robo)
}
