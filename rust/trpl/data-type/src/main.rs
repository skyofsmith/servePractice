fn main() {
    test_err();
}
fn test_err() {
    let guess: u32 = "42".parse().expect("Not a number!");
    // let guess = "42".parse().expect("Not a number!");
    // error
}