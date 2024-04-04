
// 

// Rust provides several mechanisms to change or define the type of primitive and user defined types

type Nanosecond = u64;
type Inch = u64;
type U64 = u64;

fn main() {

    // type casting 
    // Rust provides no implicit type conversion (coercion) between primitive types.
    //  But, explicit type conversion (casting) can be performed using the as keyword.

    let decimal = 65.456;

    // Explicit Conversation
    let integer = decimal as u8;

    let character = integer as char;

     println!("Casting: {} -> {} -> {}", decimal, integer, character);

     // Type Literals
     // Numeric literals can be type annotated by adding the type as a suffix. As an example, 
     // to specify that the literal 42 should have the type i32, write 42i32

     // Suffixed literals, their types are known at initialization
     let x = 1u8;
     let y = 2u32;
     let z = 3f32;

     // Unsuffixed literals, their types depend on how they are used
     let i = 1;
     let f = 1.0;

     // `size_of_val` returns the size of a variable in bytes
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));

    // interference


    // Aliases
    // The type statement can be used to give a new name to an existing type. Types must have UpperCamelCase names, 
    // or the compiler will raise a warning. The exception to this rule are the primitive types: usize, f32, etc.

    let nanosecond : Nanosecond = 5 as U64;
    let inch : Inch = 2 as U64;


    println!("{} nanoseconds + {} inches = {} unit?", nanosecond, inch, nanosecond + inch);

    println!("Hello, world!");
}
