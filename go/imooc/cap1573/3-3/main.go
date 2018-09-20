package main

import "fmt"

func main() {
	fmt.Println("Hello world!")
}

/*
package是最基本的分发单位和工程管理中依赖关系的体现;
每个GO语言源代码文件开头都拥有一个package声明,表示源码文件所属代码包;
要生成GO语言可执行程序,必须要有main的package包,且必须在该包下有main()函数;
同一个路径下只能存在一个package,一个package可以拆成多个源文件组成;
*/
