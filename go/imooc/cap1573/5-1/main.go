package main

import (
	"fmt"
)


func main() {
	a := 2
	b := 3
	fmt.Println("a+b: ", a + b)
	fmt.Println("a-b: ", a - b)
	fmt.Println("a*b: ", a * b)
	fmt.Println("a/b: ", a / b)
	a++
	fmt.Println("a++: ", a)
	// ++a	//error
	b--
	fmt.Println("b--: ", b)
	// --b	//error
}