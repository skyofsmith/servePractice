fn main() {
    test_define_enum();
    test_enum();
    test_enum2();
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
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    println!("home is {:#?}, loopback is {:#?}", home, loopback);
}