

// you’ll explore how to define your own types, functions, and methods with generics!

// We can also define structs to use a generic type parameter in one or more fields using the <>

struct Point<T> {
    x: T,
    y: T,
}

// implement method on POINT struct
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct Pointgen<T, U> {
    x: U,
    y: T
}


// we can define enums to hold generic data types in their variants.
enum Option<T> {
    Some(T),
    None,
}

// we can express the abstract concept of an optional value, and because Option<T> is generic, 


// Enum can have multiple Generics
enum Result<T, E> {
    Ok(T),
    Err(E)
}

// The Result enum is generic over two types, T and E, and has two variants: Ok, which holds a value of type T, and
//  Err, which holds a value of type E. 

// We can implement methods on structs and enums (as we did in Chapter 5) and use generic types in their definitions, too.


fn main() {
    let both_integer = Pointgen { x: 5, y: 10 };
    let both_float = Pointgen { x: 1.0, y: 4.0 };
    let integer_and_float = Pointgen { x: 5, y: 4.0 };

    let number_list = vec![50, 34, 25, 100, 65];

    let largest_number = largest(&number_list);

    println!("The largest number is {}", largest_number);

    // let mut largest_number = &number_list[0];

    // for number in &number_list {
    //     if number > largest_number {
    //         largest_number = number;
    //     }
    // }
    // println!("The largest number is {}", largest_number);

     let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

     let largest_number = largest(&number_list);

      println!("The largest number is {}", largest_number);

      let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 6};
    let float = Point { x: 8.2, y: 12.4};

    println!("p.x = {}", integer.x());
  

}


  // To eliminate this duplication, we’ll create an abstraction by defining a function that operates on any list of 
    // integers passed in a parameter. 

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {

            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
     let mut largest = &list[0];

    for item in list {
        if item > largest {

            largest = item;
        }
    }
    largest
}

// generic function
// We read this definition as: the function largest is generic over some type T. 
// This function has one parameter named list, which is a slice of values of type T. 
// The largest function will return a reference to a value of the same type T.
// fn largest_gen<T>(list: &[T]) -> &T {

// }