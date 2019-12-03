package main

import (
	"fmt"
	"strconv"
)

type (
	byte uint8
	rune int32
	文本   string
)

func main() {
	var_var()
	test()
	transform()
	testStringConvert()
	testConst()
	testEnum()
	testSymbol()
	testControl()
}

func testStringConvert() {
	var a int = 65
	b := string(a)
	fmt.Println(b) // "A"
	c := strconv.Itoa(a)
	fmt.Println(c) // "65"
	d, _ := strconv.Atoi(c)
	fmt.Println(d)
}
func var_var() {
	var (
		aaa      = "hello"
		sss, bbb = 1, 2
	)
	fmt.Println(aaa)
	fmt.Println(sss)
	fmt.Println(bbb)

	var a, b, c, d int
	a, b, c, d = 1, 2, 3, 4
	fmt.Println(a)
	fmt.Println(b)
	fmt.Println(c)
	fmt.Println(d)

	var e, f, g, h int = 5, 6, 7, 8
	fmt.Println(e)
	fmt.Println(f)
	fmt.Println(g)
	fmt.Println(h)
	var i, j, k, l = 9, 10, 11, 12
	fmt.Println(i)
	fmt.Println(j)
	fmt.Println(k)
	fmt.Println(l)
	m, n, o := 13, 14, 15
	fmt.Println(m)
	fmt.Println(n)
	fmt.Println(o)
}

func test() {
	var a int
	var b float32
	var c bool
	var d string
	var e []int
	var f [1]int
	var g 文本
	g = "这是文本"
	fmt.Println(a)
	fmt.Println(b)
	fmt.Println(c)
	fmt.Println(d)
	fmt.Println(e)
	fmt.Println(f)
	fmt.Println(g)
}

func transform() {
	var a float32 = 100.1
	fmt.Println(a)
	b := int(a)
	//c := bool(a)  // error: cannot convert a (type float32) to type bool
	fmt.Println(b)
}

func testConst() {
	const a int = 1
	const b = 'A'
	const (
		c = 1
	)
	const f, g = 1, 2
	fmt.Println(a)
	fmt.Println(b)
	fmt.Println(c)
	fmt.Println(f)
	fmt.Println(g)
}

func testEnum()  {
	const (
		a = 'A'
		b = iota
		c = 'B'
		d = iota
	)
	const (
		e = iota
	)
	fmt.Println(a)
	fmt.Println(b)
	fmt.Println(c)
	fmt.Println(d)
	fmt.Println(e)
}

func testSymbol() {
	fmt.Println(^2)	// ^2 = -3
	fmt.Println(1^2)	// 1^2 = 3
	fmt.Println(!false)	// !false = true
	fmt.Println(!true)	// !true = false
	/*
	6 : 0110
	10: 1011
	--------
	&   0010
	|   1111
	^   1101
	&^  0100
	 */
	fmt.Println(6 & 11)
	fmt.Println(6 | 11)
	fmt.Println(6 ^ 11)
	fmt.Println(6 &^ 11)
	const (
		B float64 = 1 << (iota * 10)
		KB
		MB
		GB
		TB
	)
	fmt.Println(B)
	fmt.Println(KB)
	fmt.Println(MB)
	fmt.Println(GB)
	fmt.Println(TB)
}

func testControl () {
	a := 100
	var p *int = &a
	fmt.Println(p)
	fmt.Println(*p)
	if a := 1; a > 0 {
		fmt.Println(a)
	}
	fmt.Println(a)
	b := 1
	for {
		b++
		if	b > 3 {
			break
		}
		fmt.Println(b)
	}
	fmt.Println("over")
	for b > 0 {
		b--
		fmt.Println(b)
	}
	fmt.Println("over")
	for c := 0; c < 5; c++ {
		fmt.Println(c)
	}
	fmt.Println("over")
	c := 1
	switch c {
	case 0:
		fmt.Println("a = 0")
	case 1:
		fmt.Println("a = 1")
	default:
		fmt.Println("None")
	}
	switch {
	case a >= 0:
		fmt.Println("a >= 0")
		fallthrough
	case a >= 1:
		fmt.Println("a >= 1")
	default:
		fmt.Println("None")
	}
	switch a := 3; {
	case a >= 0:
		fmt.Println("a >= 0")
		fallthrough
	case a >= 1:
		fmt.Println("a >= 1")
	default:
		fmt.Println("None")
	}
}
