fn main() {
    let s1 = String::from("Hello");
    let len = calculate_len(&s1);
    println!("The length of '{}' is {}.", s1, len );
}


fn calculate_len(s: &String) -> usize {
    s.len()
}