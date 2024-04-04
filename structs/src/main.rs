
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// TUPLE STRUCT
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


// UNIT LIKE STRUCTS
struct AlwaysEqual;

fn build_user (username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 2,
    }
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("Mubarak23"),
        email: String::from("mubarak23@gmail.com"),
        sign_in_count: 1
    };

    let mut user2 = User {
        username: String::from("Mubtech"),
        email: String::from("mubitech@gmail.com"),
        active: true,
        sign_in_count: 5,
    };

    let user3 = User {
        username: String::from("Rust"),
        email: String::from("Rust occean"),
        active: true,
        sign_in_count: 2,
    };

    let user4 = User {
        email: String::from("Rust@rust.com"),
        ..user1
    };

    // The ..user1 must come last to specify that any remaining fields should get their values from the 
    // corresponding fields in user1, but we can choose to specify values for as many fields as we want in any order,
    //  regardless of the order of the fields in the struct’s definition.


    user2.email = String::from("tech001@gmail.com");

    println!("immutable User: {} ", user1.username);

    println!("Mutable User: {} ", user2.email);

    let demouser = build_user(user1.username, user1.email);

    println!("Build User: {} ", demouser.username);

    // USE OF TUPLE STRUCTS
    let black = Color(0, 0, 0, 0);
    let origin = Point(0, 0, 0, 0)

    // use unit like struct
    let subkect = AlwaysEqual;
   
}
