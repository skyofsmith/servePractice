package main

import (
	"fmt"
)

func main() {
	var a int = 10
	fmt.Println("a: ", a);
	a += 1
	fmt.Println("a+=1", a)
	a -= 2
	fmt.Println("a-=2", a)
	a *= 3
	fmt.Println("a*=3", a)
	a /= 2
	fmt.Println("a/=2", a)
	a %= 5
	fmt.Println("a%=5", a)
	a <<= 2
	fmt.Println("a<<=2", a)
	a >>= 2
	fmt.Println("a>>=2", a)
	a &= 1
	fmt.Println("a&=1", a)
	a ^= 1
	fmt.Println("a^=1", a)
	a |= 1
	fmt.Println("a|=1", a)
}