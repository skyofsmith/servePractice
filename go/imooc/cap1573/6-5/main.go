package main

import (
	"fmt"
	"time"
)

func main() {

	for i := 0; i < 3; i++ {
		if i%2 == 1 {
			continue
		}
		fmt.Println("偶数", i)
	}

	for i := 0; i < 3; i++ {
		for j := 0; j < 2; j++ {
			fmt.Println("imooc: ", i, j)
			time.Sleep(time.Second)
		}
	}
	// all 6 times

	for i := 0; i < 3; i++ {
		for j := 0; j < 2; j++ {
			fmt.Println("imooc: ", i, j)
			time.Sleep(time.Second)
			break
		}
	}
	// all 3 times

	fmt.Println("begin")
One:
	fmt.Println("code block one")
	time.Sleep(time.Second)
	goto One
}
