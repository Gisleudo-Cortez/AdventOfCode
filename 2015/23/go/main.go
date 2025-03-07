package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func hlf(r int) int {
	return r / 2
}

func tpl(r int) int {
	return r * 3
}

func inc(r int) int {
	return r + 1
}

func jmp(v int) int {
	return v
}

func jie(r, v int) int {
	if r%2 == 0 {
		return v
	}
	return 1
}

func jio(r, v int) int {
	if r == 1 {
		return v
	}
	return 1
}

func execute(data []string, reg map[string]int) (int, int) {
	cmds := map[string]func(int, int) int{
		"hlf": func(r, _ int) int { return hlf(r) },
		"tpl": func(r, _ int) int { return tpl(r) }, // Fixed typo: "tlp" -> "tpl"
		"inc": func(r, _ int) int { return inc(r) },
		"jmp": func(_, v int) int { return jmp(v) },
		"jie": jie,
		"jio": jio,
	}
	lenData := len(data)
	i := 0
	for i >= 0 && i < lenData {
		instructions := strings.Fields(data[i])
		cmd := instructions[0]
		switch cmd {
		case "hlf", "tpl", "inc": // Fixed typo: "tlp" -> "tpl"
			r := instructions[1]
			reg[r] = cmds[cmd](reg[r], 0)
			i++
		case "jmp":
			v, _ := strconv.Atoi(instructions[1])
			i += cmds[cmd](0, v)
		case "jie", "jio":
			r := strings.TrimSuffix(instructions[1], ",")
			v, _ := strconv.Atoi(instructions[2])
			i += cmds[cmd](reg[r], v)
		default:
			return reg["a"], reg["b"]
		}
	}
	return reg["a"], reg["b"]
}

func main() {
	data, err := os.ReadFile("../input.txt")
	if err != nil {
		fmt.Println("Error reading input file", err)
		return
	}
	rawLines := strings.Split(strings.TrimSpace(string(data)), "\n")
	sep := strings.Repeat("=", 20)
	fmt.Printf("%s Part 1 %s\n", sep, sep)
	reg := map[string]int{"a": 0, "b": 0}
	a, b := execute(rawLines, reg)
	fmt.Printf("Register a:%d, b:%d\n", a, b)
	fmt.Printf("%s Part 2 %s\n", sep, sep)
	reg = map[string]int{"a": 1, "b": 0}
	a, b = execute(rawLines, reg)
	fmt.Printf("Register a:%d, b:%d\n", a, b)
}
