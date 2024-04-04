
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i
        }
    }
    s.len()
}

fn first_word_slice (s: &str) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let strs = String::from("Demo");
    let slice1 = &strs[0..2];
    let slice2 = &strs[..2];
    // You can also drop both values to take a slice of the entire string. So these are equal:
    let len = strs.len();
    let slice3 = &strs[0..len];
    let slice4 = &strs[..];

    let mut s = String::from("Hello Slice Type");
    
    // let word = first_word(&s);


    // s.clear();

    let word = first_word_slice(&s[0..6]);
    let word1 = first_word_slice(&s[..]);
    let word2 = first_word_slice(&s);

    println!("{}", word);
      println!("{}", word1);
        println!("{}", word2);
    println!("Hello, world!");

    // STRING LITERALS AS SLICE

}

// String Slice: A string slice is a reference to part of a String
