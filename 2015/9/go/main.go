package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

type Node struct {
	connections map[string]int
}

func NewNode() *Node {
	return &Node{connections: make(map[string]int)}
}

func (n *Node) AddNeighbor(destination string, cost int) {
	n.connections[destination] = cost
}

func (n *Node) GetDistance(destination string) (int, bool) {
	dist, exists := n.connections[destination]
	return dist, exists
}

func ParseInput(path string) map[string]*Node {
	nodes := make(map[string]*Node)
	file, err := os.Open(path)
	if err != nil {
		panic("Unable to read file")
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		parts := strings.Fields(scanner.Text())
		origin, destination := parts[0], parts[2]
		cost, _ := strconv.Atoi(parts[4])

		if _, exists := nodes[origin]; !exists {
			nodes[origin] = NewNode()
		}
		nodes[origin].AddNeighbor(destination, cost)

		if _, exists := nodes[destination]; !exists {
			nodes[destination] = NewNode()
		}
		nodes[destination].AddNeighbor(origin, cost)
	}

	return nodes
}

func Permutations(arr []string) [][]string {
	var helper func([]string, int)
	res := [][]string{}

	helper = func(arr []string, n int) {
		if n == 1 {
			tmp := make([]string, len(arr))
			copy(tmp, arr)
			res = append(res, tmp)
		} else {
			for i := 0; i < n; i++ {
				helper(arr, n-1)
				if n%2 == 1 {
					arr[0], arr[n-1] = arr[n-1], arr[0]
				} else {
					arr[i], arr[n-1] = arr[n-1], arr[i]
				}
			}
		}
	}

	helper(arr, len(arr))
	return res
}

func FindShortestPath(nodes map[string]*Node) ([]string, int) {
	cities := []string{}
	for city := range nodes {
		cities = append(cities, city)
	}

	shortestDistance := math.MaxInt32
	var bestRoute []string

	for _, route := range Permutations(cities) {
		currentDistance := 0
		valid := true

		for i := 0; i < len(route)-1; i++ {
			city, nextCity := route[i], route[i+1]
			if node, exists := nodes[city]; exists {
				if dist, exists := node.GetDistance(nextCity); exists {
					currentDistance += dist
				} else {
					valid = false
					break
				}
			}
		}

		if valid && currentDistance < shortestDistance {
			shortestDistance = currentDistance
			bestRoute = route
		}
	}

	return bestRoute, shortestDistance
}

func FindLongestPath(nodes map[string]*Node) ([]string, int) {
	cities := []string{}
	for city := range nodes {
		cities = append(cities, city)
	}

	longestDistance := math.MinInt32
	var bestRoute []string

	for _, route := range Permutations(cities) {
		currentDistance := 0
		valid := true

		for i := 0; i < len(route)-1; i++ {
			city, nextCity := route[i], route[i+1]
			if node, exists := nodes[city]; exists {
				if dist, exists := node.GetDistance(nextCity); exists {
					currentDistance += dist
				} else {
					valid = false
					break
				}
			}
		}

		if valid && currentDistance > longestDistance {
			longestDistance = currentDistance
			bestRoute = route
		}
	}

	return bestRoute, longestDistance
}

func main() {
	nodes := ParseInput("../input.txt")
	route, distance := FindShortestPath(nodes)
	fmt.Println("================= Part 1 =================")
	fmt.Printf("The shortest route in the data is %v with a distance of %d\n", strings.Join(route, " -> "), distance)

	route, distance = FindLongestPath(nodes)
	fmt.Println("================= Part 2 =================")
	fmt.Printf("The longest route in the data is %v with a distance of %d\n", strings.Join(route, " -> "), distance)
}
