package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type Reindeer struct {
	Name  string
	Speed int
	Time  int
	Rest  int
}

func parseLine(line string) Reindeer {
	parts := strings.Fields(line)
	if len(parts) < 8 {
		log.Fatalf("invalid input line: %s", line)
	}

	speed, err := strconv.Atoi(parts[3])
	if err != nil {
		log.Fatalf("error parsing speed %q: %v", parts[3], err)
	}

	timeVal, err := strconv.Atoi(parts[6])
	if err != nil {
		log.Fatalf("error parsing time %q: %v", parts[6], err)
	}

	rest, err := strconv.Atoi(parts[len(parts)-2])
	if err != nil {
		log.Fatalf("error parsing rest time %q: %v", parts[len(parts)-2], err)
	}

	return Reindeer{
		Name:  parts[0],
		Speed: speed,
		Time:  timeVal,
		Rest:  rest,
	}
}

func calculateDistance(r Reindeer, seconds int) int {
	cycleTime := r.Time + r.Rest
	fullCycles := seconds / cycleTime
	remainingTime := seconds % cycleTime
	if remainingTime > r.Time {
		remainingTime = r.Time
	}
	return fullCycles*(r.Speed*r.Time) + r.Speed*remainingTime
}

func getFastest(lines []string, seconds int) string {
	bestName := ""
	bestDistance := 0
	for _, line := range lines {
		r := parseLine(line)
		d := calculateDistance(r, seconds)
		if d > bestDistance {
			bestDistance = d
			bestName = r.Name
		}
	}
	return fmt.Sprintf("%s: %d", bestName, bestDistance)
}

func calculatePoints(lines []string, seconds int) map[string]int {
	reindeers := make([]Reindeer, len(lines))
	points := make(map[string]int)
	for i, line := range lines {
		r := parseLine(line)
		reindeers[i] = r
		points[r.Name] = 0
	}
	for second := 1; second <= seconds; second++ {
		maxDistance := 0
		distances := make(map[string]int)
		for _, r := range reindeers {
			d := calculateDistance(r, second)
			distances[r.Name] = d
			if d > maxDistance {
				maxDistance = d
			}
		}
		for name, d := range distances {
			if d == maxDistance {
				points[name]++
			}
		}
	}
	return points
}

func main() {
	data, err := os.ReadFile("../input.txt")
	if err != nil {
		log.Fatalf("Error reading input file: %v", err)
	}
	lines := strings.Split(strings.TrimSpace(string(data)), "\n")
	sep := strings.Repeat("=", 20)
	fastestReindeer := getFastest(lines, 2503)
	fmt.Printf("%s Part 1 %s\nThe fastest reindeer is %s\n", sep, sep, fastestReindeer)
	points := calculatePoints(lines, 2503)
	winnerName := ""
	winnerPoints := 0
	for name, pts := range points {
		if pts > winnerPoints {
			winnerPoints = pts
			winnerName = name
		}
	}
	if winnerName != "" {
		fmt.Printf("%s Part 2 %s\nThe Winner is %s with %d points\n", sep, sep, winnerName, winnerPoints)
	} else {
		fmt.Printf("%s Part 2 %s\nNo winner found\n", sep, sep)
	}
}
