package main

import (
	"bufio"
	"os"
	"strconv"
)

func Parse() map[int]int {
	scanner := bufio.NewScanner(os.Stdin)
	elves := map[int]int{}
	for elfID := 0; scanner.Scan(); {
		line := scanner.Text()

		if line == "" {
			elfID++
			continue
		}

		num, _ := strconv.Atoi(line)
		elves[elfID] += num
	}
	return elves
}