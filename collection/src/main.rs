fn main() {

    use std::collections::HashMap;
    // VECTOR 
    // vector : store more than one value in a single data structure,
    // value are put next to each other in memory
    // When we create a vector to hold a specific type, we can specify the type within angle brackets.
    // i.e v will hold element of type i32
    let ve: Vec<i32> = Vec::new();

    let sample = vec![2, 4, 6, 8];

    // updating a vector
    // we can add element using the push method
    // for us to able to change the value in the vector, it has to be mutable
    // let mut v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(4);
    v.push(8);
    v.push(10);

    // reading element from a vector
    // There are two ways to reference a value stored in a vector: via indexing or using the get method. 
    
    let third: &i32 = &v[3];
    println!("The Third Element in the Vector is {}", third);

    let fourth: Option<&i32> = v.get(4);
    match fourth {
        Some(fourth) => println!("The Fourth Element is {}", fourth),
        None => println!("The Fourth does not exist")
    }
    // Using & and [] gives us a reference to the element at the index value.
    // When we use the get method with the index passed as an argument, we get an Option<&T> that we can use with match.

    // INTERATING OVER THE ELEMENT IN A VECTOR

    // To access each element in a vector in turn, we would iterate through all of the elements 
    // rather than use indices to access one at a time.

    for i in &sample {
        println!("{i}");
    } 

    // We can also iterate over mutable references to each element in a mutable vector in order to make changes to all the elements
    let mut sum = 0;
    for i in &mut v {
        *i += 50;
        sum += *i
    }
 // *i is deferencing the value

    println!("Sum of mut Vector: {}", sum);

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = [
        SpreadsheetCell::Int(6),
        SpreadsheetCell::Float(14.12),
        SpreadsheetCell::Text(String::from("Demo from Vector")),
    ];


    // Storing UTF-8 Encoded Text with Strings

    // creating a new string
    let mut s = String::new(); // new empty string

    let data = "initial content";

    let sm = data.to_string();

    let sms = "Intial data".to_string();

    // We can also use the function String::from to create a String from a string literal
    let lit = String::from("Demo intial content");

    // Append to a String with push and push_str
    let mut item = String::from("Demo");
    item.push_str("Story");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // Hash maps
    // store a map of keys of type K to values of type V using a hash function 
    // hash map store their data inside a heap
    // hash maps are homogeneous: all of the keys must have the same type as each other, 
    // and all of the values must have the same type.

    // create a new hashmap
    let mut scores = HashMap::new();

    // adding element inside a hashmap using insert
    scores.insert(String::from("Blue"), 5);
    scores.insert(String::from("Yellow"), 45);

    // accessing value from a hash map
    let team_name = String::from("Blue");
    let team_scores = scores.get(&team_name).copied().unwrap_or(0);

    println!("Team name: {}", team_name);
    println!("Team Scores: {}", team_scores);
    // iterate over key values in a hash map
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // HASH MAP AND OWNERSHIP
    let field_name = String::from("Favorite color");
    let field_value = String::from("Green");

    let mut new_map = HashMap::new(); 
     new_map.insert(field_name, field_value); // ownership has been move into the hashmap

    // new_map.insert(field_name, field_value.clone()); // if we clone the value, we will still have it 
   //  println!("Trying to see if field_value is still here: {}", field_value);

   // UPDATING A HASH MAP

   // overriding a hash map value

   let mut league_table = HashMap::new();

   league_table.insert(String::from("Arsenal"), 1);

  //  league_table.insert(String::from("Arsenal"), 2);
  //  println!("{:?}", league_table);

  // Adding a Key and Value Only If a Key Isn’t Present
    league_table.entry(String::from("Manchester City")).or_insert(3);
    league_table.entry(String::from("LiverPool")).or_insert(2);

    // The or_insert method on Entry is defined to return a mutable reference to the value for the corresponding 
    // Entry key if that key exists, and if not, inserts the parameter as the new value for this key and returns 
    // a mutable reference to the new value

    println!("{:?}", league_table);

    // UPDATING VALUE BASE ON OLD VALUE

    let text = "hello world wonderful world";


    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    // The split_whitespace method returns an iterator over sub-slices, separated by whitespace, of the value in text. 
    // The or_insert method returns a mutable reference (&mut V) to the value for the specified key.

    println!("{:?}", map);

    println!("Hello, world!");
}
