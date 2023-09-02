
use std::collections::HashMap;
fn main() {
    // let v: Vec<i32> = Vec::new()
    // let v = vec![1,2,3,4];

    // Hash Map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 50);
    scores.insert(String::from("Yellow"), 60);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count +=1;
    }

    println!("{:?}", map);

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    let third: &i32 = &v[2];
    println!("The Third Element is {third}");
    let third: Option<&i32> = v.get(2);
     match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    // println!("Hello, Vector Collection {}"; v);
    let d = vec![100, 120, 140, 160];
    for i in &d {
        println!("Element is {i}");
    }

    // let mut m = vec![20, 40, 60];
    // for i in &mut v {
    //     *i += 50;
    //     println!("Adding 50 to each element gives {i}");
    // }

    enum SpreadsheetCell {
        Int(i32),
       //  Float(i32),
        Text(String),
    }
    let row = vec! [
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("Green")),
       // SpreadsheetCell::Float(14.12),
    ];

   //  let mut s = String::new();
    // This line creates a new empty string called s, which we can then load data into. 

    let data = "Initial Contents";
   //  let s = data.to_string();
    let s = "Initial Content".to_string();

    // We can also use the function String::from to create a String from a string literal
    let literals = String::from("initial contents");
    // We can grow a String by using the push_str method to append a string slice
   //  literals.push_str(", Push More content from here");

    
}
