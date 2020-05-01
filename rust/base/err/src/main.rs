//一
//1、rust语言将错误分为两个类别：可恢复错误和不可恢复错误
//（1）、可恢复错误通常代表向客户报告错误和重试操作是合理的情况，例如未找到文件。rust中使用Result<T,E>来实现
//（2）、不可恢复错误是bug的同义词，如尝试访问超过数组结尾的位置。rust中通过panic!来实现。
//2、panic!
//3、使用BACKTRACE=1: RUST_BACKTRACE=1 cargo run
//4、Result<T,E>
//enum Result<T,E> {
//  Ok(T),
//  Err(E),
//}
//5、简写

//二
//1、当编写一个函数，但是该函数可能会失败，此时除了在函数中处理错误外，还可以将错误传给调用者，让调用者决定如何处理，这被称为传播错误。
//2、传播错误的简写方式，提倡的方式
//3、更进一步的简写
//4、什么时候用panic!，什么时候用Result
//5、Option和Result
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(error) => Err(error),
    }
}

fn main() {
    let r = read_username_from_file();
    match r {
        Ok(s) => println!("s = {}", s),
        Err(error) => println!("error = {:?}", error),
    };
/* 
    let f = File::open("hello.txt");
    let r = match f {
        Ok(file) => file,
        Err(error) => panic!("error: {:?}", error),
    };
    println!("r = {:?}", r);

    let rr = File::open("hello.txt").unwrap();
    println!("rr = {:?}", rr);
    let rrr = File::open("hello.txt").expect("error");
    println!("rrr = {:?}", rrr);
    // panic!("crash here!");
     */
}
