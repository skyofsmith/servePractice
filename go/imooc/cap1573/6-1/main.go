package main

import (
	"fmt"
)

func main() {
	// if else
	a := 1
	if a > 0 {
		fmt.Println("a > 0")
	} else {
		fmt.Println("a !> 0")
	}
	// 嵌套if
	b := 3
	if b > 0 {
		fmt.Println("b > 0")
		if b < 4 {
			fmt.Println("b < 4")
		}
	}
}
