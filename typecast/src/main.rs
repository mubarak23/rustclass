// FIX the errors and FILL in the blank
// DON'T remove any code
fn main() {
    let decimal: f32 = 97.123_f32;

    let integer: u8 = decimal as u8; // 97 as a

    let c1: char = decimal as u8 as char;
    let c2: char = integer as char;

    assert_eq!(integer +1, 'b' as u8); // 

    println!("Success!");
}


// FROM / INTO CONVERSATION

// From is now included in `std::prelude`, so there is no need to introduce it into the current scope
// use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    // IMPLEMENT `from` method
    fn from(value: i32) -> Self {
        Self {
            value: value // or value
        }
    }
}

// FILL in the blanks
fn main() {
    let num: Number = Number::from(30);
    assert_eq!(num.value, 30);

    let num: Number = 30.into();
    assert_eq!(num.value, 30);

    println!("Success!");
}

// When performing error handling it is often useful to implement From trait for our own error type. 
// Then we can use ? to automatically convert the underlying error type to our own error type.


use std::fs;
use std::io;
use std::num;

enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {
    // IMPLEMENT from method
    fn from (e: io::Error) -> Self {
        CliError::IoError(e)
    }
}

impl From<num::ParseIntError> for CliError {
    // IMPLEMENT from method
    fn from (e: num::ParseIntError) -> Self {
        CliError::ParseError(e)
    }
}

fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    // ? automatically converts io::Error to CliError
    let contents: String = fs::read_to_string(&file_name)?;
    // num::ParseIntError -> CliError
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}

fn main() {
    println!("Success!");
}


#[derive(Debug, PartialEq)]
struct EvenNum(i32);

impl TryFrom<i32> for EvenNum {
    type Error = ();

    // IMPLEMENT `try_from`
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNum(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
    assert_eq!(EvenNum::try_from(5), Err(()));

    // FILL in the blanks
    let result: Result<EvenNum, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNum(8)));
    let result: Result<EvenNum, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    println!("Success!");
}


use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    // IMPLEMENT fmt method
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The point is ({}, {})", self.x, self.x)   
     }
}

fn main() {
    let origin: Point = Point { x: 0, y: 0 };
    // FILL in the blanks
    assert_eq!(origin.to_string(), "The point is (0, 0)");
    assert_eq!(format!("{}", origin), "The point is (0, 0)");
    print!("{}", origin);
    println!("Success!");
}


// panic 

// FILL the blanks
fn drink(beverage: &str) {
    if beverage == "lemonade" {
        println!("Success!");
        // IMPLEMENT the below code
        panic!()
     }

    println!("Exercise Failed if printing out this line!");
}

fn main() {
    drink("lemonade");

    println!("Exercise Failed if printing out this line!");
}
