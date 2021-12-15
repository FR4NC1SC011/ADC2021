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
	solve_b()
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

func solve_b() {
	log.Println("solve Part B")

	readFile, err := os.Open("examples/sample3.txt")
	check(err)

	//	most_common := false 					// true if most common is one; false if is zero

	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)

	diagnostic_report := make([]string, 0)

	for fileScanner.Scan() {
		diagnostic_report = append(diagnostic_report, fileScanner.Text())
	}

	fmt.Println(diagnostic_report)

	calculate_oxygen_generator_rating(diagnostic_report)

	readFile.Close()


}

func calculate_oxygen_generator_rating(diagnostic_report []string) {
	len_seq := len(diagnostic_report[0])

	for i := 0; i < len_seq; i++ {
		fmt.Println(diagnostic_report)
		x := calculate_most_common_bit(diagnostic_report, i)
		fmt.Println("Most Common:", string(x))

		for j, y := range diagnostic_report {
			if y[i] == x {
				remove(diagnostic_report, j)
			}
		}
	}
	fmt.Println(diagnostic_report)
}

func calculate_most_common_bit(seq []string, position int) byte {
	one := 0
	zero := 0

	for _, x := range seq {
		if x[position] == '1' {
			one += 1
		} else {
			zero += 1
		}
	}

	if float64(one) / float64(len(seq)) >= 0.5 {
		return '1'
	} else {
		return '0'
	}
}

func binary_to_int(b string) int64 {
	x, _ := strconv.ParseInt(b, 2, 64)
	return x
}

func remove(s []string, i int) []string {
    s[i] = s[len(s)-1]
    return s[:len(s)-1]
}

func check(e error) {
	if e != nil {
		log.Fatal(e)
	}
}
