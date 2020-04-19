fn main() {
    let s = String::from("hello world");

    let h = &s[0..5];
    let h1 = &s[0..=4];
    let h2 = &s[..5];
    let h3 = &s[..=4];
    println!("h = {}", h);
    println!("h1 = {}", h1);
    println!("h2 = {}", h2);
    println!("h3 = {}", h3);

    let w = &s[6..11];
    let w1 = &s[6..=10];
    let w2 = &s[6..];
    println!("w = {}", w);
    println!("w1 = {}", w1);
    println!("w2 = {}", w2);

    let s = &s[..];
    println!("s = {}", s);

    // let ss = String::from("你好");
    // let ss1 = &ss[0..1];
    // println!("ss1 = {}", ss1);
    // error

    let s3 = "hh";  // &str
    println!("s3 = {}", s3);

    let a = [1, 2, 3, 4];
    let sss = &a[1..3];
    println!("sss = {:?}, len = {}", sss, sss.len());
}
