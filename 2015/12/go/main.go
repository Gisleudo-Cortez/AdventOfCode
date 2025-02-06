package main

import (
	"encoding/json"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func totalSum1(data string) int {
	re := regexp.MustCompile(`-?\d+`)
	matches := re.FindAllString(data, -1)
	sum := 0
	for _, match := range matches {
		if num, err := strconv.Atoi(match); err == nil {
			sum += num
		}
	}
	return sum
}

func sumNonRed(data interface{}) int {
	switch v := data.(type) {
	case float64:
		return int(v)
	case []interface{}:
		sum := 0
		for _, item := range v {
			sum += sumNonRed(item)
		}
		return sum
	case map[string]interface{}:
		for _, value := range v {
			if str, ok := value.(string); ok && str == "red" {
				return 0
			}
		}
		sum := 0
		for _, value := range v {
			sum += sumNonRed(value)
		}
		return sum
	}
	return 0
}

func totalSum2(data string) int {
	var jsonData interface{}
	if err := json.Unmarshal([]byte(data), &jsonData); err != nil {
		return 0
	}
	return sumNonRed(jsonData)
}

func main() {
	fileContent, err := os.ReadFile("../input.txt")
	if err != nil {
		fmt.Println("Error reading file:", err)
		os.Exit(1)
	}

	data := string(fileContent)
	sep := strings.Repeat("=", 20)
	fmt.Println(sep, "Part 1", sep)
	fmt.Println("Sum:", totalSum1(data))
	fmt.Println(sep, "Part 2", sep)
	fmt.Println("Sum:", totalSum2(data))
}
