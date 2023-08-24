fn main() {
    println!("Hello World");
    another_function(10);
    let x = plus_one(5);
    println!("The value of x is: {x}")
   
}

// fn five() -> i32 {
//     5
// }

fn plus_one(x: i32) -> i32 {
    x + 1 // this is an expression and it return a value, if we add ; we turn it into a statment which does not return a value
}

fn another_function(x: i32){
    // println!("Another Beatiful Function");
    println!("The value of x is: {x}");
}

 // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");
    // let x = 5;

    // let x = x + 1;

    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}")
    // }
    // println!("The value of x in the outter scope is: {x}")
    // let mut x: i32 = 6;
    // print!("{x}");
    // while x != 1 {
    //     if x %2 == 0 {
    //         x = x / 2
    //     } else {
    //         x = 3 * x + 1;
    //     }
    //    print!(" -> {x}"); 
    // } 
    // println!();


//  Variables
// variables are immutable by default
//  you can make them mutable by adding mut in front of the variable name 
//  Ultimately, deciding whether to use mutability or not is up to you and depends on what you think is clearest in that particular situation.

// Constant
// constants are values that are bound to a name and are not allowed to change,
// you aren’t allowed to use mut with constants. Constants aren’t just immutable by default—they’re always immutable
// we declare constant using const keyword instead of let
// the type of the value must be annotated
//  Rust’s naming convention for constants is to use all uppercase with underscores between words

// Shadowing
//  We can shadow a variable by using the same variable’s name and repeating the use of the let keyword
// Shadowing is different from marking a variable as mut because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword
// other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let 
// we can change the type of the value but reuse the same name.

// Data Types
// Every value in Rust is of a certain data type, which tells Rust what kind of data is being specified so it knows how to work with that data
// Rust is a statically typed language, which means that it must know the types of all variables at compile time
// two data type subsets in rust: scalar and compound
// Scalar represent a single value, Rust has four primaryscalar types\
//  boolean, floating point number, integers, and characters.
    // Integers
    // number without a fractional component
    // Each variant can be either signed or unsigned and has an explicit size
    // Signed and unsigned refer to whether it’s possible for the number to be negative—in other words, whether the number needs to have a sign with it (signed) 
    // or whether it will only ever be positive and can therefore be represented without a sign (unsigned). 

// Compound Types
  // Tuple 
    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    // We create a tuple by writing a comma-separated list of values inside parentheses.
    // 
 // Array
  // a way to collect mutiple vaules
  // very element of an array must have the same type
  // arrays in Rust have a fixed length.
  // Arrays are useful when you want your data allocated on the stack rather than the heap
  // An array is a single chunk of memory of a known, fixed size that can be allocated on the stack.   

// Functions
    // Rust code uses snake case as the conventional style for function and variable names
    // We define a function in Rust by entering fn followed by a function name and a set of parentheses. 
    // The curly brackets tell the compiler where the function body begins and ends
    //  Rust doesn’t care where you define your functions, 
    // only that they’re defined somewhere in a scope that can be seen by the caller.
    // We can define functions to have parameters, which are special variables that are part of a function’s signature.
    // In function signatures, you must declare the type of each parameter. 

// Statement and Expressions
    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resultant value
    // Creating a variable and assigning a value to it with the let keyword is a statement.
    // Function defination are also statment, 
    // statment do not return a value, Therefore, you can’t assign a let statement to another variable
    // Expressions evaluate to a value and make up most of the rest of the code that you’ll write in Rust. 
    // Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, 
    //  you turn it into a statement, and it will then not return a value. Keep this in mind as you explore function return values and expressions next
// Functions with Return Values
    // Function can return value to the code that call them
    //  In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function
    // 
