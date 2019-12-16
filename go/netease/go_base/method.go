package main

import "fmt"

type SA struct {
	Name string
	age int
}
type B struct {
	Name string
}
type TZ int
func main() {
	a := SA{}
	a.Print()
	fmt.Println(a.Name, a.age)

	b := B{}
	b.Print()
	fmt.Println(b.Name)

	var c TZ
	c.Print()
	(*TZ).Print(&c)
	c.Increase(100)
	c.Print()
}
func (a *SA) Print() {
	a.Name = "AA"
	a.age = 1
	fmt.Println("A", a.Name, a.age)
}

func (a B) Print() {
	a.Name = "BB"
	fmt.Println("B", a.Name)
}

func (a *TZ) Print() {
	fmt.Println("TZ", *a)
}

func (a *TZ) Increase(num int) {
	//*a += num
	*a += TZ(num)
}