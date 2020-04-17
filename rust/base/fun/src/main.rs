fn other_fun() {
    println!("This is a function");
}

fn other_fun1(a: i32, b: u32) {
// fn other_fun1(a, b) {    // error
    println!("a = {}, b = {}", a, b);
}

fn main() {
    other_fun();
    
    let a: i32 = -1;
let b: u32 = 2;
    other_fun1(a, b);

    println!("Hello, world!");
}
