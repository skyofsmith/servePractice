package main

import (
	"fmt"
)

const a = iota
const (
	b = iota
	c = iota
	_
	d
)
const (
	e = iota
	f = 3.14
	g = iota
)

const (
	h = iota * 2
	i = iota
	j = iota
)

const (
	k = iota * 2
	l
	m
)

const (
	n = iota * 2
	o = iota * 3
	p
	q
)
const (
	r, s = iota, iota + 3
	t, u
	v = iota
)
func main() {
	// fmt.Println(iota)	//error
	fmt.Println("a: ", a)
	fmt.Println("b: ", b, "c: ", c, "d: ", d)
	fmt.Println("e: ", e, "f: ", f, "g: ", g)
	fmt.Println("h: ", h, "i: ", i, "j: ", j)
	fmt.Println("k: ", k, "l: ", l, "m: ", m)
	fmt.Println("n: ", n, "o: ", o, "p: ", p, "q: ", q)
	fmt.Println("r: ", r, "s: ", s, "t: ", t, "u: ", u, "v: ", v)
}
