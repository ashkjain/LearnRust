fn main() {
    // one_create_struct();
    let email = String::from("example@ex.com");
    let username = String::from("ashishjain");
    let user1 = build_user(email, username);
    println!("Username: {}\nEmail: {}\nActive: {}\nSign-in Count: {}",user1.username, user1.email, user1.active, user1.sign_in_count);

    let user2 = User{
        email: String::from("another@example.com"),
        ..user1
    };
    println!("Username: {}\nEmail: {}\nActive: {}\nSign-in Count: {}",user2.username, user2.email, user2.active, user2.sign_in_count);
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    println!("Black: {}, Origin: {}",black.0,origin.0);
    struct AlwaysEqual;
    let subject = AlwaysEqual;
    // println!("Subject: {}",AlwaysEqual);
}

fn one_create_struct()
{
    struct User
    {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = User
    {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
}
struct User
{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn build_user(email: String, username: String) -> User
{
    User
    {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}