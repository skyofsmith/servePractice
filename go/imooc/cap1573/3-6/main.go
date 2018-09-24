package main

import (
	imooc "fmt"

	"./learn1"
	_ "./show2"
)

func main() {
	learn1.Learn1()
	//	show2.Show2()
	imooc.Print("Hello imooc")
}

/*

别名的含义是:将导入的包命名为另一个容易记忆的别名;
点(.)操作的含义是:点(.)标识的包导入后, 调用该包中函数时可以省略前缀包名;
下划线(_)操作的含义是:导入该包, 但不导入整个包,而是执行该包中的init函数, 因此无法通过包名来调用包中的其他函数.使用下划线(_)操作往往是为了注册包里的引擎,让外部可以方便地使用

*/