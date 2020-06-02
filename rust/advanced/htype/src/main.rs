fn main() {
    {
        //1、类型别名
        type Kilometers = i32;
        let x: i32 = 5;
        let y: Kilometers = 6;
        let r: i32 = x + y;
        println!("x + y = {}", r);

        //类型别名的主要用途是减少重复。
        //（1）考虑如下类型：
        //Box<dyn Fn() + Send + 'static>
        ////如代码：
        //let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
        //
        //fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
        //    // --snip--
        //}
        //
        //fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        //    // --snip--
        //}
        //
        ////使用别名，代码：
        //type Thunk = Box<dyn Fn() + Send + 'static>;
        //let f: Thunk = Box::new(|| println!("hi"));
        //fn takes_long_type(f: Thunk) {
        //    // --snip--
        //}
        //fn returns_long_type() -> Thunk {
        //    // --snip--
        //}

        ////（2）考虑如下例子：
        //use std::io::Error; //标准库中的std::io::Error结构体代表了所有可能的I/O错误
        //use std::fmt;
        //pub trait Write {
        //    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
        //    fn flush(&mut self) -> Result<(), Error>;
        //    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
        //    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
        //}
        //
        ////加上如下类型别名声明：
        //type Result<T> = std::result::Result<T, std::io::Error>;//result<T, E> 中 E 放入了 std::io::Error
        //
        ////代码就可以变成：
        //pub trait Write {
        //    fn write(&mut self, buf: &[u8]) -> Result<usize>;
        //    fn flush(&mut self) -> Result<()>;
        //
        //    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
        //    fn write_fmt(&mut self, fmt: Arguments) -> Result<()>;
        //}
    }

    {
        //2、从不返回的never type
        //Rust 有一个叫做 ! 的特殊类型。在类型理论术语中，它被称为 empty type，因为它没有值。
        //我们更倾向于称之为 never type。在函数不返回的时候充当返回值
        // fn bar() -> ! {
        //     loop {}
        // }

        //以下例子为《Rust程序设计语言》中第二章“猜猜看”游戏的例子
        use std::io;
        use std::cmp::Ordering;
        use rand::Rng;
        println!("Guess the number!");
        let secret_number = rand::thread_rng().gen_range(1, 101);
        loop {
            println!("Please input your guess.");
            let mut guess = String::new();
            io::stdin().read_line(&mut guess)
                .expect("Failed to read line");
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue, //continue 的值是 !。
                                    //当 Rust 要计算 guess 的类型时，它查看这两个分支。
                                    //前者是 u32 值，而后者是 ! 值。
                                    //因为 ! 并没有一个值，Rust 决定 guess 的类型是u32
            };
            println!("You guessed: {}", guess);
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }
        //panic!
        //Option<T> 上的 unwrap 函数代码：
        //impl<T> Option<T> {
        //    pub fn unwrap(self) -> T {
        //        match self {
        //            Some(val) => val,
        //            None => panic!("called `Option::unwrap()` on a `None` value"),
        //        }
        //    }
        //}
    }
}
