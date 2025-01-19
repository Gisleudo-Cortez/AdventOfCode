package main

import (
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

func Part1(file *os.File) (int, error) {
	data, err := io.ReadAll(file)
	if err != nil {
		return 0, fmt.Errorf("Error reading file contents: %w", err)
	}

	total := 0
	lines := strings.Split(strings.TrimSpace(string(data)), "\n")

	for _, line := range lines {
		parts := strings.Split(line, "x")
		if len(parts) != 3 {
			return 0, fmt.Errorf("invalid input format: %s", line)
		}

		l, err := strconv.Atoi(parts[0])
		if err != nil {
			return 0, fmt.Errorf("error converting length: %w", err)
		}

		w, err := strconv.Atoi(parts[1])
		if err != nil {
			return 0, fmt.Errorf("error converting width: %w", err)
		}

		h, err := strconv.Atoi(parts[2])
		if err != nil {
			return 0, fmt.Errorf("error converting height: %w", err)
		}

		side1 := l * w
		side2 := w * h
		side3 := h * l

		smallest := min(side1, min(side2, side3))
		area := (2 * side1) + (2 * side2) + (2 * side3) + smallest

		total += area
	}

	return total, nil
}

func min(a, b int) int {
	if a < b {
		return a
	}
	return b
}

func Part2(file *os.File) (int, error) {
	data, err := io.ReadAll(file)
	if err != nil {
		return 0, fmt.Errorf("Error reading file contents: %w", err)
	}

	total := 0
	lines := strings.Split(strings.TrimSpace(string(data)), "\n")

	for _, line := range lines {
		parts := strings.Split(line, "x")
		if len(parts) != 3 {
			return 0, fmt.Errorf("invalid input format: %s", line)
		}

		dimensions := make([]int, 3)
		for i, part := range parts {
			dim, err := strconv.Atoi(part)
			if err != nil {
				return 0, fmt.Errorf("error converting dimension: %w", err)
			}
			dimensions[i] = dim
		}

		sort(dimensions)
		wrap := 2*dimensions[0] + 2*dimensions[1]
		bow := dimensions[0] * dimensions[1] * dimensions[2]
		total += wrap + bow
	}

	return total, nil
}

func sort(nums []int) {
	for i := 0; i < len(nums)-1; i++ {
		for j := i + 1; j < len(nums); j++ {
			if nums[i] > nums[j] {
				nums[i], nums[j] = nums[j], nums[i]
			}
		}
	}
}

func main() {
	file, err := os.Open("../input.txt")
	if err != nil {
		fmt.Println("Error opening the file:", err)
		return
	}
	defer file.Close()

	result1, err := Part1(file)
	if err != nil {
		fmt.Println("Error in Part1:", err)
		return
	}

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

	fmt.Printf("Part 1: %d\n", result1)
	fmt.Printf("Part 2: %d\n", result2)
}
