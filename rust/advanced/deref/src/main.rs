// 实现Deref trait允许我们重载解引用运算符
// let a: A = A::new();    // 前提：A类型必须实现Deref trait
// let b = &a;
// let c = *a; // 解引用

use std::ops::Deref;
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    {
        let x = 5;
        let y = &x;
        assert_eq!(5, x);
        assert_eq!(5, *y);

        let z = Box::new(x);
        assert_eq!(5, *z);
    }

    {
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    {
        let m = MyBox::new(String::from("Rust"));
        hello(&m);  //将MyBox变为&String，再将String的解引用变为字符串slice。   &str
        hello2(&m);
    }
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}

fn hello2(name: &String) {
    println!("Hello, {}", name);
}
//解引用多态与可变性交互：
//（1）当T：Deref<Target=U>时，从&T到&U
//（2）当T：DerefMut<Target=U>时，从&mut T 到&mut U
//（3）当T：Deref<Target=U>时，从&mut T到&U