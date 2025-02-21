package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

type Rule struct {
	From string
	To   string
}

func parse(path string) ([]Rule, string) {
	file, err := os.Open(path)
	if err != nil {
		panic(fmt.Sprintf("Error reading input file: %v", err))
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	var lines []string
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		panic(err)
	}

	splitIndex := 0
	for i, line := range lines {
		if line == "" {
			splitIndex = i
			break
		}
	}

	rulesData := lines[:splitIndex]
	molecule := lines[splitIndex+1]

	var rules []Rule
	for _, line := range rulesData {
		parts := strings.Split(line, " => ")
		if len(parts) == 2 {
			rules = append(rules, Rule{From: parts[0], To: parts[1]})
		}
	}

	return rules, molecule
}

func genReplacements(molecule string, rules []Rule) int {
	molecules := make(map[string]struct{})

	for _, rule := range rules {
		startIndex := 0
		for {
			pos := strings.Index(molecule[startIndex:], rule.From)
			if pos == -1 {
				break
			}
			pos += startIndex
			newMolecule := molecule[:pos] + rule.To + molecule[pos+len(rule.From):]
			molecules[newMolecule] = struct{}{}
			startIndex = pos + 1
		}
	}

	return len(molecules)
}

func genFromE(molecule string, rules []Rule) int {
	steps := 0
	current := molecule

	for current != "e" {
		replaced := false
		for _, rule := range rules {
			pos := strings.Index(current, rule.To)
			if pos != -1 {
				current = current[:pos] + rule.From + current[pos+len(rule.To):]
				steps++
				replaced = true
				break
			}
		}
		if !replaced {
			return -1
		}
	}
	return steps
}

func main() {
	rules, molecule := parse("../input.txt")
	sep := strings.Repeat("=", 20)
	part1 := genReplacements(molecule, rules)
	part2 := genFromE(molecule, rules)

	fmt.Printf("%s Part 1 %s\nTotal number of unique combinations is: %d\n", sep, sep, part1)
	fmt.Printf("%s Part 2 %s\nTotal number of steps required is: %d\n", sep, sep, part2)
}
