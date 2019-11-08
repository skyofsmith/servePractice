package main

import ("fmt")

type (
    byte uint8
    rune int32
    文本 string
)


func main () {
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