package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func readInput(filePath string) ([]string, error) {
	file, err := os.Open(filePath)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	var lines []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}
	if err := scanner.Err(); err != nil {
		return nil, err
	}
	return lines, nil
}

func calculateParsedLength(raw string) int {
	length := 0
	for i := 1; i < len(raw)-1; i++ {
		if raw[i] == '\\' {
			if raw[i+1] == '\\' || raw[i+1] == '"' {
				length++
				i++
			} else if raw[i+1] == 'x' && i+3 < len(raw) {
				length++
				i += 3
			}
		} else {
			length++
		}
	}
	return length
}

func calculateEncodedLength(raw string) int {
	length := 2

	for _, c := range raw {
		if c == '\\' || c == '"' {
			length += 2
		} else {
			length++
		}
	}
	return length
}

func calculateRawAndParsed(data []string) (int, int) {
	totalRaw := 0
	totalParsed := 0

	for _, rawString := range data {
		rawLength := len(rawString)
		parsedLength := calculateParsedLength(rawString)

		totalRaw += rawLength
		totalParsed += parsedLength
	}

	return totalRaw, totalParsed
}

func calculateRawAndEncoded(data []string) (int, int) {
	totalRaw := 0
	totalEncoded := 0

	for _, rawString := range data {
		rawLength := len(rawString)
		encodedLength := calculateEncodedLength(rawString)
		totalRaw += rawLength
		totalEncoded += encodedLength
	}
	return totalRaw, totalEncoded
}

func main() {
	data, err := readInput("../input.txt")
	if err != nil {
		log.Fatalf("Error reading data: %v", err)
	}

	raw, parsed := calculateRawAndParsed(data)
	fmt.Println(strings.Repeat("=", 20), "Part 1", strings.Repeat("=", 20))
	fmt.Printf("Total raw length: %d\n", raw)
	fmt.Printf("Total parsed length: %d\n", parsed)
	fmt.Printf("Difference: %d\n", raw-parsed)

	raw, encoded := calculateRawAndEncoded(data)
	fmt.Println(strings.Repeat("=", 20), "Part 2", strings.Repeat("=", 20))
	fmt.Printf("Total raw length: %d\n", raw)
	fmt.Printf("Total parsed length: %d\n", parsed)
	fmt.Printf("Difference: %d\n", encoded-raw)
}
