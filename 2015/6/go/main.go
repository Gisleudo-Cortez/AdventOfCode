package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func splitAtFirstNumber(s string) (string, string) {
	re := regexp.MustCompile(`\d`)
	match := re.FindStringIndex(s)

	if match == nil {
		return strings.TrimSpace(s), ""
	}

	idx := match[0]
	return strings.TrimSpace(s[:idx]), strings.TrimSpace(s[idx:])
}

func getCoords(s string) ([][]int, error) {
	if s == "" {
		return [][]int{{0, 0}, {0, 0}}, nil
	}

	parts := strings.Split(s, " ")
	if len(parts) < 1 {
		return nil, fmt.Errorf("invalid coordinate format: %s", s)
	}

	beginningParts := strings.Split(parts[0], ",")
	endParts := strings.Split(parts[len(parts)-1], ",")

	beginning := make([]int, 2)
	end := make([]int, 2)

	var err error
	beginning[0], err = strconv.Atoi(beginningParts[0])
	if err != nil {
		return nil, err
	}
	beginning[1], err = strconv.Atoi(beginningParts[1])
	if err != nil {
		return nil, err
	}

	end[0], err = strconv.Atoi(endParts[0])
	if err != nil {
		return nil, err
	}
	end[1], err = strconv.Atoi(endParts[1])
	if err != nil {
		return nil, err
	}

	return [][]int{beginning, end}, nil
}

func toggleCellPart1(grid [][]int, bx, by, ex, ey, target int) [][]int {
	rows, cols := len(grid), len(grid[0])

	// Clip coordinates
	if bx < 0 {
		bx = 0
	}
	if by < 0 {
		by = 0
	}
	if ex > rows {
		ex = rows
	}
	if ey > cols {
		ey = cols
	}

	for x := bx; x < ex; x++ {
		for y := by; y < ey; y++ {
			switch target {
			case 1: // turn on
				grid[x][y] = 1
			case 0: // turn off
				grid[x][y] = 0
			default: // toggle
				grid[x][y] = 1 - grid[x][y]
			}
		}
	}

	return grid
}

func toggleCellPart2(grid [][]int, bx, by, ex, ey, target int) [][]int {
	rows, cols := len(grid), len(grid[0])

	// Clip coordinates
	if bx < 0 {
		bx = 0
	}
	if by < 0 {
		by = 0
	}
	if ex > rows {
		ex = rows
	}
	if ey > cols {
		ey = cols
	}

	for x := bx; x < ex; x++ {
		for y := by; y < ey; y++ {
			switch target {
			case 1: // turn on: increase by 1
				grid[x][y]++
			case 0: // turn off: decrease by 1 but not below 0
				if grid[x][y] > 0 {
					grid[x][y]--
				}
			default: // toggle: increase by 2
				grid[x][y] += 2
			}
		}
	}

	return grid
}

func processLights(data []string, grid [][]int, toggleFunc func([][]int, int, int, int, int, int) [][]int) int {
	gridCopy := make([][]int, len(grid))
	for i := range grid {
		gridCopy[i] = make([]int, len(grid[0]))
		copy(gridCopy[i], grid[i])
	}

	for _, task := range data {
		operation, coords := splitAtFirstNumber(task)
		coordsParts, err := getCoords(coords)
		if err != nil {
			fmt.Printf("Error processing task '%s': %v\n", task, err)
			continue
		}

		bx, by := coordsParts[0][0], coordsParts[0][1]
		ex, ey := coordsParts[1][0]+1, coordsParts[1][1]+1

		switch operation {
		case "turn on":
			gridCopy = toggleFunc(gridCopy, bx, by, ex, ey, 1)
		case "turn off":
			gridCopy = toggleFunc(gridCopy, bx, by, ex, ey, 0)
		case "toggle":
			gridCopy = toggleFunc(gridCopy, bx, by, ex, ey, -1)
		}
	}

	return sumGrid(gridCopy)
}

func sumGrid(grid [][]int) int {
	total := 0
	for _, row := range grid {
		for _, val := range row {
			total += val
		}
	}
	return total
}

func readInput(filePath string) []string {
	content, err := os.ReadFile(filePath)
	if err != nil {
		fmt.Printf("Error reading input file %s: %v\n", filePath, err)
		return []string{}
	}

	lines := strings.Split(strings.TrimSpace(string(content)), "\n")
	return lines
}

func main() {
	rows, cols := 1000, 1000
	grid := make([][]int, rows)
	for i := range grid {
		grid[i] = make([]int, cols)
	}
	data := readInput("../input.txt")

	if len(data) == 0 {
		fmt.Println("Error: Could not process input files")
		return
	}
	answer1 := processLights(data, grid, toggleCellPart1)

	fmt.Printf("Part 1 - Main input result: %d\n", answer1)
	answer2 := processLights(data, grid, toggleCellPart2)

	fmt.Printf("Part 2 - Main input result: %d\n", answer2)

}
