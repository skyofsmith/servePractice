## Go基本类型

- 布尔类型: boolean
    - 长度: 1字节
    - 取值: true， false
    - 注意事项: 不能用数字代表true或false

- 整型: int/uint
    - 根据运行平台可能为32位或64位
    
- 8位整型: int8/uint8
    - 长度: 1字节
    - 取值范围: -128～127/0~255
    
- 字节型: byte（uint8别名）

- 16位整型: int16/uint16
    - 长度: 2字节
    - 取值范围: -32768～32767/0~65535
    
- 32位整型: int32(rune)/uint32
    - 长度: 4字节
    - 取值范围: -2^16～2^16-1/0~2^32-1
    
- 64位整型: int64/uint64
    - 长度: 4字节
    - 取值范围: -2^64～2^64-1/0~2^64-1

- 浮点型: float32/float64
    - 长度: 4/8字节
    - 小数位: 精确到7/15小数位
    
    
- 复数: complex64/complex128
    - 长度: 8/16字节
    
- 足够保存指针的32位或64位整数型: uintptr

- 其他值类型:
    - array
    - struct
    - string
    
- 接口类型: interface

- 函数类型: func

## 类型零值

值类型默认为0，bool为false，string为空字符串


## 类型别名
```
type (
    别名  实际类型
)
```

## 单个变量的声明与赋值
- 变量的声明格式: var <变量名称> <变量类型>
- 变量的赋值格式: <变量名称> = <表达式>
- 声明的同时赋值: var <变量名称> <变量类型> = <表达式> / <变量名称> := <表达式>

## 多个变量的声明与赋值
- 全局变量的声明可使用var()的方式进行简写
- 全局变量的声明不可以省略var，但可使用并行方式
- 所有变量都可以使用类型推断
- 局部变量不可以使用var()的方式简写，只能使用并行方式
```
var (
    aaa = "hello"
    sss, bbb = 1, 2
)

var a, b, c, d int
a, b, c, d = 1, 2, 3, 4

var e, f, g, h int = 5, 6, 7, 8
var i, j, k, l = 9, 10, 11, 12
i, m, n, o := 13, 14, 15, 16
```

## 变量的类型转换

- Go中不存在隐式转换， 所有类型转换必须是显式声明
- 转换只能发生在两种互相兼容的类型之间
- 类型转换格式：
    - `<ValueA> [:]= <TypeOfValueA>(<ValueB>)`

```
// 当前程序的包名
package main

// 导入其它的包
import std "fmt"

// 常量的定义
const PI = 3.14

// 全局变量的声明与赋值
var name = "gopher"

// 一般类型申明
type newType int

// 结构的申明
type gopher struct{}

// 接口的申明
type golang interface{}

// 由 main 函数作为程序入口点启动
func main() {
  std.Println("Hello world! 你好，世界！")
}
```
## 常量的定义

- 常量的值在编译时就已经确定
- 常量的定义格式与变量基本相同
- 等号右侧必须是常量或者常量表达式
- 常量表达式中的函数必须是内置函数

## 运算符

- Go中的运算符均是从左至右结合

优先级（从高到低）
- ^ !(一元运算符)
- \* / % \<\< \>\> & &^
- \+ - | ^
- == != \< \<= \>= \>
- <-
- &&
- ||

```
a++
a--
++a //error
--a //error
b := a++ //error
b := a-- //error

```

## 流程控制

### for循环

-
```
for {
    if 条件 {
        break
    }
}
```
-
```
for 条件 {
	自增/减
}
```
-
```
for 变量 := 初始值; 变量 < 最大值; 变量++ {
    
}
```

### switch

-
```
switch 变量 {
case 常量值1:

case 常量值2:

default:

}
```
-
```
switch {
case 条件1:
    fallthrough
case 条件2:
default:
}
```

- continue 跳过这次循环, 进行下次循环

- break 跳出循环

- goto 跳到标签处


## array


## slice

```
slice := make(类型, 长度[,容量 = 长度])
s1 := make([]int, 3, 10)
```


## map

```
map := map[键类型]值类型
map := make(map[int]string)
```