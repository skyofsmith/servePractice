fn main() {
    {
        //1、关联类型在trait定义中指定占位符类型
        //关联类型是一个将类型占位符与trait相关联的方式。
        //trait 的实现者会针对特定的实现在这个类型的位置指定相应的具体类型。
        //如此可以定义一个使用多种类型的 trait。
        // pub trait Iterator {
        //     type Item;
        //     fn next(&mut self) -> Option<Self::Item>;
        // }
        pub trait Iterator1<T> {
            fn next(&mut self) -> Option<T>;
        }

        struct A {
            value: i32
        }

        impl Iterator1<i32> for A {
            fn next(&mut self) -> Option<i32> {
                println!("in i32");
                if self.value > 3 {
                    self.value += 1;
                    Some(self.value)
                } else {
                    None
                }
            }
        }
        impl Iterator1<String> for A {
            fn next(&mut self) -> Option<String> {
                println!("in String");
                if self.value > 3 {
                    self.value += 1;
                    Some(String::from("hello"))
                } else {
                    None
                }
            }
        }

        let mut a = A {
            value: 3
        };
        // a.next();    // error
        <A as Iterator1<i32>>::next(&mut a);  //完全限定语法，带上了具体的类型
        <A as Iterator1<String>>::next(&mut a);
    }

    {
        //2、默认泛型类型参数和运算符重载
        //（1）使用泛型类型参数时，可以为泛型指定一个默认的具体类型。
        //（2）运算符重载是指在特定情况下自定义运算符行为的操作。
        //  Rust并不允许创建自定义运算符或者重载运算符，
        //  不过对于std::ops中列出的运算符和相应的trait，我们可以实现运算符相关trait来重载。
        use std::ops::Add;

        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        };

        impl Add for Point {
            type Output = Point;
            fn add(self, other: Point) -> Point {
                Point {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }

        assert_eq!(Point{ x: 1, y: 1} + Point {x: 2, y: 2}, Point {x: 3, y: 3});

        #[derive(Debug)]
        struct Millimeters(u32);
        #[derive(Debug)]
        struct Meters(u32);

        impl Add<Meters> for Millimeters {
            type Output = Millimeters;
            fn add(self, other: Meters) -> Millimeters {
                Millimeters (self.0 + other.0 * 1000)
            }
        }

        let mi = Millimeters(1);
        let m = Meters(1);
        let r = mi + m;
        println!("r = {:#?}", r);

        //trait Add<RHS=Self> { //尖括号里面为默认类型参数，RHS是一个泛型类型参数（right hand side）
        //    type Output;
        //    fn add(self, rhs: RHS) -> Self::Output;
        //}
    }
}
