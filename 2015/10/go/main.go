package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func solve(data string, nIter int) int {
	text := data

	for i := 0; i < nIter; i++ {
		var outStr strings.Builder
		chars := []rune(text)
		count := 1
		prev := chars[0]

		for j := 1; j < len(chars); j++ {
			if chars[j] == prev {
				count++
			} else {
				outStr.WriteString(strconv.Itoa(count))
				outStr.WriteRune(prev)
				count = 1
			}
			prev = chars[j]
		}
		outStr.WriteString(strconv.Itoa(count))
		outStr.WriteRune(prev)
		text = outStr.String()
	}

	return len(text)
}

func main() {
	data, err := os.ReadFile("../input.txt")
	if err != nil {
		panic("Failed to read file")
	}
	input := strings.TrimSpace(string(data))

	sep := strings.Repeat("=", 20)
	fmt.Println(sep, "Part 1", sep)
	p1 := solve(input, 40)
	fmt.Println(p1)

	fmt.Println(sep, "Part 2", sep)
	p2 := solve(input, 50)
	fmt.Println(p2)
}
