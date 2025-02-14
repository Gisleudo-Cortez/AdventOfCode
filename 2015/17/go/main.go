package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"sort"
	"strconv"
	"strings"
)

func backtrack(index int, remaining int, comboLength int, combByLength map[int]int, containers []int) {
	if remaining == 0 {
		combByLength[comboLength]++
		return
	}
	if remaining < 0 || index >= len(containers) {
		return
	}
	backtrack(index+1, remaining-containers[index], comboLength+1, combByLength, containers)
	backtrack(index+1, remaining, comboLength, combByLength, containers)
}

func findCombinations(containers []int, target int) (int, int) {
	sort.Slice(containers, func(i, j int) bool {
		return containers[i] > containers[j]
	})
	combByLength := make(map[int]int)
	backtrack(0, target, 0, combByLength, containers)
	if len(combByLength) == 0 {
		return 0, 0
	}
	totalCombinations := 0
	minLength := int(^uint(0) >> 1)
	for length, count := range combByLength {
		totalCombinations += count
		if length < minLength {
			minLength = length
		}
	}
	minLengthCombinations := combByLength[minLength]
	return totalCombinations, minLengthCombinations
}

func parseInput(path string) ([]int, error) {
	bytes, err := ioutil.ReadFile(path)
	if err != nil {
		return nil, err
	}
	lines := strings.Split(strings.TrimSpace(string(bytes)), "\n")
	var containers []int
	for _, line := range lines {
		line = strings.TrimSpace(line)
		if line == "" {
			continue
		}
		val, err := strconv.Atoi(line)
		if err != nil {
			return nil, err
		}
		containers = append(containers, val)
	}
	return containers, nil
}

func main() {
	separator := strings.Repeat("=", 20)
	targetVolume := 150
	containers, err := parseInput("../input.txt")
	if err != nil {
		fmt.Printf("Error parsing input file: %v\n", err)
		os.Exit(1)
	}
	if len(containers) == 0 {
		fmt.Println("No valid data in file")
		return
	}
	totalCombinations, minCombinations := findCombinations(containers, targetVolume)
	fmt.Printf("%s Part 1 %s \nTotal combinations: %d\n", separator, separator, totalCombinations)
	fmt.Printf("%s Part 2 %s \nMinimum containers combinations: %d\n", separator, separator, minCombinations)
}
