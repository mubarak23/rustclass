fn main() {
    let s1 = String::from("Demo Borrow");
    let len = calculate_length(&s1);

    let mut s2 = String::from("Welcome"); 

    println!("The length of '{}' is {}.", s1, len);

    change(&mut s2);

    let mut s4 = String::from("Delware");

    let r1 = &mut s4;
    let r2 = &mut s4;
}

fn calculate_length (s: &String) -> usize {
    s.len()
}

fn change (some_thing: &mut String) {
    some_thing.push_str(", HOME");
    println!("{}", some_thing);
}