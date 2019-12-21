package main

import (
	"fmt"
	"time"
)

func main() {
	go p()
	go func() {
		fmt.Println("Go Go Go!!!")
	}()
	time.Sleep(2 * time.Second)
	c := make(chan bool)
	go func() {
		fmt.Println("sync!!!")
		c <- true
		close(c)
	}()
	for v := range c {
		fmt.Println("Hahaha", v)
	}
}

func p()  {
	fmt.Println("Go Go Go!!!")
}
