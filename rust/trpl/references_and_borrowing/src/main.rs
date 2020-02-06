fn main() {
    test_reference();
    test_reference_change();
    test_multi_mut();
    test_dangling_reference();
}

fn test_reference() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn test_reference_change() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn test_multi_mut() {
    // error
//    let mut s = String::from("hello");

//    let r1 = &mut s;
//    let r2 = &mut s;

//    println!("{}, {}", r1, r2);

    // ok
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        println!("r1 is {}", r1);

    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r2 = &mut s;
    println!("r2 is {}", r2);
// error
//    let mut s = String::from("hello");

//    let r1 = &s; // 没问题
//    let r2 = &s; // 没问题
//    let r3 = &mut s; // 大问题

//    println!("{}, {}, and {}", r1, r2, r3);

    // ok
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
// 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s; // 没问题
    println!("{}", r3);
}

fn test_dangling_reference() {
//    let reference_to_nothing = dangle();
    let s = no_dangle();
    println!("s is {}", s);
}

//fn dangle() -> &String {
//    let s = String::from("hello");
//    &s
//}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
