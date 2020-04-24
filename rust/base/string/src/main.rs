//1、创建一个空String
//2、通过字面值创建一个String
//2.1、 使用String::from()
//2.2、 使用String::from()
//3、更新String
//3.1、 push_str
//3.1、 push
//3.1、 使用“+”合并字符串
//3.1、 使用format
//4、String索引
//5、str索引
//6、遍历
//6.1、chars
//6.2、bytes

fn main() {
    //1
    let mut s0 = String::new();
    s0.push_str("hello");
    println!("s0 = {}", s0);
    println!("s0 = {}", s0);

    //2
    let s1 = String::from("init some thing");
    println!("{}", s1);

    let s1 = "init some thing".to_string();
    println!("{}", s1);

    //3
    let mut s2 = String::from("hello");
    s2.push_str(", world");
    let ss = " !".to_string();
    s2.push_str(&ss);
    println!("{}", s2);
    println!("ss = {}", ss);

    let mut s2 = String::from("tea");
    s2.push('m');
    // s2.push('mx'); //error
    // s2.push("X");  //error
    println!("s2 = {}", s2);

    let s1 = "hello".to_string();
    let s2 = String::from(", world");
    let s3 = s1 + &s2;
    // println!("s1 = {}", s1); //error
    println!("s3 = {}", s3);

    let s341 = String::from("tic");
    let s342 = String::from("tac");
    let s343 = String::from("toe");
    let s344 = format!("{}-{}-{}", s341, s342, s343);
    println!("s344 = {}", s344);    // format!和println!类似
}
