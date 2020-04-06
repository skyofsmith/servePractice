fn main() {
    test_func();
    test_struct();
    test_method();
    test_method2();
    test_method3();
    test_();
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


fn test_struct() {
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }
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

fn test_method() {
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}

fn test_method2() {
    struct Point<T> {
        x: T,
        y: T,
    }
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }
    let p = Point {
        x: 3.0,
        y: 4.0
    };
    println!("p.distance_from_origin = {}", p.distance_from_origin());

}

fn test_method3() {
    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn test_() {
    #[derive(Debug)]
    enum Option<T> {
        Some(T),
        _None,
    }

    let integer = Option::Some(5);
    let float = Option::Some(5.0);
    println!("integer is {:#?}, float is {:#?}", integer, float);
//    enum Option_i32 {
//        Some(i32),
//        None,
//    }
//
//    enum Option_f64 {
//        Some(f64),
//        None,
//    }

//        let integer = Option_i32::Some(5);
//        let float = Option_f64::Some(5.0);

}