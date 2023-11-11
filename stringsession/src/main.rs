

fn main() {
    let mut s: String = String::from("hello, ");
    s.push_str("world"); // https://doc.rust-lang.org/std/string/struct.String.html#method.push_str
    s.push('!');

    move_ownership(s.clone());

    assert_eq!(s, "hello, world!");

    println!("Success!");
}

fn move_ownership(s: String) {
    println!("ownership of \"{}\" is moved here!", s)
}

// FILL in the blanks
fn main() {  
   let mut s = String::from("hello, world");

   let slice1: &str = s.as_str(); // In two ways
   // let slice4: &str = &s // first way -> &string -> &str
   assert_eq!(slice1, "hello, world");

   let slice2: &str = &s[..5];
   assert_eq!(slice2, "hello");

   let slice3: &mut String = &mut s; 
   slice3.push('!');
   assert_eq!(slice3, "hello, world!");

   println!("Success!");
}
