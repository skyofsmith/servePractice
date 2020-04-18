fn other_fun() {
    println!("This is a function");
}

fn other_fun1(a: i32, b: u32) {
// fn other_fun1(a, b) {    // error
    println!("a = {}, b = {}", a, b);
}

fn other_fun2(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    other_fun();
    
    let a: i32 = -1;
    let b: u32 = 2;
    other_fun1(a, b);
    other_fun1(1, 2);

    let c: i32 = 9;
    let r: i32 = other_fun2(a, c);
    println!("r = {}", r);

    // let x = (let y = 1) // error

    let y = {
        let x = 1;
        x + 1
    };
    println!("y = {}", y);

    println!("Hello, world!");
}
