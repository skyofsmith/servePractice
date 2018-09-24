package main

import (
	"fmt"
)

func main() {
	a := byte(0)
	b := byte(1)
	fmt.Println("a: ", a, "b: ", b)
	fmt.Println("a&b", a&b)
	fmt.Println("b&b", b&b)
	fmt.Println("a|b", a|b)
	fmt.Println("a^b", a^b)
	fmt.Println("a^a", a^a)
	fmt.Println("b^b", b^b)
	fmt.Println("b<<1", b<<1)
	fmt.Println("b>>1", b>>1)
}