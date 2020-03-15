fn main() {
    test_string();
    test_update();
    test_operator_add();
    test_iterate_string();
}

fn test_string() {
    let mut s = String::new();
    println!("s is \"{}\"", s);
    let data = "initial contents";
    s = data.to_string();
    println!("s is \"{}\"", s);
    s = "hello world!".to_string();
    println!("s is \"{}\"", s);
    let s = String::from("initial contents");
    println!("s is \"{}\"", s);

    let hello1 = String::from("السلام عليكم");
    let hello2 = String::from("Dobrý den");
    let hello3 = String::from("Hello");
    let hello4 = String::from("שָׁלוֹם");
    let hello5 = String::from("नमस्ते");
    let hello6 = String::from("こんにちは");
    let hello7 = String::from("안녕하세요");
    let hello8 = String::from("你好");
    let hello9 = String::from("Olá");
    let hello10 = String::from("Здравствуйте");
    let hello11 = String::from("Hola");
    println!("hello1 is \"{}\"", hello1);
    println!("hello2 is \"{}\"", hello2);
    println!("hello3 is \"{}\"", hello3);
    println!("hello4 is \"{}\"", hello4);
    println!("hello5 is \"{}\"", hello5);
    println!("hello6 is \"{}\"", hello6);
    println!("hello7 is \"{}\"", hello7);
    println!("hello8 is \"{}\"", hello8);
    println!("hello9 is \"{}\"", hello9);
    println!("hello10 is \"{}\"", hello10);
    println!("hello11 is \"{}\"", hello11);
}

fn test_update() {
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s now is {}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("s is {}", s);
}

fn test_operator_add() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
//    println!("s1 is {}", s1);
    println!("s2 is {}", s2);
    println!("s3 is {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s is {}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {}", s);
}

fn test_iterate_string() {
    let s1 = "abc";
    println!("s1.len = {}", s1.len());
    for c in s1.chars() {
        println!("{}", c);
    }
    let s2 = "你好，世界！";
    println!("s2.len = {}", s2.len());
    for c in s2.chars() {
        println!("{}", c);
    }
    for c in s2.bytes() {
        println!("{}", c);
    }
}