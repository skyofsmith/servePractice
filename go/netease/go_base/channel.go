package main

import (
	"fmt"
	"runtime"
	"sync"
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

	runtime.GOMAXPROCS(runtime.NumCPU())
	d := make(chan bool, 10)
	for i := 0; i < 10; i++ {
		go Go(d, i)
	}
	for i := 0; i < 10; i++ {
		<-d
	}

	wg := sync.WaitGroup{}
	wg.Add(10)
	for i := 0; i < 10; i++ {
		go GoWG(&wg, i)
	}
	wg.Wait()

	c1, c2 := make(chan int), make(chan string)
	o := make(chan bool)
	go func() {
		for {
			select {
			case v, ok := <-c1:
				if !ok {
					o <- true
					break
				}
				fmt.Println("c1", v)
			case v, ok := <-c2:
				if !ok {
					o <- true
					break
				}
				fmt.Println("c2", v)
			}
		}
	}()
	c1 <- 1
	c2 <- "hi"
	c1 <- 2
	c2 <- "hello"

	close(c1)
	close(c2)

	<-o
}

func p() {
	fmt.Println("Go Go Go!!!")
}

func Go(d chan bool, index int) {
	a := 1
	for i := 0; i < 100000000; i++ {
		a += i
	}
	fmt.Println(index, a)
	d <- true
}

func GoWG(wg *sync.WaitGroup, index int) {
	a := 1
	for i := 0; i < 100000000; i++ {
		a += i
	}
	fmt.Println(a, index)
	wg.Done()
}
