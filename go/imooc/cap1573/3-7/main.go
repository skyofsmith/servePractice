package main

import (
    "fmt"
    "unsafe"
)

func main() {
    var i uint8 = 1
    fmt.Println("uint8: ", unsafe.Sizeof(i))
    var j int32 = 1
    fmt.Println("int32: ", unsafe.Sizeof(j))
    var k int = 1
    fmt.Println("int: ", unsafe.Sizeof(k))
}
