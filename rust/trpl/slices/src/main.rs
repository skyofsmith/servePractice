fn main() {
    test_before();
    test_slices();
    test_slices_word();
    test_other_slices();
}

fn test_before() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word 的值为 5

    println!("word is {}", word);

    s.clear(); // 这清空了字符串，使其等于 ""

    println!("s is {}, word is {}", s, word);

    // word 在此处的值仍然是 5，
    // 但是没有更多的字符串让我们可以有效地应用数值 5。word 的值现在完全无效！
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn test_slices() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("hello is {}, world is {}", hello, world);
}

fn test_slices_word() {
    let s = String::from("hello world");

    let word = first_word(&s);
    println!("the first word is: {}", word);

//    s.clear(); // 错误!

    println!("the first word is: {}", word);

    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }
}

fn test_other_slices() {

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    print!("slice is ");
    for e in slice.iter() {
        print!("{} ", e);
    }
}