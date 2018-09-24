package main

import (
	"reflect"
	"fmt"
	"./B"
)
var a int
var b int = 123
var (
	c string = "imooc"
	d bool = false
)
func main() {
	var a int
	var b int = 123
	var (
		c string = "imooc"
		d bool = false
	)
	var m,n,o int = 1,2,3
	var j,k,l = 11,22,33.3
	fmt.Println(a,b,c,d)
	fmt.Println(m,n,o)
	fmt.Println(j,k,l)
	fmt.Println(reflect.TypeOf(j), reflect.TypeOf(k), reflect.TypeOf(l))
	// x,y,z = 1,2,3	//error
	x,y,z := 1,2,3	//ok, can not global
	fmt.Println(x,y,z)
	var q,_,e = 1,2,3	//begin with '_' with droped
	fmt.Println(q,"",e)

	var avalue int = 3
	var bvalue float32 = 3.01
	cvalue := float32(avalue)
	dvalue := int32(bvalue)
	fmt.Println(cvalue, reflect.TypeOf(cvalue))
	fmt.Println(dvalue, reflect.TypeOf(dvalue))
	// var boolvalue = false
	// var intvalue := int(boolvalue)	//can not transform
	fmt.Println(B.B)
	// fmt.Println(B.b)	//can not read B.b
}