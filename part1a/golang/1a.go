package main

import "fmt"

func main() {
	elves := Parse()

	max := 0
	for _, cal := range elves {
		if cal > max {
			max = cal
		}
	}

	fmt.Println(max)
}