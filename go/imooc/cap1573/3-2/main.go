//程序所属包
package main

//导入依赖包
import "fmt"

//常量定义
const NAME string = "imooc"

//全局变量的声明与赋值
var a string = "慕课网"

//一般类型声明
type imoocInt int

//结构的声明
type Learn struct{}

//声明接口
type Ilearn interface{}

//函数定义
func learnImooc() {}

//main()函数
func main() {
	fmt.Print("Hello world!")
}
