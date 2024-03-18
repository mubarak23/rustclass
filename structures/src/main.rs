
//#![allowed(dead_code)]

//#[derived(debug)]
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

struct Unit;

//struct Pair (i32, f32)
struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    buttom_left: Point
}

fn react_area (react: &Rectangle) -> f32 {
    let Rectangle {top_left: Point {x: x1, y: y1}, buttom_left: Point {x: x2, y: y2} } = react;
    let width = (x1 - x2).abs();
    let height = (y1 - y2).abs();
    
    width * height
}

fn square(top_left: Point, side_length: f32) -> Rectangle {
    let buttom_left = Point { x: top_left.x, y: top_left.y - side_length };
    let top_right = Point { x: top_left.x + side_length, y: top_left.y };
    Rectangle {
        top_left: top_left,
        buttom_left: buttom_left,
        //top_right: top_right,
        // buttom_right: Point { x: top_left.x + side_length, y: top_left.y - side_length },
    }
}


fn main() {
    let name = String::from("Peter");
    let age = 56;

    let me = Person { name, age};

    //debug struct 
    println!("{:?}", me);

    let point: Point = Point {x: 8.6, y: 12.4};

    // access field of point structs
    println!("Point coordinate: ({}, {})", point.x, point.y);

    let buttom_left = Point { x: 6.4, ..point };


    println!("Second Point Coordinate: ({}, {})", buttom_left.x, buttom_left.y);

    // destructure the point using a let binding
    let Point {x: left_edge, y: right_edge} = point;

    let _reactangle = Rectangle {
        top_left: Point { x: left_edge, y: right_edge},
        buttom_left: buttom_left
    };

    // instantiate unit
    let _unit = Unit;

    // instantiate a tuple
    let pair = Pair(1, 1.8);

    // access to field in tuple
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    //destructure tuple
    let Pair(integer, decimal) = pair;

    // access to destructure tuple pairs
    println!("pair contains {:?} and {:?}", integer, decimal);

    let reactangle = Rectangle {
        top_left: Point { x: 5.0, y: 4.0},
        buttom_left: Point { x: 6.0, y: 8.0}
    };

    println!("Area of the rectangle: {}", react_area(&reactangle));

    println!("Hello, world!");
}

// From the provided code, you learn several key concepts in Rust:

// 1. **Structs**: Rust allows you to define custom data types using `struct`. In this code, you have `Person`, `Point`, and `Rectangle` structs.

// 2. **Tuple Structs**: Rust also supports tuple structs, which are similar to tuples but have named fields. `Pair` is an example of a tuple struct in this code.

// 3. **Methods**: Although not explicitly shown in this code, you can define methods on structs to encapsulate functionality related to those structs.

// 4. **Debugging with Derive**: Rust provides a `#[derive(Debug)]` attribute that automatically implements the `Debug` trait for a struct, allowing you to print debug representations of struct instances using `println!("{:?}", variable)`.

// 5. **Pattern Matching and Destructuring**: Rust allows you to pattern match and destructure data structures, such as structs and tuples. The `react_area` function demonstrates pattern matching and destructuring to calculate the area of a rectangle.

// 6. **Borrowing and References**: Rust emphasizes ownership and borrowing to ensure memory safety. Functions like `react_area` and `square` take references (`&Rectangle`, `&Point`) rather than taking ownership of the data, which allows them to operate on the data without transferring ownership.

// 7. **Ownership and Lifetimes**: While lifetimes aren't explicitly shown in this code, Rust's ownership and borrowing system ensures memory safety by enforcing strict rules about how data is borrowed and used.

// 8. **Functions**: Rust functions are declared using the `fn` keyword. Functions can take parameters and return values, just like in most other programming languages.

// 9. **Control Flow**: Rust provides standard control flow constructs like `if` statements and loops (`for` and `while`). They are not demonstrated in this code but are essential for writing complex logic in Rust programs.

// 10. **Modules and Visibility**: Rust allows you to organize code into modules and control the visibility of items (structs, functions, etc.) using the `pub` keyword. In this code, everything is within the same module, but in larger projects, you may encounter more complex module structures.

// Overall, the provided code gives a glimpse into some fundamental concepts of Rust programming, including its strong type system, ownership model, and emphasis on safety and performance.