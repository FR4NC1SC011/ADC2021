package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	log.Println("Solve Day 2")
	//solve_a()
	solve_b()
}

func solve_a() {
	log.Println("Solve Part A")
	x := 0
	y := 0

	readFile, err := os.Open("inputs/input_day2.txt")
	check(err)

	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)

	for fileScanner.Scan() {
		z := strings.Split(fileScanner.Text(), " ")
		w, _ := strconv.Atoi(z[1])

		if z[0] == "forward" {
			x += w
		} else if z[0] == "down" {
			y += w
		} else {
			y -= w
		}
	}

	fmt.Printf("X: %d, Y: %d\n", x, y)
	fmt.Println("Result:", x*y)

	readFile.Close()

}


func solve_b() {
	log.Println("Solve Part B")

	horizontal := 0
	depth := 0
	aim := 0

	readFile, err := os.Open("inputs/input_day2.txt")
	check(err)

	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)

	for fileScanner.Scan() {
		z := strings.Split(fileScanner.Text(), " ")
		w, _ := strconv.Atoi(z[1])

		if z[0] == "forward" {
			horizontal += w
			depth += aim * w
		} else if z[0] == "down" {
			aim += w
		} else {
			aim -= w
		}
	}

	fmt.Printf("X: %d, Y: %d\n", horizontal, depth)
	fmt.Println("Result:", horizontal*depth)

	readFile.Close()


}


func check(e error) {
	if e != nil {
		log.Fatal("Error:", e)
	}
}
