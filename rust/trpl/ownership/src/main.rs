fn main() {
    test_scope();
    test_move();
    test_clone();
    test_function();
    test_return_value();
}

fn test_scope() {
    {                      // s 在这里无效, 它尚未声明
        let s = "hello";   // 从此处起，s 是有效的
        println!("s is {}", s);
        // 使用 s
    }                      // 此作用域已结束，s 不再有效
}

fn test_move() {
    let x = 5;
    let y = x;
    println!("x is {}, y is {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1;

//    println!("{}, world!", s1);
    println!("s2 is {}", s2);
}

fn test_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn test_function() {
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
    // ... 所以到这里不再有效
//    println!("s is {}", s);

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里，
    // 但 i32 是 Copy 的，所以在后面可继续使用 x
    println!("x is {}", x);
}
    // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
    // 所以不会有特殊操作

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

fn test_return_value() {
    let s1 = gives_ownership();         // gives_ownership 将返回值
    // 移给 s1
    println!("s1 is {}", s1);

    let s2 = String::from("hello");     // s2 进入作用域
//    println!("s2 is {}", s2);

    let s3 = takes_and_gives_back(s2);  // s2 被移动到    // s2 进入作用域
    println!("s3 is {}", s3);
    // takes_and_gives_back 中,
    // 它也将返回值移给 s3
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
// 所以什么也不会发生。s1 移出作用域并被丢弃

fn gives_ownership() -> String {             // gives_ownership 将返回值移动给
    // 调用它的函数

    let some_string = String::from("hello"); // some_string 进入作用域.

    some_string                              // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域

    a_string  // 返回 a_string 并移出给调用的函数
}