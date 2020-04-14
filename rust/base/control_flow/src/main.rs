fn main() {
    // if
    let y = 1;
    if y == 1 {
        println!("y = 1");
    }

    // if-else
    if y == 1 {
        println!("y = 1");
    } else {
        println!("y != 1");
    }

    // if - else if - else
    if y == 1 {
        println!("y = 1");
    } else if y == 0 {
        println!("y = 0");
    } else if y == 2 {
        println!("y = 2");
    } else {
        println!("other");
    }
    
    // let if
    let condition = true;
    let x = if condition {
        5
    } else {
        6
        // "six" // error 两个{}的返回值类型必须相同
    };
    println!("x = {}", x);

    // loop
    let mut counter = 0;
    loop {
        println!("in loop");
        if counter == 10 {
            break;
        }
        counter = counter + 1;
        counter += 1;
    }
    println!("counter = {}", counter);

    let result = loop {
        counter += 1;
        if counter == 20 {
            break counter * 2
        }
    };
    println!("result = {}", result);

    // while
    let mut i = 0;
    while i != 10 {
        i += 1;
    }
    println!("i = {}", i);

    // for
    let arr:[u32; 5] = [1, 2, 3, 4, 5];
    for element in &arr {
        println!("element = {}", element);
    }
}
