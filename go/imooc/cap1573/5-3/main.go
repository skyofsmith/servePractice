package main

import (
	"fmt"
)



func main() {
	a := true
	b := true
	c := false
	fmt.Println("a: ", a, "b: ", b, "c: ", c)
	fmt.Println("a && b", a && b)
	fmt.Println("a && c", a && c)
	
	fmt.Println("a || b", a || b)
	fmt.Println("a || c", a || c)
	
	fmt.Println("!a", !a)
}