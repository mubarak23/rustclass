
//Passing Generic value as type in function params

fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
} 

struct Point<T>{
    x: T,
    y: T
}

// Using Generic for struct type
struct Demo<T, U>{
    x: T,
    y: U
}

struct Val<T> {
    val: T,
}

//Add Generic to struct value method implementation
impl<T> Val<T> {
    fn value(&self) -> &T {
        &self.val
    }
}

struct APoint<T, U> {
    x: T,
    y: U,
}

impl<T, U> APoint<T, U> {
    // Implement mixup to make it work, DON'T modify other code.
    fn mixup<V,W>(self, other:APoint<V,W>) -> APoint<T,W> {
        APoint {
            x: self.x,
            y: other.y
        }
    }
}

fn main() {
    assert_eq!(5, sum(3i8,2i8));
    assert_eq!(60, sum(40i8, 20i8));
    assert_eq!(2.46, sum(1.23, 1.23));
    let integer: Point<i32> = Point {x: 5, y: 6};
    let float: Point<f64> = Point { x: 1.0, y:1.2};
    let demus: Demo<i32, String> = Demo {x: 4, y: "Hello Generic Struct".to_string()};
    let x: Val<f64> = Val{ val: 3.0 };
    let y: Val<String> = Val{ val: "hello".to_string()};
    let p1 = APoint { x: 5, y: 10 };
    let p2 = APoint { x: "Hello", y: '中'};

    let p3: APoint<i32, char> = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');
    println!("{}, {}", x.value(), y.value());
    println!("Hello, world!");
}
