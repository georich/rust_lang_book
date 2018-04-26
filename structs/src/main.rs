#![allow(unused_variables)]
fn main() {
    struct User {
        username: String, // a field
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // instantiate struct
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("user1's email is {}", user1.email);

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.username = String::from("adifferentuser123"); // if mutable we can change the field

    println!("user1's username is {}", user1.username);

    let built_user = build_user(String::from("test@email.com"), String::from("testuser"));

    fn build_user(email: String, username: String) -> User {
        User {
            // email: email,
            email, // like JS don't have to repeat variable and field match
            // username: username,
            username,
            active: true,
            sign_in_count: 1
        }
    } // returns a user instance with given email and username

    // Update syntax
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername789"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername789"),
        ..user1 // .. says remaining fields should have same fields as user1
    };

    // TUPLE STRUCTS
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // black and origin values are different types, as they are instances of different tuple structs
    // each struct defined is its own type
}
