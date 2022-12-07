package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	f, _ := os.Open("input.txt")
	defer f.Close()
	sc := bufio.NewScanner(f)
	score_map := map[string]int{
		"B X": 1,
		"C Y": 2,
		"A Z": 3,
		"A X": 4,
		"B Y": 5,
		"C Z": 6,
		"C X": 7,
		"A Y": 8,
		"B Z": 9,
	}
	var score int
	for sc.Scan() {
		score += score_map[sc.Text()]
	}
	fmt.Println(score)
}
