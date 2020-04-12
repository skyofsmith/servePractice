const MAX_POINT: u32 = 100000;

fn main() {
    //1、变量定义
    let a = 1;
    println!("a = {}", a);

    let mut b: u32 = 1;
    println!("b = {}", b);
    b = 2;
    println!("b = {}", b);

    //2、隐藏
    let b: f32 = 1.1;
    println!("b = {}", b);

    //3、常量
    println!("MAX_POINT = {}", MAX_POINT);

}
