fn main() {

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("user1.email is {}", user1.email);
    println!("user1.username is {}", user1.username);
    println!("user1.active is {}", user1.active);
    println!("user1.sign_in_count is {}", user1.sign_in_count);

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("user1.email is {}", user1.email);
    user1.email = String::from("anotheremail@example.com");

    println!("user1.email is {}", user1.email);

    let user2 = build_user(String::from("email2"), String::from("Sam"));
    println!("user2.email is {}", user2.email);

    let user3 = build_user_alias(String::from("email3"), String::from("Tom"));
    println!("user3.email is {}", user3.email);

    let user4 = User {
        email: String::from("another4@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    println!("user4.email is {}", user4.email);

    let user5 = User {
        email: String::from("another5@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    println!("user5.email is {}", user5.email);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_alias(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
