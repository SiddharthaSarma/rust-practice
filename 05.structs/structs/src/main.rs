mod rect;
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// without named fields
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

// Unit like structs (will discuss in chapter 10)
struct AlwaysEqual;

// struct UserRef {
//     active: bool,
//     username: &str,
//     email: &str,
//     sign_in_count: u64,
// }

fn main() {
    rect::get_rect();
    let mut user1 = User {
        active: true,
        username: String::from("siddhartha"),
        email: String::from("test@test.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("someone@example.com");
    let user2 = build_user(String::from("demouser@test.com"), String::from("demouser"));
    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };
    println!("{:#?}", user1);
    // println!("{:#?}", user2); // user2 data is no longer accessible
    println!("{:#?}", user3);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("color black: {:?}", black);
    println!("point origin: {:?}", origin);
    let subject = AlwaysEqual;

    // will discuss in chapter 10
    // let user4 = User {
    //     email: "someone@example.com",
    //     username: "someusername123",
    //     active: true,
    //     sign_in_count: 1,
    // };
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // username: username
        email,    // email: email
        sign_in_count: 1,
    }
}
