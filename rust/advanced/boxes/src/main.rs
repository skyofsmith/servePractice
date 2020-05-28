//1、最简单最直接的智能指针是box，其类型为Box<T>。box允许将值放在堆上而不是栈上，留着栈上的则是指向堆数据的指针。除了数据被存储在堆上外，box没有任何性能损失。
//2、box适合用于如下场景：
//（1）当有一个在编译时未知大小的类型，而又需要再确切大小的上下文中使用这个类型值的时候；
//（举例子：在一个list环境下，存放数据，但是每个元素的大小在编译时又不确定）
//（2）当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候；
//（3）当希望拥有一个值并只关心它的类型是否实现了特定trait而不是其具体类型时。

// error
// enum List {
//     Cons(i32, List),
//     Nil,
// }
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil
}

fn main() {
    {
        let a = &5;
        let b = Box::new(5);
        println!("a = {}", a);
        println!("b = {}", b);
    }

    {
        use List::*;
        let list = Cons(1,
            Box::new(
                Cons(2,
                    Box::new(
                        Cons(3,
                            Box::new(Nil)
                        )
                    )
                )
            )
        );
        println!("list:\n{:#?}", list);
    }
}