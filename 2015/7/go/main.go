package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Circuit struct {
	wireInstructions map[string][]string
	wireCache        map[string]uint16
}

func NewCircuit() *Circuit {
	return &Circuit{
		wireInstructions: make(map[string][]string),
		wireCache:        make(map[string]uint16),
	}
}

func (c *Circuit) AddInstruction(instruction string) error {
	parts := strings.Split(instruction, " -> ")
	if len(parts) != 2 {
		return fmt.Errorf("invalid instruction format: %s", instruction)
	}

	targetWire := strings.TrimSpace(parts[1])
	source := strings.Fields(parts[0])
	c.wireInstructions[targetWire] = source
	return nil
}

func (c *Circuit) OverrideWire(wire string, value uint16) {
	c.wireInstructions[wire] = []string{strconv.Itoa(int(value))}
}

func (c *Circuit) ResetCache() {
	c.wireCache = make(map[string]uint16)
}

func (c *Circuit) GetWireValue(wire string) (uint16, error) {
	if val, ok := c.wireCache[wire]; ok {
		return val, nil
	}

	if val, err := strconv.Atoi(wire); err == nil {
		return uint16(val), nil
	}

	instruction, ok := c.wireInstructions[wire]
	if !ok {
		return 0, fmt.Errorf("wire not found: %s", wire)
	}

	var value uint16
	var err error

	switch len(instruction) {
	case 1:
		value, err = c.GetWireValue(instruction[0])

	case 2:
		if instruction[0] != "NOT" {
			return 0, fmt.Errorf("invalid unary operation: %v", instruction)
		}
		rightVal, err := c.GetWireValue(instruction[1])
		if err != nil {
			return 0, err
		}
		value = ^rightVal & 0xFFFF

	case 3:
		left, err := c.GetWireValue(instruction[0])
		if err != nil {
			return 0, err
		}
		right, err := c.GetWireValue(instruction[2])
		if err != nil {
			return 0, err
		}

		switch instruction[1] {
		case "AND":
			value = left & right
		case "OR":
			value = left | right
		case "LSHIFT":
			value = (left << right) & 0xFFFF
		case "RSHIFT":
			value = (left >> right) & 0xFFFF
		default:
			return 0, fmt.Errorf("invalid binary operation: %s", instruction[1])
		}

	default:
		return 0, fmt.Errorf("invalid instruction format: %v", instruction)
	}

	if err != nil {
		return 0, err
	}

	c.wireCache[wire] = value
	return value, nil
}

func solvePart1(instructions []string) (uint16, error) {
	circuit := NewCircuit()
	for _, instruction := range instructions {
		if err := circuit.AddInstruction(instruction); err != nil {
			return 0, err
		}
	}
	return circuit.GetWireValue("a")
}

func solvePart2(instructions []string, part1Result uint16) (uint16, error) {
	circuit := NewCircuit()
	for _, instruction := range instructions {
		if err := circuit.AddInstruction(instruction); err != nil {
			return 0, err
		}
	}
	circuit.OverrideWire("b", part1Result)
	return circuit.GetWireValue("a")
}

func main() {
	file, err := os.Open("../input.txt")
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error opening file: %v\n", err)
		os.Exit(1)
	}
	defer file.Close()

	var instructions []string
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		instructions = append(instructions, scanner.Text())
	}

	part1Result, err := solvePart1(instructions)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Part 1 error: %v\n", err)
		os.Exit(1)
	}
	fmt.Printf("Part 1 - Wire 'a' value: %d\n", part1Result)

	part2Result, err := solvePart2(instructions, part1Result)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Part 2 error: %v\n", err)
		os.Exit(1)
	}
	fmt.Printf("Part 2 - Wire 'a' new value: %d\n", part2Result)
}
