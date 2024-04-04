
use std::convert::From;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;
// use core::fmt::Error;


#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

struct Number {
    value: i32
}

// Conversion to String
// To convert any type to a String is as simple as implementing the ToString trait for the type.
//  Rather than doing so directly, you should implement the fmt::Display trait which automagically provides ToString and also allows printing the type

struct Circle {
    radius: i32 
}

impl fmt::Display for Circle {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

// impl From<i32> for Number {
//     fn from(item: i32) -> Self {
//         Number { value: item}
//     }
// }

// impl Into<Number> for i32 {
//     fn into(self) -> Number {
//         Number { value: self}
//     }
// }

// From 
// The From trait allows for a type to define how to create itself from another type, 
// hence providing a very simple mechanism for converting between several types.

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item}
    }
}

// Into
// The Into trait is simply the reciprocal of the From trait. That is, if you have implemented the From trait
//  for your type, Into will call it when necessary
// impl Into<Number> for i32 {
//     fn into (self) -> Number {
//         Number { value: self}
//     }
// }
// impl Into<Number> for i32 {
//     fn into(self) -> Number {
//         Number { value: self }
//     }
// }

// TryFrom and TryTo
// Similar to From and Into, TryFrom and TryInto are generic traits for converting between types. Unlike From/Into,
// the TryFrom/TryInto traits are used for fallible conversions, and as such, return Results.



fn main() {
     let num = Number::from(6);
     let int = 5;
    //  let my_into_number = int.into();

    //  println!("My Into Number is {:?}", my_into_number);

    // let my_into: Number = int.into();
    // println!("My Into Number is {:?}", my_into);
   //  println!("My From Number is {:?}", num);

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    let circle = Circle { radius: 8};
    println!("{}", circle.to_string());

}
