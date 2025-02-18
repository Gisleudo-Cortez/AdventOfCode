package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func genGrid(path string) [][]rune {
	file, err := os.Open(path)
	if err != nil {
		panic("Error parsing input file")
	}
	defer file.Close()

	var data [][]rune
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		data = append(data, []rune(scanner.Text()))
	}

	return data
}

func simulateStep(grid [][]rune, stuck bool) ([][]rune, int) {
	rows, cols := len(grid), len(grid[0])
	simulated := make([][]rune, rows)
	newGrid := make([][]rune, rows)

	for i := range grid {
		simulated[i] = make([]rune, cols)
		newGrid[i] = make([]rune, cols)
		copy(simulated[i], grid[i])
		copy(newGrid[i], grid[i])
	}

	if stuck {
		corners := [][2]int{{0, 0}, {0, cols - 1}, {rows - 1, 0}, {rows - 1, cols - 1}}
		for _, c := range corners {
			newGrid[c[0]][c[1]] = '#'
			simulated[c[0]][c[1]] = '#'
		}
	}

	neighbors := [][2]int{
		{-1, -1}, {-1, 0}, {-1, 1},
		{0, -1}, {0, 1},
		{1, -1}, {1, 0}, {1, 1},
	}

	for r := 0; r < rows; r++ {
		for c := 0; c < cols; c++ {
			if stuck && ((r == 0 && c == 0) || (r == 0 && c == cols-1) || (r == rows-1 && c == 0) || (r == rows-1 && c == cols-1)) {
				continue
			}

			nLights := 0
			for _, n := range neighbors {
				rr, cc := r+n[0], c+n[1]
				if rr >= 0 && rr < rows && cc >= 0 && cc < cols && simulated[rr][cc] == '#' {
					nLights++
				}
			}

			if simulated[r][c] == '#' && !(nLights == 2 || nLights == 3) {
				newGrid[r][c] = '.'
			} else if simulated[r][c] == '.' && nLights == 3 {
				newGrid[r][c] = '#'
			}
		}
	}

	lightsOn := 0
	for _, row := range newGrid {
		for _, cell := range row {
			if cell == '#' {
				lightsOn++
			}
		}
	}

	return newGrid, lightsOn
}

func simulateNSteps(grid [][]rune, nSteps int, stuck bool) int {
	finalCount := 0
	for i := 0; i < nSteps; i++ {
		grid, finalCount = simulateStep(grid, stuck)
	}
	return finalCount
}

func main() {
	grid := genGrid("../input.txt")
	sep := strings.Repeat("=", 20)
	total1 := simulateNSteps(grid, 100, false)
	total2 := simulateNSteps(grid, 100, true)

	fmt.Printf("%s Part 1 %s\nTotal lights on after 100 steps and not stuck: %d\n", sep, sep, total1)
	fmt.Printf("%s Part 2 %s\nTotal lights on after 100 steps and stuck: %d\n", sep, sep, total2)
}
