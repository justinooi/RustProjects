// Print structs in its entirety
#[derive(Debug)]

// Default Structs with Named Fields
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple Structs without Named Fields
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    println!("Hello, world!");

    let black = Color(0,0,0);

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user1.email);

    user1.email = String::from("new@new.com");

    println!("{}", user1.email);

    let user2 = build_user(String::from("Test@domain.com"), String::from("username"));

    println!("{}", user2.email);

    let user3 = User {
        email: String::from("test@test.com"),
        username: String::from("test"),
        ..user2
    };

    println!("{} - {} - {}", user3.email, user3.username, user3.sign_in_count);
    println!("{:?}", user2);

    println!("{}", black.1);
}