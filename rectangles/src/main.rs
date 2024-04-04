
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//  Everything within this impl block will be associated with the Rectangle type. 
impl Rectangle {

    // In the signature for area, we use &self instead of rectangle: &Rectangle. 
    // The &self is actually short for self: &Self

    //  Within an impl block, the type Self is an alias for the type that the impl block is for

    fn area (&self) -> u32 {
        self.width * self.height
    }

    fn width (&self) -> bool {
        self.width > 0
    }

    fn can_hold (&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn area (width: u32, height: u32) -> u32 {
    width * height
}

// using tuple struct
fn area_tuple_struct (dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// using struct as fn argument
fn area_struct (rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}


fn main() {
    let width1 = 30;
    let height1 = 50;

   println!("The Area of a rectangle is {} pixel", area(width1, height1));

   // using tuple struct
    let rect = (50, 40);
   println!("The Area of a rectangle via tuple struct is {} pixel", area_tuple_struct(rect));

   // using struct
   let rect1 = Rectangle {
    width: 20,
    height: 30
   };
   println!("rect struct is {:?} ", rect1);
    println!("The Area of a rectangle via struct is {} pixel", area_struct(&rect1));

    // using struct method
    println!("The Area of a rectangle via struct method is {} pixel", rect1.area());

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }


    // another method on Rectangle Struct 

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 10,
        height: 15 ,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));


}
