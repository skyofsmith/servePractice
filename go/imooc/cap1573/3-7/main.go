package main

import (
	"reflect"
    "fmt"
    "unsafe"
)

type imooc int64

func main() {

    fmt.Println("----------uintxx----------")
    var i uint8 = 1
    fmt.Println("uint8: ", unsafe.Sizeof(i))
    var i2 uint16 = 1
    fmt.Println("uint16: ", unsafe.Sizeof(i2))
    var i4 uint32 = 1
    fmt.Println("uint32: ", unsafe.Sizeof(i4))
    var i8 uint64 = 1
    fmt.Println("uint64: ", unsafe.Sizeof(i8))

    fmt.Println("----------intxx----------")
    var j int8 = 1
    fmt.Println("int8: ", unsafe.Sizeof(j))
    var j2 int16 = 1
    fmt.Println("int16: ", unsafe.Sizeof(j2))
    var j4 int32 = 1
    fmt.Println("int32: ", unsafe.Sizeof(j4))
    var j8 int64 = 1
    fmt.Println("int64: ", unsafe.Sizeof(j8))

    fmt.Println("----------int----------")
    var k int = 1
    fmt.Println("int: ", unsafe.Sizeof(k))

    fmt.Println("----------floatxx----------")
    var l32 float32 = 1.1
    fmt.Println("float32: ", unsafe.Sizeof(l32))
    var l64 float64 = 1.2
    fmt.Println("float64: ", unsafe.Sizeof(l64))

    fmt.Println("----------complexXX----------")
    var c64 complex64 = 1.1
    fmt.Println("complex64: ", unsafe.Sizeof(c64))
    var c128 complex128 = 1.2
    fmt.Println("complex128: ", unsafe.Sizeof(c128))

    fmt.Println("----------bool----------")
    var m bool = false
    fmt.Println("bool: ", unsafe.Sizeof(m))

    fmt.Println("----------other----------")
    var n1 byte = 1
    fmt.Println("byte: ", unsafe.Sizeof(n1))
    var n2 rune = 1
    fmt.Println("rune: ", unsafe.Sizeof(n2))


    var dft1 int32
    var dft2 float32
    var dft3 bool
    var dft4 complex64
    var dft5 string
    fmt.Println(dft1, dft2, dft3, dft4, dft5)

    var typevar imooc = 1
    fmt.Println("imooc 类型是:", reflect.TypeOf(typevar))
    // var normal int32 = 2
    // fmt.Println("imooc + int32", typevar + normal)  //error, type is not same, can not calculate
    var typevar2 imooc = 2
    fmt.Println("imooc + int32", typevar + typevar2)    //type is same, can calculate
}
