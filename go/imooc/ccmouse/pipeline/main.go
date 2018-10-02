package main

import (
  "./pipeline"
  "fmt"
)

func main() {
  p := pipeline.Merge(
    pipeline.InMemSort(
      pipeline.ArraySourcce(3, 2, 6, 7, 4)),
    pipeline.InMemSort(
      pipeline.ArraySourcce(7, 4, 0, 3, 2, 13, 8)))
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
