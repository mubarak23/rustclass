
use std::convert::From;
#[derive(Debug)]

struct Number {
    value: i32
}

// impl From<i32> for Number {
//     fn from(item: i32) -> Self {
//         Number { value: item}
//     }
// }

impl Into<Number> for i32 {
    fn into(self) -> Number {
        Number { value: self}
    }
}



fn main() {
   //  let num = Number::from(6);
    let int = 5;
    let my_into: Number = int.into();
    println!("My Into Number is {:?}", my_into);
   // println!("My From Number is {:?}", num);
}
