package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
)

func main() {
	log.Println("DAY 1")
	solve_a()
}

func solve_a() {
	file, err := os.Open("input_a_day1.txt")
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

func check(e error) {
	if e != nil {
		log.Fatal("Error:", e)
	}
}
