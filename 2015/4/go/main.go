package main

import (
	"crypto/md5"
	"encoding/hex"
	"fmt"
	"os"
	"strings"
)

func bruteForceCheck(data string, nZeroes int) int {
	answer := 0
	baseData := []byte(data)
	for {
		testString := append(baseData, []byte(fmt.Sprintf("%d", answer))...)

		hash := md5.Sum(testString)
		md5Hash := hex.EncodeToString(hash[:])

		if strings.HasPrefix(md5Hash, strings.Repeat("0", nZeroes)) {
			return answer
		}
		answer++
	}
}

func main() {
	fileContent, err := os.ReadFile("../input.txt")
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error reading file: %v\n", err)
		os.Exit(1)
	}

	data := strings.TrimSpace(string(fileContent))

	fmt.Println("part 1:", bruteForceCheck(data, 5))
	fmt.Println("part 2:", bruteForceCheck(data, 6))
}
