
// Here we define trait that has an associated trait
// we have a function that return value of the type
trait MyTrait {
    type MyType;

    fn get_my_type(&self) -> Self::MyType;
}

struct MyStruct {}

// when implementing the trait for a specific type (MyStruct), then we have to give the associated type MyType, 
// a concreate type in this case i32 
impl MyTrait for MyStruct {
    type MyType = i32;

    fn get_my_type(&self) -> Self::MyType {
            return 42
    }
}


trait Bird {
    fn quack(&self) -> String;
}

struct Duck;
impl Duck {
    fn swim(&self) {
        println!("Look, the duck is swimming")
    }
}
struct Swan;
impl Swan {
    fn fly(&self) {
        println!("Look, the duck.. oh sorry, the swan is flying")
    }
}

impl Bird for Duck {
    fn quack(&self) -> String{
        "duck duck".to_string()
    }
}

impl Bird for Swan {
    fn quack(&self) -> String{
        "swan swan".to_string()
    }
}



// FILL in the blanks.
trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", self)
    }
}


// dynamic and static dispatch


trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String { format!("u8: {}", *self) }
}

impl Foo for String {
    fn method(&self) -> String { format!("string: {}", *self) }
}

// IMPLEMENT below with generics.
fn static_dispatch<T: Foo>(a: T){
    a.method();
}


// Implement below with trait objects.
fn dynamic_dispatch(a: &dyn Foo){
    a.method();
}

// fn main() {
//     let x: u8 = 5u8;
//     let y: String = "Hello".to_string();

//     static_dispatch(x);
//     dynamic_dispatch(&y);

//     println!("Success!");
// }




fn main() {
    // FILL in the blank.
    let duck: Duck = Duck;
    duck.swim();

    let bird: Box<dyn Bird> = hatch_a_bird(2);
    // This bird has forgotten how to swim, so below line will cause an error.
    // bird.swim();
    // But it can quak.
    assert_eq!(bird.quack(), "duck duck");

    let bird: Box<dyn Bird> = hatch_a_bird(1);
    // This bird has forgotten how to fly, so below line will cause an error.
    // bird.fly();
    // But it can quak too.
    assert_eq!(bird.quack(), "swan swan");

    println!("Success!");

      let x: f64 = 1.1f64;
    let y: u8 = 8u8;

    // Draw x.
    draw_with_box(Box::new(x));

    // Draw y.
    draw_with_ref(&y);

    println!("Success!");

}   

// IMPLEMENT this function.
fn hatch_a_bird(species: u8) -> Box<dyn Bird> {
    match species {
        1 => Box::new(Swan),
        2 => Box::new(Duck),
        _ => panic!(),
    }
}

// &dyn and Box<dyn>

fn draw_with_box(x: Box<dyn Draw>) {
    x.draw();
}

fn draw_with_ref(x: &dyn Draw) {
    x.draw();
}

