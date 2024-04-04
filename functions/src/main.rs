
// Associated Funtions and Method

struct Point {
    x: f64,
    y: f64,
}

// Implementation block, for `Point` type associated functions & methods
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    fn origin() -> Point {
        Point {x: 0.0, y: 0.0}
    }

    // new point function that take values of X and Y for new Point
    fn new (x: f64, y:f64) -> Point {
        Point { x: x, y: y}
    }

}

// another struct Rectangle with type
struct Rectangle {
    p1: Point,
    p2: Point
}

impl Rectangle {
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    fn area (&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        let Point {x: x1, y: y1} = self.p1;
        let Point {x: x2, y: y2} = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point {x: x1, y: y1} = self.p1;
        let Point {x: x2, y: y2} = self.p2;
        2.0 * ( (x1 - x2).abs() + (y1 - y2).abs())
    }
 // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    fn translate (&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;
        
        self.p1.y += y;
        self.p2.y += y;
    }
}

struct Pair (Box<i32>, Box<i32>);

impl Pair {
    fn destroy(self) {
        let Pair (first, second) = self;
        println!("Destroying Pair({}, {})", first, second);
    }
}

// Normal Functions
fn is_divisible_by(lhs: i32, rhs:i32) -> bool {
    if rhs == 0 {
        return false;
    }
    
    lhs % rhs == 0 // an expression so the return keyword is not needed
}

fn fizzbuzz (n: i32) -> () {
    if is_divisible_by(n, 15) {
        println!("FizzBuzz");
    } else if is_divisible_by(n, 3) {
        println!("Fizz");
    } else if is_divisible_by(n, 2) {
        println!("Buzz");
    } else {
        println!("{}", n)
    }
}

fn fizzbuzz_to(n: i32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}

fn main() {
    let demo_rec = Rectangle {
        p1: Point::origin(),
        p2: Point::new(5.0, 4.0),
    };
   // Methods are called using the dot operator
   // Note that the first argument `&self` is implicitly passed, i.e.
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", demo_rec.perimeter());
    println!("Rectangle area: {}", demo_rec.area()); 

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0)
    };

    square.translate(1.0, 1.5);
    
    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();

    fizzbuzz_to(20);

    // println!("Hello, world!");
}