fn main() {
    test_define_enum();
    test_enum();
    test_enum2();
    test_option();
}

fn test_define_enum () {
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("four is {:#?}, six is {:#?};", four, six);
}

fn test_enum() {
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("home is {:#?}, loopback is {:#?}", home, loopback);
}

fn test_enum2() {
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    println!("home is {:#?}, loopback is {:#?}", home, loopback);
}

fn test_option() {
    #[derive(Debug)]
    enum Option<T> {
        Some(T),
        None,
    }

    let some_number = Some(5);
    let some_string = Some("a string");

//    let absent_number: Option<i32> = None;
    println!("some_number is {:#?}", some_number);
    println!("some_string is {:#?}", some_string);

    /*
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
    */
}
