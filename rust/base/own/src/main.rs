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

    println!("Hello, world!");
}
