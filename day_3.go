package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

func main() {
	log.Println("Solve Day 3")
	solve_a()
}

func solve_a() {
	log.Println("Solve Part A")
	gamma := ""
	epsilon := ""

	readFile, err := os.Open("inputs/input_day3.txt")
	check(err)

	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)

	gamma_number := make([]int, 12)
	lines := 0

	for fileScanner.Scan() {
		lines += 1
		sequence := fileScanner.Text()

		for index, b_value := range sequence {
			if b_value == '1' {
				gamma_number[index] += 1
			}
		}
	}

	for _, x := range gamma_number {
		if float64(x) / float64(lines) > 0.5 {
			gamma = gamma + "1"
			epsilon = epsilon + "0"
		} else {
			gamma = gamma + "0"
			epsilon = epsilon + "1"
		}
	}

	gamma_int := binary_to_int(gamma)

	epsilon_int := binary_to_int(epsilon)

	result := gamma_int * epsilon_int
	result_binary := fmt.Sprintf("%b", result)
	fmt.Println("Answer in binary:", result_binary)
	fmt.Println("Answer:", result)

	readFile.Close()

}

func binary_to_int(b string) int64 {
	x, _ := strconv.ParseInt(b, 2, 64)
	return x
}


func check(e error) {
	if e != nil {
		log.Fatal(e)
	}
}
