package main

import (
	"fmt"
	"reflect"
)

type User struct {
	Id   int
	Name string
	Age  int
}

func (u User) Hello() {
	fmt.Println("hello world!")
}

type Manager struct {
	User
	title string
}

func main() {
	//u := User{1, "OK", 12}
	//Info(u)
	m := Manager{User: User{1, "OK", 12}, title: "qwe"}
	t := reflect.TypeOf(m)
	fmt.Printf("%#v\n", t.FieldByIndex([]int{0, 0}))
}

func Info(o interface{}) {
	t := reflect.TypeOf(o)
	fmt.Println("Type:", t.Name())

	v := reflect.ValueOf(o)
	fmt.Println("Fields:")

	for i := 0; i < t.NumField(); i++ {
		f := t.Field(i)
		val := v.Field(i).Interface()
		fmt.Printf("%6s: %v = %v", f.Name, f.Type, val)
	}

	for i := 0; i < t.NumField(); i++ {
		m := t.Method(i)
		fmt.Printf("%6s: %v\n", m.Name, m.Type)
	}
}
