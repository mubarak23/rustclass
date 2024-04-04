
fn take_ownership(some_string: String) {

    println!("{}", some_string);
}

fn make_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn give_ownership() -> String {

    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_len (s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
fn main() {
    let s = String::from("This is a String Type");

    take_ownership(s);
    // The double colon :: operator allows us to namespace this particular from function under the String type 
    // rather than using some sort of name like string_from

    // string literals

    let _literal = "Hello";

    let mut str = String::from("Hello");

    str.push_str(", World"); // append literals to a String

    let x = 4;
    let y = x;

    make_copy(x);

    println!("x = {}, y = {}", x, y);

    let s1 = String::from("Hello");
    // let s2 = s1; // this action free s1

    let s2 = s1.clone();

    // clone copy heap data of a STRING , Not just the stack data

    println!("{}", s1);

    println!("{}", s2);

    println!("{}", str);

    let s3 = give_ownership();

    println!("{}", s3);

    let s4 = String::from("Copy and Move Syntax");

    let s5 = takes_and_gives_back(s4.clone());

    let (s4, len) = calculate_len(s4.clone());

    println!("The length of '{}' is {}.", s4, len);

    let s6 = String::from("hello");
    let (s6, len) = calculate_len(s6.clone());

    println!("The length of '{}' is {}.", s5, len);


}
