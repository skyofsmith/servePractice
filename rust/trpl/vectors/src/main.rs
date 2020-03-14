fn main() {
    test_vector();
    test_read_vector();
    test_iterate_vector();
    test_use_enum();
}

fn test_vector () {
    let v: Vec<i32> = Vec::new();
    println!("v is {:#?}", v);
    let v = vec![1, 2, 3];
    println!("v is {:#?}", v);

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("v is {:#?}", v);
}

fn test_read_vector() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

fn test_iterate_vector() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }
}

fn test_use_enum() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}