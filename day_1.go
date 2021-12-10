package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
    "fmt"
)

func main() {
	log.Println("DAY 1")
        fmt.Println("PART 1:")
	solve_a()
        fmt.Println("PART 2:")
	solve_b()
}

func solve_a() {
	file, err := os.Open("inputs/input_day1.txt")
	check(err)

	increases := 0

	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)

	old_distance := 0
	new_distance := 0

	for scanner.Scan() {
		new_distance, _ = strconv.Atoi(scanner.Text())
		if new_distance > old_distance {
			increases++
		}
		old_distance = new_distance
	}

	log.Println("increases:", increases-1)

	file.Close()
}

func solve_b() {
	file, err := os.Open("inputs/input_day1.txt")
	check(err)

	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)
	var distances []string

	increases := 0

	for scanner.Scan() {
		distances = append(distances, scanner.Text())
	}

	var_x := 0
	var_y := 0
	var_z := 0

	old_sum := 0
	actual_sum := 0

	for x := 0; x < len(distances)-2; x++ {
		var_x, _ = strconv.Atoi(distances[x])
		var_y, _ = strconv.Atoi(distances[x+1])
		var_z, _ = strconv.Atoi(distances[x+2])

		actual_sum = var_x + var_y + var_z

		if actual_sum > old_sum {
			increases++
		}
		old_sum = actual_sum
	}

	log.Println("INCREASES:", increases-1)

	file.Close()
}

func check(e error) {
	if e != nil {
		log.Fatal("Error:", e)
	}
    
}
