fn main() {
    #[derive(Debug)]
    struct User {
        name: String,
        count: String,
        nonce: u64,
        active: bool,
    }

    let xiaoming = User {
        name: String::from("小明"),
        count: String::from("80001000"),
        nonce: 10000,
        active: true,
    };

    let mut xiaohuang = User {
        name: String::from("xiaohuang"),
        count: String::from("80001000"),
        nonce: 10000,
        active: true,
    };
    xiaohuang.nonce = 20000;

    let name = String::from("xiaohong");
    let count = String::from("12345678");
    let nonce = 200000;
    let active = false;
    let user1 = User {
        name,
        count,
        nonce,
        active,
    };
    let user2 = User {
        name: String::from("xiaowang"),
        ..user1
    };
    println!("name = {}", user2.name);
    println!("nonce = {}", user2.nonce);
    
    struct Point (i32, i32);
    let a = Point (10, 20);
    let b = Point (30, 20);
    println!("a is ({}, {}), b is ({}, {})", a.0, a.1, b.0, b.1);

    struct A {};

    println!("xiaoming = {:?}", xiaoming);
    println!("xiaohuang = {:#?}", xiaohuang);
    println!("user2 = {:#?}", user2);
}
