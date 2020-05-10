//1、Rust中每一个引用都有其生命周期，也就是引用保持有效的作用域。大部分时候生命周期是隐含并可以推断的，正如大部分时候类型可以推断一样。
//2、生命周期的主要目标是避免悬垂引用
//3、Rust编译器使用借用检查器来检查生命周期是否有效
// fn longest(x: &str, y: &str) -> &str {
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
fn longest<'c>(x: &'c str, y: &'c str) -> &'c str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn get_str<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// error
// fn a_str<'a>(x: &'a str, y: &'a str) -> &'a str {
//     let r = String::from("abc");
//     r.as_str()
// }

//1、结构体中的生命周期
#[derive(Debug)]
struct A<'a> {
    name: &'a str,
}

//2、生命周期忽略
fn get_a_str(s: &str) -> &str {
    s
}
//（1）没有生命周期注解却能够编译，原因：早期的rust中必须显式的声明生命周期，后来rust团队将很明确的模式进行了注解的简化。
//（2）遵守生命周期省略规则的情况下能明确变量的生命周期，则无需明确指定生命周期。函数或者方法的参数的生命周期称为输入生命周期，而返回值的生命周期称为输出生命周期。
//（3）编译器采用三条规则判断引用何时不需要生命周期注解，当编译器检查完这三条规则后仍任不能计算出引用的生命周期，则会停止并生成错误。
//（4）生命周期注解省略规则适用于fn定义以及impl块定义，如下：
//     a、每个引用的参数都有它自己的生命周期参数。例如如下：
//       一个引用参数的函数，其中有一个生命周期： fn foo<'a>(x: &'a i32)
//       两个引用参数的函数，则有两个生命周期： fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
//       以此类推。
//     b、如果只有一个输入生命周期参数，那么它被赋予所以输出生命周期参数：
//          fn foo(x: &i32) -> &i32   等价于   fn foo<'a>(x: &'a i32) -> &'a i32
//     c、如果方法有多个输入生命周期参数， 不过其中之一因为方法的缘故为&self或者&mut self，那么self的生命周期被赋予所以输出生命周期参数。例子在下面来看
//          fn function(&self, x: &str, y: &st, ...) -> &str {}
fn main() {
    {
        // error
        /*
        let r;
        {
            let x = 5;
            r = &x;
        }
        println!("r = {}", r);
        */
        let r;
        let x = 5;
        r = &x;
        println!("x = {}", x);
        println!("r = {}", r);
        println!("---------------");
    }

    {
        let s1 = String::from("abcde");
        let s2 = String::from("ab");
        let r = longest(s1.as_str(), s2.as_str());
        println!("r = {}", r);
        let ss = get_str(s1.as_str(), s2.as_str());
        println!("ss = {}", ss);
        // let ss2 = a_str(s1.as_str(), s2.as_str());
        // println!("ss2 = {}", ss2);
        println!("---------------");
    }

    {
        let n = String::from("hello");
        let a = A {
            name: &n
        };
        println!("a = {:#?}", a);
    }
}
