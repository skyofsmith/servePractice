package main

import (
	"fmt"
)

const imooc string = "imooc"
const name = "zz"
const (
	person = "asd"
	age int8 = 27
)

const apple, banana string = "apple", "banana"
const a1, a2 = 1, "asddd"
const lena2 = len(a2)

func ab(b string) string {
	return ""
}

// const lena2self = ab(a2)	//只能使用内置函数

func main() {
	fmt.Println(imooc)
	fmt.Println(name)
	fmt.Println(person)
	fmt.Println(age)
	fmt.Println(apple, banana)
	fmt.Println(a1, a2, lena2)
}