package main

import (
	"bufio"
	"fmt"
	"os"

	"./pipeline"
)

func main() {
	const filename = "large.in"
	const n = 100000000
	file, err := os.Create(filename)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	p := pipeline.RandomSource(n)

  writer := bufio.NewWriter(file)
	pipeline.WriterSink(writer, p)
  writer.Flush()

	file, err = os.Open(filename)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	p = pipeline.ReaderSource(bufio.NewReader(file), -1)
	count := 0
	for v := range p {
		fmt.Println(v)
		count++
		if count >= 100 {
			break
		}
	}

	// NergeDemo()
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

func MergeDemo() {
	p := pipeline.Merge(
		pipeline.InMemSort(
			pipeline.ArraySourcce(3, 2, 6, 7, 4)),
		pipeline.InMemSort(
			pipeline.ArraySourcce(7, 4, 0, 3, 2, 13, 8)))
	for v := range p {
		fmt.Println(v)
	}
}
