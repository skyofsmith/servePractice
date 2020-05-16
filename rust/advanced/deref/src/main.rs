// 实现Deref trait允许我们重载解引用运算符
// let a: A = A::new();    // 前提：A类型必须实现Deref trait
// let b = &a;
// let c = *a; // 解引用

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

        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
}
