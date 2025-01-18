package main

import (
	"fmt"
	"io"
	"os"
)

func Part1(file *os.File) (int, error) {
	data, err := io.ReadAll(file)
	if err != nil {
		return 0, fmt.Errorf("error reading file contents: %w", err)
	}

	counter := 0
	for _, c := range string(data) {
		switch c {
		case '(':
			counter++
		case ')':
			counter--
		}
	}

	return counter, nil
}

func Part2(file *os.File) (int, error) {
	data, err := io.ReadAll(file)
	if err != nil {
		return 0, fmt.Errorf("error reading file contents: %w", err)
	}

	counter := 0
	for i, c := range string(data) {
		switch c {
		case '(':
			counter++
		case ')':
			counter--
		}
		if counter < 0 {
			return i + 1, nil
		}
	}

	return 0, fmt.Errorf("no position where counter goes negative")
}

func main() {
	file, err := os.Open("../input.txt")
	if err != nil {
		fmt.Println("Error opening the file:", err)
		return
	}
	defer file.Close()

	_, err = file.Seek(0, 0)
	if err != nil {
		fmt.Println("Error resetting file pointer:", err)
		return
	}
	result1, err := Part1(file)
	if err != nil {
		fmt.Println("Error in Part1:", err)
		return
	}
	fmt.Printf("Part 1: %d\n", result1)

	_, err = file.Seek(0, 0)
	if err != nil {
		fmt.Println("Error resetting file pointer:", err)
		return
	}
	result2, err := Part2(file)
	if err != nil {
		fmt.Println("Error in Part2:", err)
		return
	}
	fmt.Printf("Part 2: %d\n", result2)
}
