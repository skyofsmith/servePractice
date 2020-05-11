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
}
