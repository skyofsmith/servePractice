fn main() {
    test_err();
    test_float();
    test_operate();
    test_bool();
    test_char();

    test_tuple();
    test_array();
}

fn test_array() {
    let a = [1, 2, 3, 4, 5];
    println!("{}", a[0]);

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    println!("{}", months[0]);

    let a = [3; 5];
    println!("{}", a[0]);
}

fn test_tuple() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("five_hundred is {}, six_point_four is {}, one is {}", five_hundred, six_point_four, one);
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