fn main() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64
    }
    let mut myuser = User {
        active: true,
        username: String::from("mubarak23"),
        email: String::from("mubarakaminu340@gmail.com"),
        sign_in_count: 1
    };
    myuser.email = String::from("anotheremail@example.com");
    println!("{}", myuser.email );
    println!("Hello, world!");
}

// STRUCT
// the pieces of a struct can be different types.
// struct you’ll name each piece of data so it’s clear what the values mean.
// To define a struct, we enter the keyword struct and name the entire struct. 
// A struct’s name should describe the significance of the pieces of data being grouped together
// when initializing a struct, We don’t have to specify the fields in the same order in which we declared them in the struct.
