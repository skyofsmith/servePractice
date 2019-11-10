// 当前程序的包名
package main

// 导入其他的包
import (
	"fmt"
)

// 常量的定义
const PI = 3.14

// 全局变量
var name = "gopher"

// 一般类型声明
type newType int

// 结构的声明
type gopher struct {}

// 接口的声明
type golang interface {}

// 由main函数作为程序入口启动点
func main()  {
	fmt.Println("Hello world!")
}
/*
Go内置关键字（25个均为小写）
break	default		func	interface	select
case	defer		go		map			struct
chan	else		goto	package		switch
const	fallthrough	if		range		type
continue	for		import	return		var
*/

//Go注释方法
//单行注释
/* 多行注释 */