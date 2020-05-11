//1、闭包是可以保存进变量或者作为参数传递给其他函数的匿名函数。闭包和函数不同的是，闭包允许捕获调用者作用域中的值。
//2、闭包的使用方式
//3、使用带有泛型和Fn trait的闭包
fn main() {
    let use_closure = || {
        println!("This is a closure");
    };
    use_closure();

    {
        fn add_one_v1(x: u32) -> u32 {
            x + 1
        }
        println!("{}", add_one_v1(1));

        let add_one_v2 = |x: u32| -> u32 {
            x + 1
        };
        println!("{}", add_one_v2(1));

        //闭包定义会为每个参数和返回值类型推导一个具体的类型，但是不能推导两次
        let add_one_v3 = |x| { x + 1 };
        println!("{}", add_one_v3(1));

        let add_one_v4 = |x| x + 1;
        println!("{}", add_one_v4(1));

        let example_closure = |x| x;
        let s = example_closure(String::from("hello"));
        println!("s = {}", s);

        // let n = example_closure(1);  // error
        let n = example_closure(String::from("s"));
        println!("n = {}", n);

        //捕捉环境中的变量
        let i = 1;
        let exe = |x| x + i;
        let r = exe(5);
        println!("r = {}", r);
    }

    {
        //实现一个缓存，只处理第一次传入的值并保存
        struct Cacher<T>
            where T: Fn(u32) -> u32
        {
            calculation: T,
            value: Option<u32>,
        }

        impl<T> Cacher<T>
            where T: Fn(u32) -> u32
        {
            fn new(calculation: T) -> Cacher<T> {
                Cacher {
                    calculation,
                    value: None,
                }
            }

            fn value(&mut self, arg: u32) -> u32 {
                match self.value {
                    Some(v) => v,
                    None => {
                        let v = (self.calculation)(arg);
                        self.value  = Some(v);
                        v
                    }
                }
            }
        }

        let mut c = Cacher::new(|x| x + 1);
        let v1 = c.value(1);
        println!("v1 = {}", v1);
        let v2 = c.value(2);
        println!("v2 = {}", v2);
    }

    //闭包可以通过三种方式捕获其环境，他们对应函数的三种获取参数方式，分别是获取所有权、可变借用、不可遍借用。
    //这三种捕获值的方式被编码为如下三个Fn trait:
    //（1）FnOnce消费从周围作用域捕获的变量，闭包周围的作用域被称为其环境。为了消费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移进闭包。其名称的Once部分代表了闭包不能多次获取相同变量的所有权。
    //（2）FnMut获取可变的借用值，所以可以改变其环境。
    //（3）Fn从其环境获取不可变的借用值。
    //当创建一个闭包时，rust会根据其如何使用环境中的变量来推断我们希望如何引用环境。由于所有闭包都可以被调用至少一次，因此所有闭包都实现了FnOnce。没有移动被捕获变量的所有权到闭包的闭包也实现了FnMut，而不需要对捕获的变量进行可变访问的闭包实现了Fn。
    {
        let x = 4;
        let equal_to_x = |z| z == x;
        let y = 4;
        assert!(equal_to_x(y));

        let x = vec![1, 2, 3];
        println!("x = {:?}", x);
        let equal_to_x = move |z| z == x;
        let y = vec![1, 2, 3];
        // println!("x = {:?}", x);
        assert!(equal_to_x(y));
    }
}
