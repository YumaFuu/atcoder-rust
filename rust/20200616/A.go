package main

import (
	"fmt"
)

func main() {
	var a, b, c int
	fmt.Scan(&a, &b, &c)
	s := []int{a, b, c}
	m := make(map[int]int)

	for _, i := range s {
		m[i]++
	}

	if len(m) == 2 {
		fmt.Println("Yes")
	} else {
		fmt.Println("No")
	}

}
