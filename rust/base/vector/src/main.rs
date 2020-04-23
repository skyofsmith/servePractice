//1、创建空的vector: Vec<T>
//2、创建包含初始值的vector
//3、丢弃vector
//4、读取元素
//5、更新
//6、遍历
//7、使用枚举
fn main() {
    // 1
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    println!("v = {:?}", v);

    // 2
    let v = vec![1, 2, 3];
    println!("v = {:?}", v);

    // 3
    {
        let v1 = vec![1, 2, 3];
        println!("v1 = {:?}", v1);
    }

    // 4
    let one: &i32 = &v[0];
    println!("one = {}", one);
    println!("one = {}", *one);

    // let f: i32 = v[3];   // panicked
    // println!("f = {}", f);

    println!("v = {:?}", v);
    match v.get(1) {
        Some(value) => println!("value = {}", value),
        _ => println!("None"),
    }

    match v.get(4) {
        Some(value) => println!("value = {}", value),
        _ => println!("None"),
    }

    // 5
    let mut v2: Vec<i32> = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);
    println!("v2 = {:?}", v2);

    // 6
    // (1)不可变的遍历
    for i in &v2 {
        println!("i = {}", i);
    }
    println!("v2 = {:?}", v2);

    // (2)可变的遍历

    for i in &mut v2 {
        *i += 1;
        println!("i = {}", i);
    }
    println!("v2 = {:?}", v2);

    // 7
    #[derive(Debug)]
    enum Context {
        Text(String),
        Float(f32),
        Int(i32),
    };

    let v3 = vec![
        Context::Text(String::from("string")),
        Context::Int(-1),
        Context::Float(3.14),
    ];
    println!("v3 = {:?}", v3);

    // 8
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    println!("first = {}", first);   // ok
    v.push(6);
    // println!("first = {}", first);   //  error
}
