fn main() {
    test_func();
    test_struct();
    test_method();
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn test_func() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn test_struct() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("integer is {:#?}, float is {:#?}", integer, float);
}

// #[derive(Debug)]
// enum Option<T> {
//     Some(T),
//     None,
// }

// #[derive(Debug)]
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn test_method() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}