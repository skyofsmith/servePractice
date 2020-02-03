fn main() {
    test_err();
    test_float();
    test_operate();
    test_bool();
    test_char();
}

fn test_char() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{} {} {}", c, z, heart_eyed_cat);
}

fn test_bool() {
    let t = true;

    let f: bool = false; // 显式指定类型注解

    println!("{} {}", t, f);
}

fn test_operate() {
    // 加法
    let sum = 5 + 10;
    println!("5 + 10 = {}", sum);

    // 减法
    let difference = 95.5 - 4.3;
    println!("95.5 - 4.3 = {}", difference);

    // 乘法
    let product = 4 * 30;
    println!("4 * 3 = {}", product);

    // 除法
    let quotient = 56.7 / 32.2;
    println!("56.7 / 32.2 = {}", quotient);

    // 取余
    let remainder = 43 % 5;
    println!("43 % 5 = {}", remainder);
}

fn test_float() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("x is {}, y is {}", x, y);
}

fn test_err() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess is {}", guess);
    // let guess = "42".parse().expect("Not a number!");
    // error
}