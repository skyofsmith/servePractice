static HELLO_WORLD: &str = "Hello, world";
static mut COUNTER: u32 = 0;
fn main() {
    {
        //4、解引用裸指针
        //不可变和可变的，分别写作*const T, *mut T
        //（1）允许忽略借用规则，可以同时拥有不可变和可变的指针，或者是多个指向相同位置的可变指针
        //（2）不保证指向的内存是有效的
        //（3）允许为空
        //（4）不能实现任何自动清理的功能
        let mut num = 5;
        //创建不可变和可变的裸指针可以在安全的代码中，只是不能在不安全代码块之外解引用裸指针
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        // println!("r1 is: {}", *r1);  // error
        // println!("r2 is: {}", *r2);  // error

        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }

        let add = 0x12345usize;
        let _r = add as *const i32;
    }

    {
        //5、调用不安全的函数或者方法
        unsafe fn dangerous () {
            println!("do something dangerous");
        }

        fn foo () {
            let mut num = 5;
            let r1 = &num as *const i32;
            let r2 = &mut num as *mut i32;

            unsafe {
                println!("r1 is: {}", *r1);
                println!("r2 is: {}", *r2);
            }
        }

        // dangerous(); // error
        unsafe {
            dangerous();
        }
        foo();
    }

    {
        //调用c语言的函数
        extern "C" {
            fn abs(input: i32) -> i32;
        }
        unsafe {
            println!("abs(-3): {}", abs(-3));
        }
    }

    {
        //6、访问或者修改可变静态变量
        println!("{}", HELLO_WORLD);
        //静态变量和常量的区别：
        //1、静态变量有一个固定的内存地址（使用这个值总会访问相同的地址），常量则允许在任何被用到的时候复制其数据。
        //2、静态变量可以是可变的，虽然这可能是不安全（用unsafe包含）
        fn add_counter(inc: u32) {
            unsafe {
                COUNTER += inc;
            }
        }

        add_counter(3);
        add_counter(3);
        unsafe {
            println!("counter: {}", COUNTER);
        }
    }

    {
        //7、实现不安全的trait
        //（1）当至少有一个方法中包含编译器不能验证的不变量时，该trait就是不安全的。
        //（2）在trait之前增加unsafe声明其为不安全的，同事trait的实现也必须用unsafe标记。
        unsafe trait Foo {
            fn foo(&self);
        }

        struct Bar();

        unsafe impl Foo for Bar {
            fn foo(&self) {
                println!("foo");
            }
        }

        let a = Bar();
        a.foo();
    }
}
