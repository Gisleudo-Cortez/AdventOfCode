package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Sue struct {
	number     int
	properties map[string]int
}

type GiftAnalyzer struct {
	sues []Sue
}

var MFCSAM_READING = map[string]int{
	"children":    3,
	"cats":        7,
	"samoyeds":    2,
	"pomeranians": 3,
	"akitas":      0,
	"vizslas":     0,
	"goldfish":    5,
	"trees":       3,
	"cars":        2,
	"perfumes":    1,
}

var GREATER_THAN_PROPS = []string{"cats", "trees"}

var LESS_THAN_PROPS = []string{"pomeranians", "goldfish"}

func NewGiftAnalyzer(path string) *GiftAnalyzer {
	file, err := os.Open(path)
	if err != nil {
		log.Fatalf("Error reading input file: %v", err)
	}
	defer file.Close()

	var sues []Sue
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if line == "" {
			continue
		}

		fields := strings.Fields(line)
		if len(fields) < 8 {
			fmt.Fprintf(os.Stderr, "Skipping malformed line: %s\n", line)
			continue
		}

		numStr := strings.TrimSuffix(fields[1], ":")
		number, err := strconv.Atoi(numStr)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error parsing sue number in line: %s\n", line)
			continue
		}

		properties := make(map[string]int)

		key1 := strings.TrimSuffix(fields[2], ":")
		valStr1 := strings.TrimSuffix(fields[3], ",")
		val1, err := strconv.Atoi(valStr1)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error parsing property value in line: %s\n", line)
			continue
		}
		properties[key1] = val1

		key2 := strings.TrimSuffix(fields[4], ":")
		valStr2 := strings.TrimSuffix(fields[5], ",")
		val2, err := strconv.Atoi(valStr2)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error parsing property value in line: %s\n", line)
			continue
		}
		properties[key2] = val2

		key3 := strings.TrimSuffix(fields[6], ":")
		valStr3 := strings.TrimSuffix(fields[7], ",")
		val3, err := strconv.Atoi(valStr3)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error parsing property value in line: %s\n", line)
			continue
		}
		properties[key3] = val3

		sue := Sue{
			number:     number,
			properties: properties,
		}
		sues = append(sues, sue)
	}

	if err := scanner.Err(); err != nil {
		log.Fatalf("Error reading file: %v", err)
	}

	return &GiftAnalyzer{sues: sues}
}

func contains(slice []string, item string) bool {
	for _, s := range slice {
		if s == item {
			return true
		}
	}
	return false
}

func (ga *GiftAnalyzer) isExactMatch(sue *Sue) bool {
	for prop, value := range sue.properties {
		if expected, ok := MFCSAM_READING[prop]; ok {
			if value != expected {
				return false
			}
		}
	}
	return true
}

func (ga *GiftAnalyzer) isRangeMatch(sue *Sue) bool {
	for prop, value := range sue.properties {
		expected, ok := MFCSAM_READING[prop]
		if !ok {
			continue
		}
		if contains(GREATER_THAN_PROPS, prop) {
			if value <= expected {
				return false
			}
		} else if contains(LESS_THAN_PROPS, prop) {
			if value >= expected {
				return false
			}

		} else {

			if value != expected {
				return false
			}
		}
	}
	return true
}

func (ga *GiftAnalyzer) findExactMatch() (int, bool) {
	for _, sue := range ga.sues {
		if ga.isExactMatch(&sue) {
			return sue.number, true
		}
	}
	return -1, false
}

func (ga *GiftAnalyzer) findRangeMatch() (int, bool) {
	for _, sue := range ga.sues {
		if ga.isRangeMatch(&sue) {
			return sue.number, true
		}
	}
	return -1, false
}

func main() {
	analyzer := NewGiftAnalyzer("../input.txt")
	sep := strings.Repeat("=", 20)

	if exactMatch, found := analyzer.findExactMatch(); found {
		fmt.Printf("%s Part 1 %s\nThe aunt sue that got you the gift is %d\n", sep, sep, exactMatch)
	} else {
		fmt.Println("No exact match found for Part 1")
	}

	if rangeMatch, found := analyzer.findRangeMatch(); found {
		fmt.Printf("%s Part 2 %s\nThe aunt sue that got you the gift is %d\n", sep, sep, rangeMatch)
	} else {
		fmt.Println("No range match found for Part 2")
	}
}
