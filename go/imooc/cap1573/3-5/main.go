package main

import (
	"fmt"

	"./learn1"
	"./show2"
)

func init() {
	fmt.Println("main init")
}

func main() {
	learn1.Learn1()
	show2.Show2()
	fmt.Print("Hello imooc")
}

/*

如果一个main导入其他包,包将被顺序导入;
如果导入的包中依赖其他包(包B),会首先导入B包,然后初始化B包中的常量和变量,最后如果B包中有init, 会自动执行init();
所以包导入完成后才会对main中常量和变量进行初始化,然后执行main中的init函数(如果存在), 最后执行main函数;
如果一个包被导入多次则该包只会被导入一次

示例:

main
import pkg1
			pkg1
			import pkg2
							pkg2
							const ...
							var ...
							init()
			const ...
			var ...
			init()

const ...
var ...
init()
main()

*/
