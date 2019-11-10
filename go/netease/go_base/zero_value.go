package main

import ("fmt")

type (
    byte uint8
    rune int32
    文本 string
)


func main () {
    test()
    var (
        aaa = "hello"
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