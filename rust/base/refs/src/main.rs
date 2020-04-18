fn main() {
    let s1 = gives_ownersip();
    println!("s1 = {}", s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("s3 = {}", s3);
    let s2 = takes_and_gives_back(s3);
    println!("s2 = {}", s2);

    // 引用 &
    // 用法：让我们创建一个指向值的引用，但是并不拥有它，因为不拥有这个值，所以，当引用离开其值所指向的作用域后也不会被丢弃
    let s1 = String::from("hello");
    let s = &s1;
    println!("&s1 = {}", s);
    let len = calcute_length(s);
    println!("len = {}", len);
    println!("s1 = {}", s1);

    // 借用 &mut
    let mut s2 = String::from("hello");
    println!("s2 = {}", s2);
    println!("&s2 = {}", &mut s2);
    modify_s(&mut s2);
    println!("s2 = {}", s2);

    // 借用变量后不能使用该变量的引用

    let mut ss = String::from("hello");

    let ss1 = &ss;
    let ss2 = &ss;
    println!("ss1 = {}", ss1);  // can use
    println!("ss2 = {}", ss2);  // can use
    let ss3 = &mut ss;

    // println!("ss1 = {}", ss1);   // error, can not use
    // println!("ss2 = {}", ss2);   // error, can not use
    println!("ss3 = {}", ss3);

    // 悬垂指针
    // 在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用，在有了可变引用之后不能再有不可变引用
    // let ref_s = dangle();
    // println!("ref_s = {}", ref_s);
}

fn gives_ownersip() -> String {
    let s = String::from("hello");
    s
}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn calcute_length(s: &String) -> usize {
    s.len()
}

fn modify_s(s: &mut String) {
    s.push_str(", world");
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
