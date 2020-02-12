fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

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

}
