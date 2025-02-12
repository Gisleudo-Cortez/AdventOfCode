package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Ingredient struct {
	name       string
	capacity   int
	durability int
	flavor     int
	texture    int
	calories   int
}

func parse(data string) Ingredient {
	binding := strings.ReplaceAll(data, ",", "")
	parts := strings.Fields(binding)

	name := strings.TrimSuffix(parts[0], ":")

	capacity, err := strconv.Atoi(parts[2])
	if err != nil {
		panic("Error parsing capacity")
	}

	durability, err := strconv.Atoi(parts[4])
	if err != nil {
		panic("Error parsing durability")
	}

	flavor, err := strconv.Atoi(parts[6])
	if err != nil {
		panic("Error parsing flavor")
	}

	texture, err := strconv.Atoi(parts[8])
	if err != nil {
		panic("Error parsing texture")
	}

	calories, err := strconv.Atoi(parts[10])
	if err != nil {
		panic("Error parsing calories")
	}

	return Ingredient{
		name:       name,
		capacity:   capacity,
		durability: durability,
		flavor:     flavor,
		texture:    texture,
		calories:   calories,
	}
}

func calculateScore(amounts []int, ingredients []Ingredient, target_calories *int) int {
	capacity, durability, flavor, texture, calories := 0, 0, 0, 0, 0

	for i, amt := range amounts {
		capacity += amt * ingredients[i].capacity
		durability += amt * ingredients[i].durability
		flavor += amt * ingredients[i].flavor
		texture += amt * ingredients[i].texture
		calories += amt * ingredients[i].calories
	}

	if target_calories != nil && calories != *target_calories {
		return 0
	}
	if capacity < 0 {
		capacity = 0
	}
	if durability < 0 {
		durability = 0
	}
	if flavor < 0 {
		flavor = 0
	}
	if texture < 0 {
		texture = 0
	}

	return capacity * durability * flavor * texture
}

func findBestCombination(index int, remainingTeaspoons int, currentAmounts []int, ingredients []Ingredient, targetCalories *int, maxScore *int, bestAmounts *map[string]int) {
	if index == len(ingredients)-1 {
		currentAmounts[index] = remainingTeaspoons
		score := calculateScore(currentAmounts, ingredients, targetCalories)
		if score > *maxScore {
			*maxScore = score
			newMap := make(map[string]int)
			for i, ing := range ingredients {
				newMap[ing.name] = currentAmounts[i]
			}
			*bestAmounts = newMap
		}
		return
	}

	for teaspoons := 0; teaspoons <= remainingTeaspoons; teaspoons++ {
		currentAmounts[index] = teaspoons
		findBestCombination(index+1, remainingTeaspoons-teaspoons, currentAmounts, ingredients, targetCalories, maxScore, bestAmounts)
	}
}

func calculateMaxScore(ingredients []Ingredient, nSpoons int, targetCalories *int) (int, map[string]int) {
	maxScore := 0
	bestAmounts := make(map[string]int)
	currentAmounts := make([]int, len(ingredients))
	findBestCombination(0, nSpoons, currentAmounts, ingredients, targetCalories, &maxScore, &bestAmounts)
	return maxScore, bestAmounts
}

func main() {
	data, err := os.ReadFile("../input.txt")
	if err != nil {
		panic("Error reading input file")
	}
	lines := strings.Split(strings.TrimSpace(string(data)), "\n")

	var ingredients []Ingredient
	for _, line := range lines {
		if strings.TrimSpace(line) == "" {
			continue
		}
		ing := parse(line)
		ingredients = append(ingredients, ing)
	}

	sep := strings.Repeat("=", 20)

	maxScore1, bestAmounts1 := calculateMaxScore(ingredients, 100, nil)
	fmt.Printf("%s Part 1 %s\nmax_score: %d\nbest_amounts: %v\n", sep, sep, maxScore1, bestAmounts1)

	calories := 500
	maxScore2, bestAmounts2 := calculateMaxScore(ingredients, 100, &calories)
	fmt.Printf("%s Part 2 %s\nmax_score: %d\nbest_amounts: %v\n", sep, sep, maxScore2, bestAmounts2)
}
