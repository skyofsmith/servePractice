fn main() {
    let x: i32 = 1;
    {
        let y: i32 = 1;
        println!("x = {}", x);
        println!("y = {}", y);
    }

    {
        let mut s1 = String::from("hello");
        s1.push_str(" world");
        println!("s1 = {}", s1);    // String类型离开作用域的时候会调用drop方法

        let s2 = s1;
        println!("s2 = {}", s2);
        // println!("s1 = {}", s1); // move to s2, s1 invalid

        // clone
        let s3 = s2.clone();
        println!("s3 = {}", s3);
        println!("s2 = {}", s2);

    }

    {
        // copy
        let a = 1;
        let b = a;
        println!("a = {}, b = {}", a, b);
        // 常用的具有copy trait有：
        // 所以的整型
        // 浮点型
        // 布尔值
        // 字符类型 char
        // 元组
    }

    fn tasks_ownership(some_string: String) -> String{
        println!("{}", some_string);
        some_string
    }

    fn makes_copy(i: i32) {
        println!("i = {}", i);
    }
    {

        
        let s = String::from("hello");
        let s1 = tasks_ownership(s);
        println!("s1 = {}", s1);
        
        let x = 5;
        makes_copy(x);
        println!("x = {}", x);
    }

    println!("Hello, world!");
}
