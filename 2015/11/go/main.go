package main

import (
	"fmt"
	"os"
	"strings"
)

var forbidden = []rune{'i', 'o', 'l'}

func hasIncreasingStraight(password string) bool {
	chars := []rune(password)
	for i := 0; i < len(chars)-2; i++ {
		if chars[i]+1 == chars[i+1] && chars[i]+2 == chars[i+2] {
			return true
		}
	}
	return false
}

func hasTwoPairs(password string) bool {
	pairs := make(map[rune]struct{})
	chars := []rune(password)
	i := 0
	for i < len(chars)-1 {
		if chars[i] == chars[i+1] {
			pairs[chars[i]] = struct{}{}
			i += 2
		} else {
			i++
		}
		if len(pairs) >= 2 {
			return true
		}
	}
	return false
}

func incrementPassword(password string) string {
	chars := []rune(password)
	i := len(chars) - 1
	for i >= 0 {
		chars[i] = (chars[i]-'a'+1)%26 + 'a'

		if contains(forbidden, chars[i]) {
			chars[i] = (chars[i]-'a'+1)%26 + 'a'
			for j := i + 1; j < len(chars); j++ {
				chars[j] = 'a'
			}
		}

		if chars[i] != 'a' {
			break
		}
		i--
	}
	return string(chars)
}

func generatePassword(password string) string {
	for {
		password = incrementPassword(password)
		if !containsAny(password, forbidden) && hasIncreasingStraight(password) && hasTwoPairs(password) {
			return password
		}
	}
}

func contains(arr []rune, char rune) bool {
	for _, c := range arr {
		if c == char {
			return true
		}
	}
	return false
}

func containsAny(password string, chars []rune) bool {
	for _, c := range password {
		if contains(chars, c) {
			return true
		}
	}
	return false
}

func main() {
	data, err := os.ReadFile("../input.txt")
	if err != nil {
		fmt.Println("Error reading file:", err)
		return
	}
	input := strings.TrimSpace(string(data))
	sep := strings.Repeat("=", 20)

	fmt.Println(sep, "Part 1", sep)
	password := generatePassword(input)
	fmt.Printf("Original: %s, updated: %s\n", input, password)

	fmt.Println(sep, "Part 2", sep)
	newPassword := generatePassword(password)
	fmt.Printf("Original: %s, updated: %s\n", password, newPassword)
}
