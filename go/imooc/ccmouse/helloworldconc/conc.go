package main

import (
  "fmt"
  // "time"
)

func main() {
  ch := make(chan string)
  for i := 0; i < 5000; i++ {
    go printHelloWorld(i, ch)
  }

  for {
    msg := <- ch
    fmt.Println(msg)
  }
  // time.Sleep(time.Millisecond)
}

func printHelloWorld(i int, ch chan string)  {
  for {
    ch <- fmt.Sprintf("Hello world from gorountine %d!\n",
      i)
  }
}
