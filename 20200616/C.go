package main

import "fmt"

func main() {
	var n int
	fmt.Scan(&n)

	s := make([]string, 0)

	for i := 0; i < n; i++ {
		var tmp string
		fmt.Scan(&tmp)
		s = append(s, tmp)
	}

	m := make(map[string]int)
	for _, v := range s {
		m[v]++
	}

	var max int
	for _, v := range m {
		if max < v {
			max = v
		}
	}

	result := make([]string, 0)
	for k, v := range m {
		if v == max {
			result = append(result, k)
		}

	}

	for _, v := range result {
		fmt.Println(v)
	}
}
