package main

import (
  "./pipeline"
  "fmt"
)

func main() {
  p := pipeline.InMemSort(
    pipeline.ArraySourcce(3, 2, 6, 7, 4))
  for v := range p {
    fmt.Println(v)
  }
  /*
  for {
    if num, ok := <- p; ok {
      fmt.Println(num)
    } else {
      break
    }
  }
  */
}
