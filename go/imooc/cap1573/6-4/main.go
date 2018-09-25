package main

import (
	"fmt"
	"time"
)

func main() {
	// 有条件的for
	for i := 1; i < 10; i++ {
		fmt.Println(i)
	}

	//遍历数组
	var list = []string{"apple", "banana", "orange"}
	for key, value := range list {
		fmt.Println("key: ", key)
		fmt.Println("value: ", value)
	}

	for {
		fmt.Println("imooc")
		time.Sleep(time.Second)
	}
	// for {}  is like for true {}

}
