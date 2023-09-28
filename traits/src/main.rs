#[warn(unused_variables)]
trait Animal {
    fn sound (&self) -> String;
}

// Trait Animal has one methods, sounds(), in trait we define only the signature

struct Sheep;
struct Cow;

impl Animal for Sheep {
    fn sound(&self) -> String {
        String::from("Maah")
    }
}

impl Animal for Cow {
    fn sound(&self) -> String {
        String::from("Mooh")
    } 
}


trait Hello {
    fn say_hi(&self) -> String {
        String::from("hi")
    }

    fn say_something(&self) -> String;
}

struct Student {}
impl Hello for Student {
    fn say_something(&self) -> String{
        String::from("I'm a good student")
    }
}

struct Teacher {}
impl Hello for Teacher {
    fn say_hi(&self) -> String {
        String::from("Hi, I'm your new teacher")
    }
    
    fn say_something(&self) -> String {
        String::from("I'm not a bad teacher")
    }
}

trait NonLiving {
    fn noise(&self);
}

struct Dog;
struct Pig;

impl NonLiving for Dog {
    fn noise (&self){
        println!("Woff")
    }
}
impl NonLiving for Pig {
    fn noise (&self){
        println!("Meow")
    }
}

// dynamic trait object
// fn return_animal(s: &str) -> &dyn NonLiving {
//     match s {
//         "dog" => &Dog {},
//         "pig" => &Pig {},
//         - => painic!(),
//     }
// }

fn randon_number(num: u16) -> Box<dyn NonLiving>{
    if num >10 {
        Box::new(Pig {})
    } else {
        Box::new(Dog {})
    }
}

fn main() {
    let s = Student {};
    assert_eq!(s.say_hi(), "hi");
    assert_eq!(s.say_something(), "I'm a good student");
    let t = Teacher {};
    assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
    assert_eq!(t.say_something(), "I'm not a bad teacher");
    // let animal1 = return_animal("pig");
    // let animal2 = return_animal("dog");

    let animal_noise = 6;
    let the_noise = randon_number(animal_noise);
    the_noise.noise();
}
