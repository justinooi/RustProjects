// Print structs in its entirety
#[derive(Debug)]

// Normal structs
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Tuple Structs - No Named Fields

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like Structs with no fields

struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 20,
    };

    user1.email = String::from("newemail@email.com");

    // You can declare a mutable user here instead of making the function return the mutability. Interesting insight!

    let mut user2 = build_user(String::from("test@test.com"), String::from("username"));

    user2.email = String::from("Test");

    // Creating users from an existing struct instance.

    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // Create an instance with less code by using .. syntax.println!

    let user4 = User {
        email: String::from("new@email.com"),
        ..user3
    };

    // Creating field nameless struct instances.

    let black = Color(0, 0, 0);
    let origin = Point(1, 2, 3);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 0,
    }
}
