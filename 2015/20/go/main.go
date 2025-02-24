package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func minHouses(target int) int {
	max_houses := target / 10
	presents := make([]int, max_houses+1)

	for elf := 1; elf <= max_houses; elf++ {
		for house := elf; house <= max_houses; house += elf {
			presents[house] += elf * 10
		}
	}
	for house, total := range presents {
		if total >= target {
			return house
		}
	}
	return 0
}

func min11Houses(target int) int {
	max_houses := target / 10
	presents := make([]int, max_houses+1)

	for elf := 1; elf <= max_houses; elf++ {
		count := 0
		for house := elf; house <= max_houses; house += elf {
			presents[house] += elf * 11
			count += 1
			if count == 50 {
				break
			}
		}
	}
	for house, total := range presents {
		if total >= target {
			return house
		}
	}
	return 0
}

func main() {
	data, err := os.ReadFile("../input.txt")
	if err != nil {
		fmt.Println("Error reading input file: ", err)
		return
	}
	target, err := strconv.Atoi(strings.TrimSpace(string(data)))
	if err != nil {
		fmt.Println("Error parsing input into int: ", err)
		return
	}

	part1 := minHouses(target)
	part2 := min11Houses(target)
	sep := strings.Repeat("=", 20)
	fmt.Printf("%s Part 1 %s\nThe first house to recive at least %d presents is house nº %d\n", sep, sep, target, part1)
	fmt.Printf("%s Part 2 %s\nThe first house to recive at least %d presents is house nº %d\n", sep, sep, target, part2)
}
