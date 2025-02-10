package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Preferences map[string]map[string]int

func parse(data []string) Preferences {
	outDict := make(Preferences)

	for _, line := range data {
		parts := strings.Fields(line)
		person1 := parts[0]
		person2 := strings.TrimSuffix(parts[len(parts)-1], ".")
		happiness, _ := strconv.Atoi(parts[3])
		if parts[2] == "lose" {
			happiness = -happiness
		}

		if _, exists := outDict[person1]; !exists {
			outDict[person1] = make(map[string]int)
		}
		outDict[person1][person2] = happiness
	}
	return outDict
}

func parse2(data []string) Preferences {
	preferences := parse(data)
	allGuests := make(map[string]bool)

	for person, prefs := range preferences {
		allGuests[person] = true
		for guest := range prefs {
			allGuests[guest] = true
		}
	}

	preferences["you"] = make(map[string]int)
	for guest := range allGuests {
		preferences["you"][guest] = 0
		if _, exists := preferences[guest]; !exists {
			preferences[guest] = make(map[string]int)
		}
		preferences[guest]["you"] = 0
	}
	return preferences
}

func calculateHappiness(arrangement []string, preferences Preferences) int {
	totalHappiness := 0
	numPeople := len(arrangement)

	for i := 0; i < numPeople; i++ {
		person := arrangement[i]
		leftNeighbor := arrangement[(i-1+numPeople)%numPeople]
		rightNeighbor := arrangement[(i+1)%numPeople]

		totalHappiness += preferences[person][leftNeighbor] + preferences[person][rightNeighbor]
	}
	return totalHappiness
}

func findBestSeating(preferences Preferences) int {
	people := []string{}
	for person := range preferences {
		people = append(people, person)
	}

	permutations := generatePermutations(people)
	maxHappiness := -1 << 31 // Minimum int value

	for _, arrangement := range permutations {
		if happiness := calculateHappiness(arrangement, preferences); happiness > maxHappiness {
			maxHappiness = happiness
		}
	}
	return maxHappiness
}

func generatePermutations(arr []string) [][]string {
	var res [][]string
	var backtrack func(int)

	backtrack = func(start int) {
		if start == len(arr) {
			dst := make([]string, len(arr))
			copy(dst, arr)
			res = append(res, dst)
			return
		}
		for i := start; i < len(arr); i++ {
			arr[start], arr[i] = arr[i], arr[start]
			backtrack(start + 1)
			arr[start], arr[i] = arr[i], arr[start]
		}
	}

	backtrack(0)
	return res
}

func main() {
	file, err := os.Open("../input.txt")
	if err != nil {
		fmt.Println("Failed to read file:", err)
		return
	}
	defer file.Close()

	var data []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		data = append(data, scanner.Text())
	}

	sep := strings.Repeat("=", 20)
	part1 := findBestSeating(parse(data))
	part2 := findBestSeating(parse2(data))

	fmt.Printf("%s Part 1 %s \nMax happiness score: %d\n", sep, sep, part1)
	fmt.Printf("%s Part 2 %s \nMax happiness score: %d\n", sep, sep, part2)
}
