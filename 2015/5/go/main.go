package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func countVowels(s string) int {
	count := 0
	for _, c := range s {
		if strings.ContainsRune("aeiou", c) {
			count++
		}
	}

	return count
}

func hasDoubleLetter(s string) bool {
	l := len(s)
	r := []rune(s)
	for i := range l - 1 {
		i = i + 1
		if r[i] == r[i-1] {
			return true
		}

	}
	return false
}

func hasForbidenStrings(s string) bool {
	r := []rune(s)
	l := len(r)
	for i := 0; i < l-1; i++ {
		if r[i] == 'a' && r[i+1] == 'b' ||
			r[i] == 'c' && r[i+1] == 'd' ||
			r[i] == 'p' && r[i+1] == 'q' ||
			r[i] == 'x' && r[i+1] == 'y' {
			return false // fails the criteria
		}
	}

	return true
}

func isNiceStringPart1(s string) bool {
	if countVowels(s) >= 3 && hasDoubleLetter(s) && hasForbidenStrings(s) {
		return true
	}
	return false
}

func hasRepeatingPair(s string) bool {
	r := []rune(s)
	l := len(r)
	for i := 0; i < l-1; i++ {
		pair := string(r[i : i+2])
		if strings.Contains(string(r[i+2:]), pair) {
			return true
		}
	}
	return false
}

func hasRepeatingWithOneBetween(s string) bool {
	r := []rune(s)
	l := len(r)
	for i := 0; i < l-2; i++ {
		if r[i] == r[i+2] {
			return true
		}
	}
	return false
}

func isNiceStringPart2(s string) bool {
	if hasRepeatingPair(s) && hasRepeatingWithOneBetween(s) {
		return true
	}
	return false
}

func part1(ls []string) int {
	total := 0
	for _, s := range ls {
		if isNiceStringPart1(s) {
			total++
		}
	}
	return total
}

func part2(ls []string) int {
	total := 0
	for _, s := range ls {
		if isNiceStringPart2(s) {
			total++
		}
	}
	return total
}

func readFile(path string) []string {
	file, err := os.Open(path)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, strings.TrimSpace(scanner.Text()))
	}

	if err := scanner.Err(); err != nil {
		panic(err)
	}

	return lines
}

func main() {
	lines := readFile("../input.txt")
	fmt.Println("==================== Part1 ====================")
	fmt.Printf("Total nice strings: %d\n", part1(lines))
	fmt.Println("==================== Part2 ====================")
	fmt.Printf("Total nice strings: %d\n", part2(lines))
}
