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
