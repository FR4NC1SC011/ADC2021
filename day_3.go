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

	readFile, err := os.Open("inputs/input_day3.txt")
	check(err)

	//	most_common := false 					// true if most common is one; false if is zero

	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)

	diagnostic_report := make([]string, 0)

	for fileScanner.Scan() {
		diagnostic_report = append(diagnostic_report, fileScanner.Text())
	}



	oxygen := binary_to_int(calculate_oxygen_generator_rating(diagnostic_report))
	co2 := binary_to_int(calculate_co2_scrubber_rating(diagnostic_report))


	fmt.Println("OXYGEN:", oxygen)
	fmt.Println("CO2:", co2)

	fmt.Println("ANSWER:", oxygen * co2)

	readFile.Close()


}

func calculate_oxygen_generator_rating(diagnostic_report []string) string {
	len_seq := len(diagnostic_report[0])

	for i := 0; i < len_seq; i++ {
		x := calculate_most_common_bit(diagnostic_report, i)

		for _, y := range diagnostic_report {
			if y[i] != x {
				//fmt.Println("Remove:", diagnostic_report[j])
				diagnostic_report = remove(diagnostic_report, y)
			}
		}
	}
	return diagnostic_report[0]
}

func calculate_co2_scrubber_rating(diagnostic_report []string) string {
	len_seq := len(diagnostic_report[0])

	for i := 0; i < len_seq; i++ {
		x := calculate_less_common_bit(diagnostic_report, i)

		for _, y := range diagnostic_report {
			if y[i] != x {
				//fmt.Println("Remove:", diagnostic_report[j])
				diagnostic_report = remove(diagnostic_report, y)
			}
		}
	}
	return diagnostic_report[0]

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

func calculate_less_common_bit(seq []string, position int) byte {
	one := 0
	zero := 0

	for _, x := range seq {
		if x[position] == '1' {
			one += 1
		} else {
			zero += 1
		}
	}

	if float64(zero) / float64(len(seq)) <= 0.5 {
		return '0'
	} else {
		return '1'
	}
}


func binary_to_int(b string) int64 {
	x, _ := strconv.ParseInt(b, 2, 64)
	return x
}

func remove(s []string, i string) []string {
    //s[i] = s[len(s)-1]
    //return s[:len(s)-1]
	new_slice := make([]string, 0)

	for _, x := range s {
		if len(s) == 1 {
			return s
		}

		if x != i {
			new_slice = append(new_slice, x)
		}
	}

	return new_slice
}

func check(e error) {
	if e != nil {
		log.Fatal(e)
	}
}
