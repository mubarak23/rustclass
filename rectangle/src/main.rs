
struct Reactangle {
    width: u32,
    height: u32,
}

impl Reactangle {
    fn width(&self) -> bool {
        self.width > 0
    }
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Reactangle) -> bool{
        self.width > other.width && self.height > other.height
    }

    fn square (size: u32) -> Self {
        Self {
            height: size,
            width: size,
        }
    }
// The Self keywords in the return type and in the body of the function are aliases for the type that appears 
// after the impl keyword, which in this case is Rectangle.

}

fn main() {
    // let width1 = 6;
    // let height1 = 4;
    let rect1 = Reactangle {
        width: 40,
        height: 40,
    };
    let react2 = Reactangle {
        width: 4,
        height: 6,
    };
    let react3 = Reactangle {
        width: 12,
        height: 6,
    };

    let sqr = Reactangle::square(4);
    println!("Can react1 hold react2? {}", rect1.can_hold(&react2));
    println!("Can react2 hold react3? {}", react2.can_hold(&react3));
   //  println!("{}", sqr);

    // println!("The Area of a Reactangle is {} square pixels", rect1.area());
    // if rect1.width() {
    //     println!("The Rectangle has a non zero width, it is {}", rect1.width);
    // }
}

// Here, we’re choosing to make the width method return true if the value in the instance’s width field is greater
//  than 0 and false if the value is 0: we can use a field within a method of the same name for any purpose.
//   In main, when we follow rect1.width with parentheses, Rust knows we mean the method width. 
//   When we don’t use parentheses, Rust knows we mean the field width.


// fn area(width1: u32, height1: u32) -> u32 {
//     width1 * height1
// }

// fn area(diamention: (u32, u32)) -> u32 {
//     diamention.0 * diamention.1
// }

fn area(rectangle: &Reactangle) -> u32 {
    rectangle.width * rectangle.height
}