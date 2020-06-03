
fn main() {
    {
        //1、函数指针
        //函数指针允许我们使用函数作为另一个函数的参数。
        //函数的类型是 fn ，fn 被称为 函数指针。指定参数为函数指针的语法类似于闭包。
        fn add_one(x: i32) -> i32 {
            x + 1
        }

        fn do_twice(f: fn(i32) -> i32, val: i32) -> i32 {
            f(val) + f(val)
        }
        let r = do_twice(add_one, 5);
        println!("r = {}", r);

        fn wapper_func<T> (t: T, v: i32) -> i32 
            where T: Fn(i32) -> i32 {
            t(v)
        }

        fn func(v: i32) -> i32 {
            v + 1
        }
        let a = wapper_func(|x| x + 1, 1);
        println!("a = {}", a);
        let b = wapper_func(func, 1);
        println!("b = {}", b);

        //函数指针实现了Fn、FnMut、FnOnce
    }
    {
        //2、返回闭包
        // fn return_clo() -> Fn(i32) -> i32 {
        //     |x| x + 1
        // }
        fn return_clo() -> Box<dyn Fn(i32) -> i32> {
           Box::new(|x| x+1)
        }
        let c = return_clo();
        println!("1 + 1 = {}", c(1));
    }
}
