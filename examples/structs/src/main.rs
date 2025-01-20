fn main() {
    let mut user1 = build_user(String::from("user1@example.com"), String::from("someuser123"));;
    let user2 = User {
        email: String::from("user2@example.com"),
        ..user1
    };
    user1.username = String::from("mo");
    println!("username = {}", user2.username);
    println!("email = {}", user2.email);

    let black = Color(0, 0, 0);
    let origin = Color(0, 0, 0);

}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        //username: username,
        //email: email,
        email,
        username,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;