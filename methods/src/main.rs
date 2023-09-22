
#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32
}
impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }
}

impl Rectangle {
     fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        height: 4,
        width: 4
    };
    println!("The Area of a Rectangle is {} square pixel", rect1.area());
}
