#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle{
        Rectangle {
            width,
            height
        }
    }
}


struct TrafficLight {
    color: String,
}

impl TrafficLight {
    
    pub fn new() -> Self {
        Self {
            color: String::from("red"),
        }
    }

    pub fn get_state(&self) -> &str {
        &self.color
    }
}

enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

// Implement TrafficLightColor with a method.
impl TrafficLightColor {
    fn color (&self) -> &str {
        match self {
            TrafficLightColor::Yellow => "yellow",
            TrafficLightColor::Red => "red",
            TrafficLightColor::Green => "green"
        }
    }
}

fn main() {
      let light = TrafficLight::new();
    assert_eq!(light.get_state(), "red");
    let rect1 : Rectangle = Rectangle::new(6,4);
    let c = TrafficLightColor::Red;
    assert_eq!(c.color(), "red");
    println!("Rectangle: {:?}", rect1);
}
