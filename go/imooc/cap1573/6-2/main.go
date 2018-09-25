package main

import (
	"fmt"
)

func main() {
	var a int = 3
	switch a {
	case 1:
		fmt.Println("=1")
	case 2:
		fmt.Println("=2")
	default:
		fmt.Println("neither")
	}

	var b interface{}
	b = 2
	switch b.(type) {
	case int:
		fmt.Println("int")
	case string:
		fmt.Println("string")
	default:
		fmt.Println("neither")
	}
}
